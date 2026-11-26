//! The executable's stack size, as Horizon's game executables get it (`packaging/executable_link.rs`):
//! the wasm guests need a large main-thread stack, which Windows and macOS fix at link time. Horizon's
//! icon is not embedded (its branding; see `packaging/windows/exe.rs`).
#[path = "../../packaging/executable_link.rs"]
mod executable_link;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo sets the target OS");
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    println!("cargo:rerun-if-changed=../../packaging/executable_link.rs");
    println!("cargo:rerun-if-changed=../horizon-guest-stack/src/guest_stack_constants.rs");
    for arg in executable_link::arguments(&target_os, &target_env) {
        println!("cargo:rustc-link-arg-bin=pvp-client={arg}");
    }
}
