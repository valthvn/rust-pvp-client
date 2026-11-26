//! "Play the PvP client": the native client (Horizon + pvp-core), started for the default account as Horizon's
//! Launch API asks (`pvp_launch`). Its files live in `<launcher data>/pvp-client/`, installed and updated from
//! the latest published GitHub release; `PVP_CLIENT_BIN` may instead point at a folder holding a build of it
//! (a `build.yml` artifact), which is then used as it is.

use crate::hooks::Actions;
use crate::launcher;
use oneclient_core::LauncherState;
use oneclient_events::{GroupedProgressChild, GroupedProgressSession, TaskCategory};
use pvp_launch::{Layout, Session};
use std::path::{Path, PathBuf};
use std::process::Stdio;

/// The player's own Minecraft folder, where the official launcher keeps 26.3.
fn minecraft_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        std::env::var_os("APPDATA").map(|a| PathBuf::from(a).join(".minecraft"))
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME")
            .map(|h| PathBuf::from(h).join("Library/Application Support/minecraft"))
    } else {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".minecraft"))
    }
}

/// The client's game folder, where its module profiles live.
pub fn instance_dir() -> Option<PathBuf> {
    layout().map(|(l, _)| l.instance)
}

/// The client's folders, and whether its files are a developer's build (`PVP_CLIENT_BIN`) never updated.
fn layout() -> Option<(Layout, bool)> {
    let root = oneclient_common::paths::data_dir().ok()?.join("pvp-client");
    let mut layout = Layout::under(&root);
    let dev = std::env::var_os("PVP_CLIENT_BIN").map(PathBuf::from);
    if let Some(bin) = &dev {
        layout.bin = bin.clone();
    }
    if let Some(prepared) = std::env::var_os("PVP_CLIENT_PREPARED") {
        layout.prepared = PathBuf::from(prepared);
    }
    Some((layout, dev.is_some()))
}

/// Account-free developer play: only in a development build of the launcher (never a release), only with an
/// offline account, and only on resources already prepared on this machine (nothing is downloaded). The
/// client gets no session, so it plays single player only, as Horizon's own developer mode does.
fn run_developer_offline() -> Result<(), String> {
    let (layout, _) = layout().ok_or("no launcher data folder")?;
    layout.check_installed().map_err(|e| e.to_string())?;
    let prepared = layout
        .check_command()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    if !prepared {
        return Err(format!(
            "offline developer play needs resources prepared already ({} is not); set PVP_CLIENT_PREPARED",
            layout.prepared.display()
        ));
    }
    let mut child = layout.launch_offline().map_err(|e| e.to_string())?;
    tracing::warn!(target: "pvp_client", "started the PvP client offline (developer build, single player only)");
    std::thread::spawn(move || {
        let _ = child.wait();
    });
    Ok(())
}

async fn get(state: &LauncherState, url: &str) -> Result<Vec<u8>, String> {
    let res = state
        .services
        .requester
        .http()
        .get(url)
        .header("User-Agent", "pvp-client-launcher")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("{url} answered {}", res.status()));
    }
    Ok(res.bytes().await.map_err(|e| e.to_string())?.to_vec())
}

/// Installs or updates the client from the latest published release. Offline, an installed client is kept.
async fn ensure_client(state: &LauncherState, layout: &Layout) -> Result<(), String> {
    let installed = pvp_launch::installed_tag(&layout.bin);
    let asset = pvp_launch::asset_name().ok_or("no PvP client build for this platform")?;
    let release = match get(state, pvp_launch::LATEST_RELEASE).await {
        Ok(json) => pvp_launch::release_from_json(&String::from_utf8_lossy(&json), asset),
        Err(err) => {
            tracing::warn!(target: "pvp_client", "checking for a release: {err}");
            None
        }
    };
    let Some(release) = release else {
        return match installed {
            Some(_) => Ok(()),
            None => Err("no release of the PvP client is published yet".into()),
        };
    };
    if installed.as_deref() == Some(release.tag.as_str()) {
        return Ok(());
    }
    tracing::info!(target: "pvp_client", tag = %release.tag, "installing the PvP client");
    let sums = String::from_utf8_lossy(&get(state, &release.sums_url).await?).into_owned();
    let expected = pvp_launch::sha256_for(&sums, asset)
        .ok_or("the release has no checksum for this platform")?;
    let archive = get(state, &release.archive_url).await?;
    let mut hash = polyio::ChecksumStream::new(polyio::ChecksumAlgorithm::Sha256);
    hash.update(&archive);
    if !hash.finish().eq_ignore_ascii_case(expected) {
        return Err("the downloaded PvP client does not match its checksum".into());
    }
    // Unpacked beside the old one, then swapped, so a failed update keeps a working client.
    let fresh = layout.bin.with_extension("new");
    let _ = std::fs::remove_dir_all(&fresh);
    polyio::unzip_bytes(archive, &fresh)
        .await
        .map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for exe in ["pvp-client", "horizon-prepare", "horizon-ctl"] {
            let _ =
                std::fs::set_permissions(fresh.join(exe), std::fs::Permissions::from_mode(0o755));
        }
    }
    std::fs::write(fresh.join(pvp_launch::INSTALLED_TAG), &release.tag)
        .map_err(|e| e.to_string())?;
    let _ = std::fs::remove_dir_all(&layout.bin);
    std::fs::rename(&fresh, &layout.bin).map_err(|e| e.to_string())
}

/// Downloads the server jar `horizon-prepare` needs once, checked against the SHA-1 Mojang gives.
async fn ensure_server_jar(
    state: &LauncherState,
    layout: &Layout,
    minecraft: &Path,
) -> Result<(), String> {
    let jar = layout.server_jar();
    let source = pvp_launch::server_jar_source(minecraft).map_err(|e| e.to_string())?;
    if jar.is_file()
        && polyio::sha1_bytes(&std::fs::read(&jar).map_err(|e| e.to_string())?) == source.sha1
    {
        return Ok(());
    }
    let bytes = get(state, &source.url).await?;
    if polyio::sha1_bytes(&bytes) != source.sha1 {
        return Err("the server jar Mojang sent does not match its checksum".into());
    }
    std::fs::create_dir_all(&layout.server_jars).map_err(|e| e.to_string())?;
    std::fs::write(&jar, bytes).map_err(|e| e.to_string())
}

/// Runs the preparer, its `fraction` lines moving the progress bar.
fn prepare(
    layout: &Layout,
    minecraft: &Path,
    progress: &GroupedProgressChild,
) -> Result<(), String> {
    use std::io::{BufRead, Read};
    let mut child = layout
        .prepare_command(minecraft, None)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", layout.preparer().display()))?;
    if let Some(out) = child.stdout.take() {
        for line in std::io::BufReader::new(out).lines().map_while(Result::ok) {
            if let Some(f) = pvp_launch::progress_fraction(&line) {
                progress.set_progress((f * 1000.0) as u64, Some(1000));
            } else if let Some(msg) = pvp_launch::progress_message(&line) {
                tracing::info!(target: "pvp_client", "prepare: {msg}");
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        return Ok(());
    }
    let mut err = String::new();
    if let Some(mut e) = child.stderr.take() {
        let _ = e.read_to_string(&mut err);
    }
    Err(format!(
        "preparing the game's resources failed: {}",
        err.lines().last().unwrap_or("unknown error")
    ))
}

async fn run() -> Result<(), String> {
    let state = launcher::state().map_err(|e| format!("{e:#}"))?;
    // The owner gate: a signed-in Microsoft account, refreshed for this launch.
    let account = state
        .auth
        .default_account_for_launch()
        .await
        .map_err(|e| format!("{e:#}"))?
        .ok_or_else(|| pvp_launch::Error::NotSignedIn.to_string())?;
    if oneclient_auth::dev_offline_allowed() && account.kind == oneclient_auth::AccountKind::Offline
    {
        return run_developer_offline();
    }
    let session = Session::for_account(&account).map_err(|e| e.to_string())?;
    let (layout, dev) = layout().ok_or("no launcher data folder")?;
    if !dev {
        ensure_client(&state, &layout).await?;
    }
    layout.check_installed().map_err(|e| e.to_string())?;
    let minecraft = minecraft_dir().ok_or("no Minecraft folder")?;

    let prepared = layout
        .check_command()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    if !prepared {
        let session_bar =
            GroupedProgressSession::start(&state.services.events, "Preparing the PvP client");
        let result = async {
            ensure_server_jar(&state, &layout, &minecraft).await?;
            let child = session_bar.child("Game resources", 1000, TaskCategory::Assets);
            let (l, m, c) = (layout.clone(), minecraft.clone(), child.clone());
            tokio::task::spawn_blocking(move || prepare(&l, &m, &c))
                .await
                .map_err(|e| e.to_string())??;
            child.finish();
            Ok::<(), String>(())
        }
        .await;
        match result {
            Ok(()) => session_bar.finish(),
            Err(err) => {
                session_bar.abort();
                return Err(err);
            }
        }
    }

    let mut child = layout.launch(&session).map_err(|e| e.to_string())?;
    std::thread::spawn(move || match child.wait() {
        Ok(status) => tracing::info!(target: "pvp_client", %status, "the PvP client exited"),
        Err(e) => tracing::warn!(target: "pvp_client", "waiting for the PvP client: {e}"),
    });
    Ok(())
}

/// Starts the PvP client, telling the player what went wrong when it can't.
pub async fn launch(actions: &Actions) {
    if let Err(err) = run().await {
        tracing::warn!(target: "pvp_client", "launch failed: {err}");
        actions
            .notify("Could not start the PvP client")
            .body(err)
            .error()
            .send();
    }
}
