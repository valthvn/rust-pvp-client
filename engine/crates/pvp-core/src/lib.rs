//! PvP client core: what every module and the menu build on.
//!
//! - [`Module`]: one feature, a `static` with declarative [`Setting`]s, optional HUD elements and keybinds.
//! - [`registry`]: the compiled-in modules, their state, and per-event bitmasks of the modules that are on.
//! - [`config`]: versioned TOML profiles in `<game dir>/pvp-client/`.
//! - [`hud`]: anchored placement and editor snapping.
//! - [`runtime`]: the single statically linked mod that connects all of it to Horizon's mod API.
//!
//! Nothing here depends on a Horizon engine crate: only on the public mod API (docs/ARCHITECTURE.md § 3).

pub mod config;
pub mod hud;
pub mod module;
pub mod registry;
pub mod runtime;
pub mod setting;

pub use hud::{Anchor, HudPos};
pub use module::{Category, Cx, Events, HudElement, Keybind, Module};
pub use registry::Registry;
pub use setting::{Kind, Setting, Value};
