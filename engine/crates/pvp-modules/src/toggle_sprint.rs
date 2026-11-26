//! Toggle sprint and sneak (PolySprint): a key toggles holding the game's own sprint or sneak mapping, so the
//! game acts exactly as if the key were held (the server sees a normal client). A HUD line tells the state.

use crate::text::{self, Line};
use mod_api::ClientHost;
use pvp_core::{Anchor, Category, Cx, Events, HudElement, HudPos, Keybind, Module, Setting};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ToggleSprint;
pub static TOGGLE_SPRINT: ToggleSprint = ToggleSprint;

const SPRINT: usize = 3;
const SNEAK: usize = 4;

static SETTINGS: [Setting; 5] = [
    text::BACKGROUND,
    text::COLOR,
    text::SHADOW,
    Setting::toggle("sprint", "pvp.toggle_sprint.sprint", true),
    Setting::toggle("sneak", "pvp.toggle_sprint.sneak", true),
];

static KEYS: [Keybind; 2] = [
    Keybind {
        id: "sprint",
        label: "pvp.toggle_sprint.key_sprint",
        default_key: "key.keyboard.r",
    },
    Keybind {
        id: "sneak",
        label: "pvp.toggle_sprint.key_sneak",
        default_key: "key.keyboard.unknown",
    },
];

/// Toggled on by the keys.
static SPRINTING: AtomicBool = AtomicBool::new(false);
static SNEAKING: AtomicBool = AtomicBool::new(false);

/// What the HUD line says.
fn state(sprint_toggled: bool, sneak_toggled: bool, sprint_held: bool) -> Option<&'static str> {
    if sneak_toggled {
        Some("[Sneaking (Toggled)]")
    } else if sprint_toggled {
        Some("[Sprinting (Toggled)]")
    } else if sprint_held {
        Some("[Sprinting (Key Held)]")
    } else {
        None
    }
}

fn down(host: &dyn ClientHost, mapping: &str) -> bool {
    host.mapping(mapping).ok().flatten().is_some_and(|m| m.down)
}

/// Holds `mapping` while `on`. It is set only when not already down: with the game's own toggle option on,
/// setting it down flips it, so asking every tick would flicker.
fn hold(host: &mut dyn ClientHost, mapping: &str, on: bool) {
    if on && !down(host, mapping) {
        let _ = host.set_mapping_down(mapping, true);
    }
}

static HUD: [&dyn HudElement; 1] = [&STATE];
static STATE: Line = Line {
    id: "state",
    label: "pvp.toggle_sprint.state",
    pos: HudPos::new(Anchor::BottomLeft, 4.0, -4.0),
    widest: |_| "[Sprinting (Key Held)]".to_owned(),
    text: |cx, host| {
        state(
            cx.bool(SPRINT) && SPRINTING.load(Ordering::Relaxed),
            cx.bool(SNEAK) && SNEAKING.load(Ordering::Relaxed),
            down(host, "key.sprint"),
        )
        .map(str::to_owned)
    },
};

impl Module for ToggleSprint {
    fn id(&self) -> &'static str {
        "toggle_sprint"
    }
    fn name(&self) -> &'static str {
        "Toggle Sprint"
    }
    fn description(&self) -> &'static str {
        "pvp.toggle_sprint.description"
    }
    fn category(&self) -> Category {
        Category::Pvp
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::TICK
    }
    fn keybinds(&self) -> &'static [Keybind] {
        &KEYS
    }
    fn hud(&self) -> &'static [&'static dyn HudElement] {
        &HUD
    }
    // The module is turned off from the menu, a screen: opening it already let go of every held mapping.
    fn on_disable(&self, _: &Cx) {
        SPRINTING.store(false, Ordering::Relaxed);
        SNEAKING.store(false, Ordering::Relaxed);
    }
    fn on_keybind(&self, id: &str, cx: &Cx, host: &mut dyn ClientHost) {
        let (flag, mapping, allowed) = match id {
            "sprint" => (&SPRINTING, "key.sprint", cx.bool(SPRINT)),
            "sneak" => (&SNEAKING, "key.sneak", cx.bool(SNEAK)),
            _ => return,
        };
        if !allowed {
            return;
        }
        let now = !flag.load(Ordering::Relaxed);
        flag.store(now, Ordering::Relaxed);
        if !now {
            let _ = host.set_mapping_down(mapping, false);
        }
    }
    fn tick(&self, cx: &Cx, host: &mut dyn ClientHost) {
        if !host.playing() {
            return;
        }
        let sprint = cx.bool(SPRINT) && SPRINTING.load(Ordering::Relaxed);
        let sneak = cx.bool(SNEAK) && SNEAKING.load(Ordering::Relaxed);
        hold(host, "key.sprint", sprint);
        hold(host, "key.sneak", sneak);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sneaking_shows_before_sprinting() {
        assert_eq!(state(true, true, true), Some("[Sneaking (Toggled)]"));
        assert_eq!(state(true, false, false), Some("[Sprinting (Toggled)]"));
        assert_eq!(state(false, false, true), Some("[Sprinting (Key Held)]"));
        assert_eq!(state(false, false, false), None);
    }
}
