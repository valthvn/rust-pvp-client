//! Zoom (PolyZoom, OptiFine's zoom): while its key is held, the field of view is divided by a factor.

use mod_api::{ClientHost, ComputeFovModifierEvent};
use pvp_core::{Category, Cx, Events, Keybind, Module, Setting};

pub struct Zoom;
pub static ZOOM: Zoom = Zoom;

const FACTOR: usize = 0;

static SETTINGS: [Setting; 1] = [Setting::slider(
    "factor",
    "pvp.zoom.factor",
    2.0,
    10.0,
    0.5,
    4.0,
)];

static KEYS: [Keybind; 1] = [Keybind {
    id: "zoom",
    label: "pvp.zoom.key",
    default_key: "key.keyboard.c",
}];

/// The zoomed modifier, kept inside the range the game accepts (0.01 to 10).
fn zoomed(modifier: f32, factor: f32) -> f32 {
    (modifier / factor.max(1.0)).clamp(0.01, 10.0)
}

impl Module for Zoom {
    fn id(&self) -> &'static str {
        "zoom"
    }
    fn name(&self) -> &'static str {
        "Zoom"
    }
    fn description(&self) -> &'static str {
        "pvp.zoom.description"
    }
    fn category(&self) -> Category {
        Category::Visual
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::FOV
    }
    fn keybinds(&self) -> &'static [Keybind] {
        &KEYS
    }
    fn fov(&self, cx: &Cx, host: &dyn ClientHost, event: &mut ComputeFovModifierEvent<'_>) {
        if host.is_down(pvp_core::runtime::MOD_ID, "zoom.zoom") {
            event.modifier = zoomed(event.modifier, cx.f32(FACTOR));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_divides_and_stays_in_range() {
        assert_eq!(zoomed(1.0, 4.0), 0.25);
        assert_eq!(zoomed(0.02, 10.0), 0.01);
        assert_eq!(zoomed(1.0, 0.0), 1.0);
    }
}
