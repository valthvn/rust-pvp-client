//! The client's built-in modules, in menu order. Each reimplements a Polyfrost mod's features
//! (`docs/MODULES.md`) on `pvp_core::Module`; none copies Polyfrost code.
#![forbid(unsafe_code)]

pub mod armour;
pub mod crosshair;
pub mod effects;
pub mod fps;
pub mod hitbox;
pub mod info;
pub mod keystrokes;
pub mod nametags;
pub mod text;
pub mod toggle_sprint;
pub mod zoom;

use pvp_core::Module;

/// Every module, in the order the menu lists them.
pub static ALL: &[&dyn Module] = &[
    &fps::FPS,
    &info::PING,
    &info::COORDS,
    &info::CLOCK,
    &keystrokes::KEYSTROKES,
    &armour::ARMOUR,
    &effects::EFFECTS,
    &toggle_sprint::TOGGLE_SPRINT,
    &crosshair::CROSSHAIR,
    &hitbox::HITBOX,
    &nametags::NAMETAGS,
    &zoom::ZOOM,
];
