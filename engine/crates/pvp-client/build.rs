//! The executable's stack size, as Horizon's game executables get it (`packaging/executable_link.rs`):
//! the wasm guests need a large main-thread stack, which Windows and macOS fix at link time. Horizon's
//! icon is not embedded (its branding; see `packaging/windows/exe.rs`).
#[path = "../../packaging/executable_link.rs"]
mod executable_link;

/// The mods linked into the executable, whose manifests go beside it (`native-mods/<id>`).
const MODS: [&str; 2] = ["pvp", "pvp_ui"];

fn main() {
    copy_manifests();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo sets the target OS");
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    println!("cargo:rerun-if-changed=../../packaging/executable_link.rs");
    println!("cargo:rerun-if-changed=../horizon-guest-stack/src/guest_stack_constants.rs");
    for arg in executable_link::arguments(&target_os, &target_env) {
        println!("cargo:rustc-link-arg-bin=pvp-client={arg}");
    }
}

/// Copies `native-mods/<id>` to `<profile dir>/horizon-native-mods/<id>`, beside the executable, where the
/// mod host reads the manifests of built-in mods (`horizon_mod_package::runtime::Runtime::load`).
fn copy_manifests() {
    let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // OUT_DIR is <profile dir>/build/<package>-<hash>/out.
    let Some(profile) = out.ancestors().nth(3) else {
        return;
    };
    for id in MODS {
        let from = manifest_dir.join("native-mods").join(id);
        let to = profile.join("horizon-native-mods").join(id);
        for file in ["horizon-mod.toml", "Cargo.toml", "src/lib.rs"] {
            println!("cargo:rerun-if-changed={}", from.join(file).display());
            let dest = to.join(file);
            std::fs::create_dir_all(dest.parent().unwrap()).expect("creating horizon-native-mods");
            std::fs::copy(from.join(file), &dest).expect("copying a built-in mod's manifest");
        }
    }
}
