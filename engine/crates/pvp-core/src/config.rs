//! Profiles: the modules' switches, settings and HUD placements in `<game dir>/pvp-client/profiles/<name>.toml`,
//! with the active profile's name in `<game dir>/pvp-client/state.toml`. Each file carries `schema`; an older file
//! is migrated (after a `.bak` copy) and an unreadable value falls back to its default, never failing the load.
//! `options.txt` and Horizon's own files are never touched.

use crate::hud::HudPos;
use crate::registry::Registry;
use std::path::{Path, PathBuf};
use toml::Table;

/// The profile format this build writes.
pub const SCHEMA: i64 = 1;

/// Migrations: `MIGRATIONS[n]` takes a schema `n + 1` table to schema `n + 2`. Empty while there is one schema.
pub static MIGRATIONS: &[fn(&mut Table)] = &[];

pub const DEFAULT_PROFILE: &str = "default";

/// The client's folder in the game directory.
pub fn root(game_dir: &Path) -> PathBuf {
    game_dir.join("pvp-client")
}

fn profiles_dir(game_dir: &Path) -> PathBuf {
    root(game_dir).join("profiles")
}

/// A profile name is a file name: letters, digits, `-`, `_` and spaces, 1 to 32 characters.
pub fn valid_name(name: &str) -> bool {
    (1..=32).contains(&name.chars().count())
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | ' '))
        && name.trim() == name
}

fn profile_path(game_dir: &Path, name: &str) -> PathBuf {
    profiles_dir(game_dir).join(format!("{name}.toml"))
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidName,
    Exists,
    Missing,
    Io(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidName => f.write_str("invalid profile name"),
            ConfigError::Exists => f.write_str("a profile with that name exists"),
            ConfigError::Missing => f.write_str("no such profile"),
            ConfigError::Io(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

/// Writes `text` to `path` through a temporary file and a rename, so a crash never leaves half a file.
fn write_atomic(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("toml.tmp");
    std::fs::write(&tmp, text)?;
    std::fs::rename(&tmp, path)
}

/// The registry's state as a profile table.
pub fn to_table(reg: &Registry) -> Table {
    let mut modules = Table::new();
    for (i, m) in reg.modules().iter().enumerate() {
        let mut entry = Table::new();
        entry.insert("enabled".into(), reg.is_enabled(i).into());
        let mut settings = Table::new();
        for (s, value) in m.settings().iter().zip(reg.values(i)) {
            settings.insert(s.id.into(), s.to_toml(&value));
        }
        if !settings.is_empty() {
            entry.insert("settings".into(), settings.into());
        }
        let mut hud = Table::new();
        for (e, el) in m.hud().iter().enumerate() {
            if let Ok(t) = Table::try_from(reg.hud_pos(i, e)) {
                hud.insert(el.id().into(), t.into());
            }
        }
        if !hud.is_empty() {
            entry.insert("hud".into(), hud.into());
        }
        modules.insert(m.id().into(), entry.into());
    }
    let mut t = Table::new();
    t.insert("schema".into(), SCHEMA.into());
    t.insert("modules".into(), modules.into());
    t
}

/// Brings `table` to the current schema. Returns whether it changed. A table from a newer build is left as is
/// (its known keys still load).
pub fn migrate(table: &mut Table) -> bool {
    let mut schema = table
        .get("schema")
        .and_then(|v| v.as_integer())
        .unwrap_or(1);
    let start = schema;
    while schema < SCHEMA {
        if let Some(step) = usize::try_from(schema - 1)
            .ok()
            .and_then(|n| MIGRATIONS.get(n))
        {
            step(table);
        }
        schema += 1;
        table.insert("schema".into(), schema.into());
    }
    schema != start
}

/// Applies a profile table to the registry: everything not in it goes back to its default. Unknown modules and
/// settings are ignored, invalid values keep the default. Modules are switched last, so `on_enable` sees the
/// profile's settings.
pub fn apply(reg: &Registry, table: &Table) {
    reg.reset();
    let modules = table.get("modules").and_then(|v| v.as_table());
    let mut switch_on = Vec::new();
    for (i, m) in reg.modules().iter().enumerate() {
        let Some(entry) = modules
            .and_then(|t| t.get(m.id()))
            .and_then(|v| v.as_table())
        else {
            continue;
        };
        if let Some(settings) = entry.get("settings").and_then(|v| v.as_table()) {
            for (s, spec) in m.settings().iter().enumerate() {
                if let Some(v) = settings.get(spec.id).and_then(|v| spec.from_toml(v)) {
                    reg.set_value(i, s, v);
                }
            }
        }
        if let Some(hud) = entry.get("hud").and_then(|v| v.as_table()) {
            for (e, el) in m.hud().iter().enumerate() {
                if let Some(pos) = hud
                    .get(el.id())
                    .and_then(|v| v.clone().try_into::<HudPos>().ok())
                {
                    reg.set_hud_pos(i, e, pos);
                }
            }
        }
        if entry.get("enabled").and_then(|v| v.as_bool()) == Some(true) {
            switch_on.push(i);
        }
    }
    for i in switch_on {
        reg.set_enabled(i, true);
    }
    reg.take_dirty();
}

/// The profiles on disk, sorted by name.
pub fn list(game_dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(profiles_dir(game_dir))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            (p.extension()? == "toml").then(|| p.file_stem()?.to_str().map(str::to_owned))?
        })
        .filter(|n| valid_name(n))
        .collect();
    names.sort();
    names
}

/// The active profile's name (the default one when none is recorded or it is gone).
pub fn active(game_dir: &Path) -> String {
    std::fs::read_to_string(root(game_dir).join("state.toml"))
        .ok()
        .and_then(|s| s.parse::<Table>().ok())
        .and_then(|t| t.get("profile")?.as_str().map(str::to_owned))
        .filter(|n| valid_name(n) && profile_path(game_dir, n).exists())
        .unwrap_or_else(|| DEFAULT_PROFILE.to_owned())
}

fn set_active(game_dir: &Path, name: &str) -> std::io::Result<()> {
    let mut t = Table::new();
    t.insert("schema".into(), SCHEMA.into());
    t.insert("profile".into(), name.into());
    write_atomic(&root(game_dir).join("state.toml"), &t.to_string())
}

/// Loads profile `name` into the registry and makes it the active one. A missing profile loads the defaults.
pub fn load(reg: &Registry, game_dir: &Path, name: &str) -> Result<(), ConfigError> {
    if !valid_name(name) {
        return Err(ConfigError::InvalidName);
    }
    let path = profile_path(game_dir, name);
    let table = match std::fs::read_to_string(&path) {
        Ok(text) => match text.parse::<Table>() {
            Ok(mut t) => {
                if migrate(&mut t) {
                    let _ = std::fs::copy(&path, path.with_extension("toml.bak"));
                    write_atomic(&path, &t.to_string())?;
                }
                t
            }
            Err(e) => {
                // Keep the damaged file for the player, start from the defaults.
                log::warn!("pvp: profile {name} is not valid TOML ({e}); using defaults");
                let _ = std::fs::copy(&path, path.with_extension("toml.bak"));
                Table::new()
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Table::new(),
        Err(e) => return Err(e.into()),
    };
    apply(reg, &table);
    set_active(game_dir, name)?;
    Ok(())
}

/// Saves the registry into profile `name`.
pub fn save(reg: &Registry, game_dir: &Path, name: &str) -> Result<(), ConfigError> {
    if !valid_name(name) {
        return Err(ConfigError::InvalidName);
    }
    write_atomic(&profile_path(game_dir, name), &to_table(reg).to_string())?;
    Ok(())
}

/// A new profile `to`, a copy of `from` (or of the defaults when `from` doesn't exist).
pub fn duplicate(game_dir: &Path, from: &str, to: &str) -> Result<(), ConfigError> {
    if !valid_name(from) || !valid_name(to) {
        return Err(ConfigError::InvalidName);
    }
    let dest = profile_path(game_dir, to);
    if dest.exists() {
        return Err(ConfigError::Exists);
    }
    let text = std::fs::read_to_string(profile_path(game_dir, from)).unwrap_or_default();
    write_atomic(&dest, &text)?;
    Ok(())
}

pub fn rename(game_dir: &Path, from: &str, to: &str) -> Result<(), ConfigError> {
    if !valid_name(from) || !valid_name(to) {
        return Err(ConfigError::InvalidName);
    }
    let (src, dest) = (profile_path(game_dir, from), profile_path(game_dir, to));
    if !src.exists() {
        return Err(ConfigError::Missing);
    }
    if dest.exists() {
        return Err(ConfigError::Exists);
    }
    std::fs::rename(src, dest)?;
    if active(game_dir) == from || !profile_path(game_dir, &active(game_dir)).exists() {
        set_active(game_dir, to)?;
    }
    Ok(())
}

/// Deletes profile `name`. The last profile can't be deleted.
pub fn delete(game_dir: &Path, name: &str) -> Result<(), ConfigError> {
    if !valid_name(name) {
        return Err(ConfigError::InvalidName);
    }
    let path = profile_path(game_dir, name);
    if !path.exists() {
        return Err(ConfigError::Missing);
    }
    if list(game_dir).len() <= 1 {
        return Err(ConfigError::Exists);
    }
    std::fs::remove_file(path)?;
    Ok(())
}

/// Copies profile `name` to `dest` (any path), for sharing.
pub fn export(game_dir: &Path, name: &str, dest: &Path) -> Result<(), ConfigError> {
    let src = profile_path(game_dir, name);
    if !src.exists() {
        return Err(ConfigError::Missing);
    }
    std::fs::copy(src, dest)?;
    Ok(())
}

/// Imports a profile file as profile `name` (must not exist). The file must parse as TOML.
pub fn import(game_dir: &Path, file: &Path, name: &str) -> Result<(), ConfigError> {
    if !valid_name(name) {
        return Err(ConfigError::InvalidName);
    }
    let dest = profile_path(game_dir, name);
    if dest.exists() {
        return Err(ConfigError::Exists);
    }
    let text = std::fs::read_to_string(file)?;
    let mut table: Table = text.parse().map_err(|e| {
        ConfigError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{e}"),
        ))
    })?;
    migrate(&mut table);
    write_atomic(&dest, &table.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hud::{Anchor, HudPos};
    use crate::module::{Category, Cx, HudElement, Module};
    use crate::setting::{Setting, Value};
    use mod_api::{ClientHost, DrawContext};

    struct Box2;
    impl HudElement for Box2 {
        fn id(&self) -> &'static str {
            "main"
        }
        fn label(&self) -> &'static str {
            "x"
        }
        fn default_pos(&self) -> HudPos {
            HudPos::new(Anchor::TopLeft, 2.0, 2.0)
        }
        fn size(&self, _: &Cx, _: &dyn ClientHost, _: &dyn DrawContext) -> [f32; 2] {
            [10.0, 10.0]
        }
        fn draw(&self, _: &Cx, _: &dyn ClientHost, _: &mut dyn DrawContext, _: [f32; 2]) {}
    }

    struct M;
    static SETTINGS: [Setting; 3] = [
        Setting::toggle("flag", "x", false),
        Setting::slider("size", "x", 1.0, 10.0, 1.0, 4.0),
        Setting::choice("shape", "x", &["cross", "dot"], 0),
    ];
    static HUD: [&dyn HudElement; 1] = [&Box2];
    impl Module for M {
        fn id(&self) -> &'static str {
            "m"
        }
        fn name(&self) -> &'static str {
            "x"
        }
        fn description(&self) -> &'static str {
            "x"
        }
        fn category(&self) -> Category {
            Category::Hud
        }
        fn settings(&self) -> &'static [Setting] {
            &SETTINGS
        }
        fn hud(&self) -> &'static [&'static dyn HudElement] {
            &HUD
        }
    }
    static MODS: [&dyn Module; 1] = [&M];

    fn temp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("pvp-core-test-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_saved_profile_loads_back_identically() {
        let dir = temp_dir("roundtrip");
        let reg = Registry::new(&MODS);
        reg.set_enabled(0, true);
        reg.set_value(0, 1, Value::Num(7.0));
        reg.set_value(0, 2, Value::Choice(1));
        reg.set_hud_pos(
            0,
            0,
            HudPos {
                anchor: Anchor::BottomRight,
                x: -4.0,
                y: -6.0,
                scale: 1.5,
            },
        );
        save(&reg, &dir, "pvp").unwrap();

        let fresh = Registry::new(&MODS);
        load(&fresh, &dir, "pvp").unwrap();
        assert!(fresh.is_enabled(0));
        assert_eq!(fresh.values(0), reg.values(0));
        assert_eq!(fresh.hud_pos(0, 0), reg.hud_pos(0, 0));
        assert_eq!(active(&dir), "pvp");
        assert!(!fresh.take_dirty(), "loading is not a change to save");
    }

    #[test]
    fn bad_values_and_a_damaged_file_fall_back_to_defaults() {
        let dir = temp_dir("bad");
        let reg = Registry::new(&MODS);
        std::fs::create_dir_all(profiles_dir(&dir)).unwrap();
        std::fs::write(
            profile_path(&dir, "p"),
            "schema = 1\n[modules.m]\nenabled = true\n[modules.m.settings]\nsize = 99\nshape = \"star\"\nflag = \"yes\"\n[modules.unknown]\nenabled = true\n",
        )
        .unwrap();
        load(&reg, &dir, "p").unwrap();
        assert!(reg.is_enabled(0));
        assert_eq!(
            reg.values(0),
            vec![Value::Bool(false), Value::Num(10.0), Value::Choice(0)]
        );

        std::fs::write(profile_path(&dir, "broken"), "this is [ not toml").unwrap();
        load(&reg, &dir, "broken").unwrap();
        assert!(!reg.is_enabled(0));
        assert!(
            profile_path(&dir, "broken")
                .with_extension("toml.bak")
                .exists()
        );
    }

    #[test]
    fn a_file_without_schema_is_treated_as_schema_one() {
        let mut t: Table = "[modules.m]\nenabled = true\n".parse().unwrap();
        assert!(!migrate(&mut t) || t["schema"].as_integer() == Some(SCHEMA));
        assert_eq!(
            t.get("schema").and_then(|v| v.as_integer()).unwrap_or(1),
            SCHEMA
        );
    }

    #[test]
    fn profiles_can_be_duplicated_renamed_and_deleted_but_not_the_last_one() {
        let dir = temp_dir("manage");
        let reg = Registry::new(&MODS);
        save(&reg, &dir, "a").unwrap();
        duplicate(&dir, "a", "b").unwrap();
        assert!(matches!(
            duplicate(&dir, "a", "b"),
            Err(ConfigError::Exists)
        ));
        rename(&dir, "b", "c").unwrap();
        assert_eq!(list(&dir), vec!["a".to_string(), "c".to_string()]);
        delete(&dir, "c").unwrap();
        assert!(matches!(delete(&dir, "a"), Err(ConfigError::Exists)));
        assert!(matches!(
            save(&reg, &dir, "../evil"),
            Err(ConfigError::InvalidName)
        ));
    }

    #[test]
    fn an_exported_profile_imports_under_a_new_name() {
        let dir = temp_dir("share");
        let reg = Registry::new(&MODS);
        reg.set_value(0, 1, Value::Num(9.0));
        save(&reg, &dir, "mine").unwrap();
        let file = dir.join("shared.toml");
        export(&dir, "mine", &file).unwrap();
        import(&dir, &file, "theirs").unwrap();
        let other = Registry::new(&MODS);
        load(&other, &dir, "theirs").unwrap();
        assert_eq!(other.value(0, 1), Value::Num(9.0));
    }
}
