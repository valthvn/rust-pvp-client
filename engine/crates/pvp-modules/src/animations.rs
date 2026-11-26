//! 1.7/1.8 animations (OverflowAnimations), as far as 26.3 allows: the sword block, flat item pose and hand
//! swing of 1.8.9, the attack indicator hidden, the attack sounds 1.9 added silenced. Horizon draws all of
//! it as its Combat feel Legacy; this module turns its look on from our menu (the engine's combat override,
//! installed by `pvp-client`). Only pictures and sounds change: never what is sent to a server, nor the
//! rules a hosted world fights by.

use pvp_core::{Category, Module, Setting, registry};
use std::sync::OnceLock;

pub struct Animations;
pub static ANIMATIONS: Animations = Animations;

static SETTINGS: [Setting; 3] = [
    Setting::toggle("legacy", "pvp.animations.legacy", true),
    Setting::toggle("indicator", "pvp.animations.indicator", false),
    Setting::toggle("sounds", "pvp.animations.sounds", true),
];

/// What the module asks for, `None` while it is off (the game's own Combat feel applies).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Look {
    /// 1.8.9's pictures (block, pose, swing).
    pub legacy: bool,
    /// Keep the attack indicator.
    pub indicator: bool,
    /// Silence the attack sounds 1.9 added.
    pub sounds: bool,
}

pub fn wanted() -> Option<Look> {
    static I: OnceLock<Option<usize>> = OnceLock::new();
    let reg = registry::get()?;
    let i = (*I.get_or_init(|| reg.index_of("animations")))?;
    if !reg.is_enabled(i) {
        return None;
    }
    let b = |s| reg.value(i, s).as_bool();
    Some(Look {
        legacy: b(0),
        indicator: b(1),
        sounds: b(2),
    })
}

impl Module for Animations {
    fn id(&self) -> &'static str {
        "animations"
    }
    fn name(&self) -> &'static str {
        "Old Animations"
    }
    fn description(&self) -> &'static str {
        "pvp.animations.description"
    }
    fn category(&self) -> Category {
        Category::Visual
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
}
