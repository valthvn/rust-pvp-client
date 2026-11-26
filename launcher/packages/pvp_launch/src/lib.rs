//! Starts the native PvP client from the launcher, as Horizon's Launch API asks (`engine/docs/api/launch.md`):
//!
//! 1. only for a signed-in Microsoft account (the owner gate): [`Session::for_account`] refuses anything else;
//! 2. prepare the game's resources from the player's own Minecraft with `horizon-prepare`
//!    ([`Layout::check_command`], [`Layout::prepare_command`]); the server jar it needs once is downloaded by the
//!    launcher from Mojang ([`server_jar_source`]) for that same account;
//! 3. start `pvp-client` with the session written to its standard input, never on the command line
//!    ([`Layout::launch`]).
//!
//! The client's files (`pvp-client`, `horizon-prepare`, `research-lock.json`, the exporter sources and the
//! built-in mods' manifests) live in [`Layout::bin`], as a release of the client ships them.

pub mod profiles;

use oneclient_auth::{AccountKind, MinecraftAccount};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// The Minecraft version this client is built for (`horizon-prepare --launch-api`'s `minecraft`).
pub const MINECRAFT: &str = "26.3";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sign in with a Microsoft account that owns Minecraft: Java Edition to play")]
    NotSignedIn,
    #[error("the PvP client is not installed ({0} is missing)")]
    NotInstalled(PathBuf),
    #[error(
        "Minecraft {MINECRAFT} is not installed in {0}: start it once from the official launcher"
    )]
    NoMinecraft(PathBuf),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

/// The account the game joins online-mode servers with: the Launch API's session JSON.
#[derive(Clone, PartialEq, Eq)]
pub struct Session {
    pub name: String,
    /// Without dashes.
    pub uuid: String,
    pub access_token: String,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("name", &self.name)
            .field("uuid", &self.uuid)
            .finish_non_exhaustive()
    }
}

impl Session {
    /// The session of `account`, which must be a Microsoft account still signed in (an offline account
    /// never starts the game: Horizon's owner gate).
    pub fn for_account(account: &MinecraftAccount) -> Result<Self, Error> {
        if account.kind != AccountKind::Microsoft
            || account.signed_out
            || account.access_token.is_empty()
        {
            return Err(Error::NotSignedIn);
        }
        Ok(Self {
            name: account.username.clone(),
            uuid: account.id.simple().to_string(),
            access_token: account.access_token.clone(),
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::json!({
            "name": self.name,
            "uuid": self.uuid,
            "access_token": self.access_token,
            "user_type": "msa",
        })
        .to_string()
    }
}

/// Where the client's files go, under one root in the launcher's data folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    /// The client's release: executables, `research-lock.json`, exporter sources, mod manifests.
    pub bin: PathBuf,
    /// The resources `horizon-prepare` writes and `pvp-client --resources` reads.
    pub prepared: PathBuf,
    /// The game folder (worlds, options, the client's profiles in `pvp-client/`).
    pub instance: PathBuf,
    /// The server jars downloaded for preparing.
    pub server_jars: PathBuf,
}

fn exe(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
}

impl Layout {
    pub fn under(root: &Path) -> Self {
        Self {
            bin: root.join("bin"),
            prepared: root.join("prepared"),
            instance: root.join("instance"),
            server_jars: root.join("server-jars"),
        }
    }

    pub fn client(&self) -> PathBuf {
        exe(&self.bin, "pvp-client")
    }

    pub fn preparer(&self) -> PathBuf {
        exe(&self.bin, "horizon-prepare")
    }

    pub fn server_jar(&self) -> PathBuf {
        self.server_jars.join(format!("{MINECRAFT}.jar"))
    }

    /// The files a release must have put in [`bin`](Self::bin).
    pub fn check_installed(&self) -> Result<(), Error> {
        for path in [
            self.client(),
            self.preparer(),
            self.bin.join("research-lock.json"),
        ] {
            if !path.is_file() {
                return Err(Error::NotInstalled(path));
            }
        }
        Ok(())
    }

    /// `horizon-prepare --check`: exit 0 when the resources are complete and current.
    pub fn check_command(&self) -> Command {
        let mut cmd = Command::new(self.preparer());
        cmd.current_dir(&self.bin)
            .arg("--root")
            .arg(&self.bin)
            .arg("--check")
            .arg("--output")
            .arg(&self.prepared);
        cmd
    }

    /// `horizon-prepare --prepare --complete`, reading the player's Minecraft in `minecraft_dir`, with progress
    /// as JSON lines on stdout. `java_home` is a Java of the pinned version, when the launcher manages one.
    pub fn prepare_command(&self, minecraft_dir: &Path, java_home: Option<&Path>) -> Command {
        let mut cmd = Command::new(self.preparer());
        cmd.current_dir(&self.bin)
            .arg("--root")
            .arg(&self.bin)
            .arg("--prepare")
            .arg("--complete")
            .arg("--output")
            .arg(&self.prepared)
            .arg("--minecraft-dir")
            .arg(minecraft_dir)
            .arg("--server-jar")
            .arg(self.server_jar())
            .args(["--progress", "json"]);
        if let Some(java) = java_home {
            cmd.arg("--java-home").arg(java);
        }
        cmd
    }

    /// The client's command line, without the session (it goes to stdin).
    pub fn launch_command(&self) -> Command {
        let mut cmd = Command::new(self.client());
        cmd.current_dir(&self.instance)
            .arg("--resources")
            .arg(&self.prepared)
            .arg("--game-dir")
            .arg(&self.instance)
            .arg("--home")
            .arg(&self.instance)
            .arg("--session-stdin");
        cmd
    }

    /// Starts the client for `session`: its JSON is written to the game's stdin, which is then closed.
    pub fn launch(&self, session: &Session) -> Result<Child, Error> {
        self.check_installed()?;
        std::fs::create_dir_all(&self.instance)?;
        let mut child = self
            .launch_command()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(session.to_json().as_bytes())?;
        }
        tracing::info!(client = %self.client().display(), "started the PvP client");
        Ok(child)
    }
}

impl Layout {
    /// The client's command line for an account-free developer session: no session at all, so the game
    /// plays single player only (online-mode servers refuse it). For development builds of the launcher
    /// only (Horizon's own `--developer-single-player` rule); never offered in a release.
    pub fn offline_command(&self) -> Command {
        let mut cmd = Command::new(self.client());
        cmd.current_dir(&self.instance)
            .arg("--resources")
            .arg(&self.prepared)
            .arg("--game-dir")
            .arg(&self.instance)
            .arg("--home")
            .arg(&self.instance);
        cmd
    }

    /// Starts [`offline_command`](Self::offline_command).
    pub fn launch_offline(&self) -> Result<Child, Error> {
        self.check_installed()?;
        std::fs::create_dir_all(&self.instance)?;
        Ok(self
            .offline_command()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?)
    }
}

/// Where Mojang serves the server jar `horizon-prepare` needs once: the URL, SHA-1 and size the player's own
/// version JSON (`<minecraft>/versions/26.3/26.3.json`) gives. The launcher downloads it for a signed-in owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerJar {
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

pub fn server_jar_source(minecraft_dir: &Path) -> Result<ServerJar, Error> {
    let json = minecraft_dir
        .join("versions")
        .join(MINECRAFT)
        .join(format!("{MINECRAFT}.json"));
    if !json.is_file() {
        return Err(Error::NoMinecraft(minecraft_dir.to_owned()));
    }
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&json)?)?;
    let s = &v["downloads"]["server"];
    let field = |k: &str| s[k].as_str().map(str::to_owned);
    match (field("url"), field("sha1"), s["size"].as_u64()) {
        (Some(url), Some(sha1), Some(size)) => Ok(ServerJar { url, sha1, size }),
        _ => Err(Error::NoMinecraft(minecraft_dir.to_owned())),
    }
}

/// The latest published release of the client (drafts are never listed).
pub const LATEST_RELEASE: &str =
    "https://api.github.com/repos/valthvn/rust-pvp-client/releases/latest";
/// The file in [`Layout::bin`] naming the release installed there.
pub const INSTALLED_TAG: &str = ".release-tag";

/// This platform's client archive, as `release.yml` names it (`client-<platform>.zip`).
pub fn asset_name() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => Some("client-windows-x64.zip"),
        ("linux", "x86_64") => Some("client-linux-x64.zip"),
        ("macos", "aarch64") => Some("client-macos-arm64.zip"),
        _ => None,
    }
}

/// A release's archive for this platform and its checksum list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub tag: String,
    pub archive_url: String,
    pub sums_url: String,
}

/// Reads GitHub's release JSON: the archive named `asset` and `SHA256SUMS`, both required.
pub fn release_from_json(json: &str, asset: &str) -> Option<Release> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let url = |name: &str| {
        v["assets"].as_array()?.iter().find_map(|a| {
            (a["name"].as_str() == Some(name))
                .then(|| a["browser_download_url"].as_str().map(str::to_owned))
                .flatten()
        })
    };
    Some(Release {
        tag: v["tag_name"].as_str()?.to_owned(),
        archive_url: url(asset)?,
        sums_url: url("SHA256SUMS")?,
    })
}

/// The SHA-256 `sha256sum` wrote for `file` in `sums`.
pub fn sha256_for<'a>(sums: &'a str, file: &str) -> Option<&'a str> {
    sums.lines().find_map(|l| {
        let (hash, name) = l.split_once(char::is_whitespace)?;
        (name.trim_start_matches([' ', '*']) == file).then_some(hash)
    })
}

/// The release installed in `bin`, if any.
pub fn installed_tag(bin: &Path) -> Option<String> {
    std::fs::read_to_string(bin.join(INSTALLED_TAG))
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
}

/// How far `horizon-prepare --progress json` is, from its `{"fraction": …}` lines.
pub fn progress_fraction(line: &str) -> Option<f64> {
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    v["fraction"].as_f64().map(|f| f.clamp(0.0, 1.0))
}

/// One line of `horizon-prepare --progress json`: what to show the player, if anything.
pub fn progress_message(line: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    v["message"]
        .as_str()
        .or_else(|| v["error"].as_str())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(kind: AccountKind, signed_out: bool) -> MinecraftAccount {
        serde_json::from_value(serde_json::json!({
            "id": "069a79f4-44e9-4726-a5be-fca90e38aaf5",
            "username": "Alex",
            "access_token": "token",
            "refresh_token": "r",
            "expires": "2030-01-01T00:00:00Z",
            "kind": match kind { AccountKind::Microsoft => "microsoft", AccountKind::Offline => "offline" },
            "signed_out": signed_out,
        }))
        .unwrap()
    }

    #[test]
    fn only_a_signed_in_microsoft_account_gets_a_session() {
        let s = Session::for_account(&account(AccountKind::Microsoft, false)).unwrap();
        let json: serde_json::Value = serde_json::from_str(&s.to_json()).unwrap();
        assert_eq!(json["uuid"], "069a79f444e94726a5befca90e38aaf5");
        assert_eq!(json["user_type"], "msa");
        assert!(
            !format!("{s:?}").contains("token"),
            "the token never reaches logs"
        );
        assert!(Session::for_account(&account(AccountKind::Offline, false)).is_err());
        assert!(Session::for_account(&account(AccountKind::Microsoft, true)).is_err());
    }

    #[test]
    fn the_session_never_goes_on_the_command_line() {
        let l = Layout::under(Path::new("root"));
        let args: Vec<_> = l
            .launch_command()
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert!(args.contains(&"--session-stdin".to_owned()));
        assert!(args.iter().all(|a| !a.contains("token")));
        let prep: Vec<_> = l
            .prepare_command(Path::new("mc"), None)
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert!(
            prep.windows(2)
                .any(|w| w[0] == "--server-jar" && w[1].ends_with("26.3.jar"))
        );
    }

    #[test]
    fn an_offline_session_carries_no_session() {
        let l = Layout::under(Path::new("root"));
        let args: Vec<_> = l
            .offline_command()
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert!(!args.iter().any(|a| a.starts_with("--session")));
        assert!(args.contains(&"--resources".to_owned()));
    }

    #[test]
    fn server_jar_comes_from_the_players_version_json() {
        let dir = std::env::temp_dir().join(format!("pvp_launch_test_{}", std::process::id()));
        let v = dir.join("versions").join(MINECRAFT);
        std::fs::create_dir_all(&v).unwrap();
        std::fs::write(
            v.join(format!("{MINECRAFT}.json")),
            r#"{"downloads":{"server":{"url":"https://example/s.jar","sha1":"abc","size":5}}}"#,
        )
        .unwrap();
        let jar = server_jar_source(&dir).unwrap();
        assert_eq!(jar.sha1, "abc");
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(server_jar_source(&dir).is_err());
    }

    #[test]
    fn releases_and_checksums() {
        let json = r#"{"tag_name":"v0.1.0","assets":[
            {"name":"client-windows-x64.zip","browser_download_url":"https://x/c.zip"},
            {"name":"SHA256SUMS","browser_download_url":"https://x/sums"}]}"#;
        let r = release_from_json(json, "client-windows-x64.zip").unwrap();
        assert_eq!(r.tag, "v0.1.0");
        assert_eq!(r.archive_url, "https://x/c.zip");
        assert!(release_from_json(json, "client-linux-x64.zip").is_none());
        let sums = "aa11  client-linux-x64.zip
bb22  client-windows-x64.zip
";
        assert_eq!(sha256_for(sums, "client-windows-x64.zip"), Some("bb22"));
        assert_eq!(sha256_for(sums, "other.zip"), None);
        assert_eq!(progress_fraction(r#"{"fraction": 0.5}"#), Some(0.5));
    }

    #[test]
    fn progress_lines() {
        assert_eq!(
            progress_message(r#"{"message":"Artifacts"}"#).as_deref(),
            Some("Artifacts")
        );
        assert_eq!(progress_message("not json"), None);
    }
}
