//! The PvP client executable. It starts the game the way `horizon_game::main` does, but hands the mod host
//! our runtime (`pvp_core::runtime::entry`) as a statically linked mod, as the launcher's generated
//! executable does for native mods (`horizon_mod_host::main_with`).
//!
//! Unlike `horizon-official`, it registers none of Horizon's online parts (the "I'm playing" heartbeat, the
//! official server entry, Play with Friends, Discord presence): this is not Horizon's official build, so it
//! must not report itself to Horizon's services.

#![forbid(unsafe_code)]

mod look;

/// The game identity marker for launcher imports, retained by `main`.
static GAME_IDENTITY: [u8; horizon_game::GAME_IDENTITY.len()] = horizon_game::GAME_IDENTITY;

/// The mods linked in, as their manifests beside the executable declare them (`native-mods/`, `build.rs`).
const BUILT: &[mod_api::__private::native::BuiltMod] = &[
    mod_api::__private::native::BuiltMod {
        id: pvp_core::runtime::MOD_ID,
        version: "0.1.0",
        side: mod_api::Side::Client,
    },
    mod_api::__private::native::BuiltMod {
        id: pvp_ui::entry::MOD_ID,
        version: "0.1.0",
        side: mod_api::Side::Client,
    },
];

/// Each built-in mod's package files, embedded: the mod host checks a manifest beside the executable
/// (`horizon-native-mods/<id>`) for every mod linked in, so the executable writes them there when they are missing
/// and works however it was copied.
const PACKAGES: [(&str, [(&str, &str); 3]); 2] = [
    (
        pvp_core::runtime::MOD_ID,
        [
            (
                "horizon-mod.toml",
                include_str!("../native-mods/pvp/horizon-mod.toml"),
            ),
            ("Cargo.toml", include_str!("../native-mods/pvp/Cargo.toml")),
            ("src/lib.rs", include_str!("../native-mods/pvp/src/lib.rs")),
        ],
    ),
    (
        pvp_ui::entry::MOD_ID,
        [
            (
                "horizon-mod.toml",
                include_str!("../native-mods/pvp_ui/horizon-mod.toml"),
            ),
            (
                "Cargo.toml",
                include_str!("../native-mods/pvp_ui/Cargo.toml"),
            ),
            (
                "src/lib.rs",
                include_str!("../native-mods/pvp_ui/src/lib.rs"),
            ),
        ],
    ),
];

/// Writes the built-in mods' packages beside the executable where they are missing or differ.
fn install_packages() {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let root = exe.with_file_name("horizon-native-mods");
    for (id, files) in PACKAGES {
        for (name, text) in files {
            let path = root.join(id).join(name);
            if std::fs::read_to_string(&path).is_ok_and(|t| t == text) {
                continue;
            }
            let written = path
                .parent()
                .map_or(Ok(()), std::fs::create_dir_all)
                .and_then(|()| std::fs::write(&path, text));
            if let Err(e) = written {
                eprintln!("pvp-client: cannot write {}: {e}", path.display());
            }
        }
    }
}

fn main() {
    horizon_game::run_main(|| {
        std::hint::black_box(&GAME_IDENTITY);
        install_packages();
        // The platform's menu look, as horizon_game::main gives it.
        horizon_game::give_menu_look();
        horizon_guest_stack::prepare_main();
        horizon_mod_host::exit_if_compile_child();
        let startup = horizon_client::entry::prepare();
        #[cfg(any(target_os = "linux", target_os = "macos", windows))]
        std::hint::black_box(&horizon_mod_package::holder::CAPABILITY);
        pvp_core::registry::init(pvp_modules::ALL);
        look::install();
        horizon_mod_host::main_with(
            vec![pvp_core::runtime::entry(), pvp_ui::entry()],
            &[],
            BUILT,
            startup,
        );
    });
}
