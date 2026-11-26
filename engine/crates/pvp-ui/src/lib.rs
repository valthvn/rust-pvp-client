//! The PvP client's in-game UI, reproducing OneConfig's (Polyfrost): the module menu, the HUD editor and
//! the drawing they share. It draws through Horizon's mod API only: Poppins text and rounded shapes are
//! rasterized here into one dynamic GUI texture (`gfx::ATLAS_ID`).
#![forbid(unsafe_code)]

pub mod atlas;
pub mod ctx;
pub mod entry;
pub mod font;
pub mod gfx;
pub mod hud_editor;
pub mod icons;
pub mod menu;
pub mod shape;
pub mod theme;
pub mod widgets;

pub use entry::entry;
