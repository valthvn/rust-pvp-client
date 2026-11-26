//! The client's module profiles, as `pvp-core` keeps them in the game folder: `pvp-client/profiles/<name>.toml`,
//! the active one named in `pvp-client/state.toml`. The launcher only lists them and picks the active one; the
//! in-game menu edits them.

use std::path::{Path, PathBuf};

pub const DEFAULT: &str = "default";

fn root(instance: &Path) -> PathBuf {
    instance.join("pvp-client")
}

/// `pvp-core`'s rule: letters, digits, `-`, `_` and spaces, 1 to 32 characters, no outer spaces.
pub fn valid_name(name: &str) -> bool {
    (1..=32).contains(&name.chars().count())
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | ' '))
        && name.trim() == name
}

/// The profiles on disk, sorted, the default one always among them.
pub fn list(instance: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(root(instance).join("profiles"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            (p.extension()? == "toml").then(|| p.file_stem()?.to_str().map(str::to_owned))?
        })
        .filter(|n| valid_name(n))
        .collect();
    if !names.iter().any(|n| n == DEFAULT) {
        names.push(DEFAULT.to_owned());
    }
    names.sort();
    names
}

/// The active profile (the default one when none is recorded).
pub fn active(instance: &Path) -> String {
    std::fs::read_to_string(root(instance).join("state.toml"))
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                let v = l
                    .trim()
                    .strip_prefix("profile")?
                    .trim_start()
                    .strip_prefix('=')?;
                Some(v.trim().trim_matches('"').to_owned())
            })
        })
        .filter(|n| valid_name(n))
        .unwrap_or_else(|| DEFAULT.to_owned())
}

/// Makes `name` the profile the game loads next time it starts.
pub fn set_active(instance: &Path, name: &str) -> std::io::Result<()> {
    if !valid_name(name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "not a profile name",
        ));
    }
    let dir = root(instance);
    std::fs::create_dir_all(&dir)?;
    let tmp = dir.join("state.toml.tmp");
    std::fs::write(&tmp, format!("profile = \"{name}\"\nschema = 1\n"))?;
    std::fs::rename(tmp, dir.join("state.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_and_switches_profiles() {
        let dir = std::env::temp_dir().join(format!("pvp_profiles_{}", std::process::id()));
        let p = dir.join("pvp-client/profiles");
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(p.join("ranked.toml"), "schema = 1\n").unwrap();
        std::fs::write(p.join("bad/name.toml.bak"), "").ok();
        assert_eq!(list(&dir), vec!["default".to_owned(), "ranked".to_owned()]);
        assert_eq!(active(&dir), "default");
        set_active(&dir, "ranked").unwrap();
        assert_eq!(active(&dir), "ranked");
        assert!(set_active(&dir, "../x").is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
