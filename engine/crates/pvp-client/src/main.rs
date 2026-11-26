//! The PvP client executable. It starts the game the way `horizon_game::main` does, but hands the mod host
//! our runtime (`pvp_core::runtime::entry`) as a statically linked mod, as the launcher's generated
//! executable does for native mods (`horizon_mod_host::main_with`).
//!
//! Unlike `horizon-official`, it registers none of Horizon's online parts (the "I'm playing" heartbeat, the
//! official server entry, Play with Friends, Discord presence): this is not Horizon's official build, so it
//! must not report itself to Horizon's services.
#![forbid(unsafe_code)]

/// The game identity marker for launcher imports, retained by `main`.
static GAME_IDENTITY: [u8; horizon_game::GAME_IDENTITY.len()] = horizon_game::GAME_IDENTITY;

fn main() {
    horizon_game::run_main(|| {
        std::hint::black_box(&GAME_IDENTITY);
        // The platform's menu look, as horizon_game::main gives it.
        horizon_game::give_menu_look();
        horizon_guest_stack::prepare_main();
        horizon_mod_host::exit_if_compile_child();
        let startup = horizon_client::entry::prepare();
        #[cfg(any(target_os = "linux", target_os = "macos", windows))]
        std::hint::black_box(&horizon_mod_package::holder::CAPABILITY);
        pvp_core::registry::init(pvp_modules::ALL);
        horizon_mod_host::main_with(
            vec![pvp_core::runtime::entry(), pvp_ui::entry()],
            &[],
            &[],
            startup,
        );
    });
}
