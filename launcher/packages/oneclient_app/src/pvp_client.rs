//! "Play the PvP client": the native client (Horizon + pvp-core), started for the default account as Horizon's
//! Launch API asks (`pvp_launch`). Its files live in `<launcher data>/pvp-client/`; while there is no release
//! to download, `PVP_CLIENT_BIN` may point at a folder holding a build of it (a `build.yml` artifact).

use crate::hooks::Actions;
use crate::launcher;
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

fn layout() -> Option<Layout> {
    let root = oneclient_common::paths::data_dir().ok()?.join("pvp-client");
    let mut layout = Layout::under(&root);
    if let Some(bin) = std::env::var_os("PVP_CLIENT_BIN") {
        layout.bin = PathBuf::from(bin);
    }
    Some(layout)
}

/// Downloads the server jar `horizon-prepare` needs once, checked against the SHA-1 Mojang gives.
async fn ensure_server_jar(
    http: &impl ServerJarFetch,
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
    let bytes = http.fetch(&source.url).await?;
    if polyio::sha1_bytes(&bytes) != source.sha1 {
        return Err("the server jar Mojang sent does not match its checksum".into());
    }
    std::fs::create_dir_all(&layout.server_jars).map_err(|e| e.to_string())?;
    std::fs::write(&jar, bytes).map_err(|e| e.to_string())
}

trait ServerJarFetch {
    async fn fetch(&self, url: &str) -> Result<Vec<u8>, String>;
}

struct Http<'a>(&'a oneclient_core::LauncherState);

impl ServerJarFetch for Http<'_> {
    async fn fetch(&self, url: &str) -> Result<Vec<u8>, String> {
        let res = self
            .0
            .services
            .requester
            .http()
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Mojang answered {}", res.status()));
        }
        Ok(res.bytes().await.map_err(|e| e.to_string())?.to_vec())
    }
}

/// Runs the preparer, notifying its progress messages.
fn prepare(layout: &Layout, minecraft: &Path) -> Result<(), String> {
    use std::io::BufRead;
    let mut child = layout
        .prepare_command(minecraft, None)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", layout.preparer().display()))?;
    if let Some(out) = child.stdout.take() {
        for line in std::io::BufReader::new(out).lines().map_while(Result::ok) {
            if let Some(msg) = pvp_launch::progress_message(&line) {
                tracing::info!(target: "pvp_client", "prepare: {msg}");
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        let mut err = String::new();
        if let Some(mut e) = child.stderr.take() {
            use std::io::Read;
            let _ = e.read_to_string(&mut err);
        }
        return Err(format!(
            "preparing the game's resources failed: {}",
            err.lines().last().unwrap_or("unknown error")
        ));
    }
    Ok(())
}

async fn run(actions: &Actions) -> Result<(), String> {
    let state = launcher::state().map_err(|e| format!("{e:#}"))?;
    // The owner gate: a signed-in Microsoft account, refreshed for this launch.
    let account = state
        .auth
        .default_account_for_launch()
        .await
        .map_err(|e| format!("{e:#}"))?
        .ok_or_else(|| pvp_launch::Error::NotSignedIn.to_string())?;
    let session = Session::for_account(&account).map_err(|e| e.to_string())?;
    let layout = layout().ok_or("no launcher data folder")?;
    layout.check_installed().map_err(|e| e.to_string())?;
    let minecraft = minecraft_dir().ok_or("no Minecraft folder")?;

    let prepared = layout
        .check_command()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    if !prepared {
        actions
            .notify("Preparing the PvP client")
            .body("Building the game's resources from your Minecraft 26.3 (a few minutes, once).")
            .send();
        ensure_server_jar(&Http(&state), &layout, &minecraft).await?;
        let (l, m) = (layout.clone(), minecraft.clone());
        tokio::task::spawn_blocking(move || prepare(&l, &m))
            .await
            .map_err(|e| e.to_string())??;
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
    if let Err(err) = run(actions).await {
        tracing::warn!(target: "pvp_client", "launch failed: {err}");
        actions
            .notify("Could not start the PvP client")
            .body(err)
            .error()
            .send();
    }
}
