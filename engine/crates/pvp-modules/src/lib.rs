//! The client's built-in modules, in menu order. Each reimplements a Polyfrost mod's features
//! (`docs/MODULES.md`) on `pvp_core::Module`; none copies Polyfrost code.
#![forbid(unsafe_code)]

pub mod fps;

use pvp_core::Module;

/// Every module, in the order the menu lists them.
pub static ALL: &[&dyn Module] = &[&fps::FPS];
