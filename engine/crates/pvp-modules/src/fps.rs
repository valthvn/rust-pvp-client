//! FPS display: frames drawn in the last second, as a HUD element. The phase 1 witness module: it exercises
//! the settings (toggles, colour), the HUD placement and the profile round trip end to end.

use mod_api::{ClientHost, DrawContext};
use pvp_core::{Anchor, Category, Cx, Events, HudElement, HudPos, Module, Setting};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct Fps;
pub static FPS: Fps = Fps;

const LABEL: usize = 0;
const BACKGROUND: usize = 1;
const COLOR: usize = 2;
const SHADOW: usize = 3;

static SETTINGS: [Setting; 4] = [
    Setting::toggle("label", "pvp.fps.label", true),
    Setting::toggle("background", "pvp.hud.background", true),
    Setting::color("color", "pvp.hud.text_color", 0xFFFF_FFFF),
    Setting::toggle("shadow", "pvp.hud.shadow", true),
];

impl Module for Fps {
    fn id(&self) -> &'static str {
        "fps"
    }
    fn name(&self) -> &'static str {
        "FPS"
    }
    fn description(&self) -> &'static str {
        "pvp.fps.description"
    }
    fn category(&self) -> Category {
        Category::Hud
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::HUD
    }
    fn hud(&self) -> &'static [&'static dyn HudElement] {
        &HUD
    }
}

/// Counts frames over one-second windows.
#[derive(Debug, Default)]
struct Window {
    start: Option<Instant>,
    frames: u32,
    last: u32,
}

impl Window {
    /// Counts a frame at `now`; the last full second's count.
    fn frame(&mut self, now: Instant) -> u32 {
        let start = *self.start.get_or_insert(now);
        self.frames += 1;
        let elapsed = now.duration_since(start);
        if elapsed >= Duration::from_secs(1) {
            self.last = (self.frames as f64 / elapsed.as_secs_f64()).round() as u32;
            self.frames = 0;
            self.start = Some(now);
        }
        self.last
    }
}

struct Counter {
    window: Mutex<Window>,
}

static HUD: [&dyn HudElement; 1] = [&COUNTER];

static COUNTER: Counter = Counter {
    window: Mutex::new(Window {
        start: None,
        frames: 0,
        last: 0,
    }),
};

/// Padding around the text when the background is on (OneConfig's HUD default, `HudStyle`).
const PAD: f32 = 4.0;

fn text(cx: &Cx, fps: u32) -> String {
    if cx.bool(LABEL) {
        format!("{fps} FPS")
    } else {
        fps.to_string()
    }
}

impl HudElement for Counter {
    fn id(&self) -> &'static str {
        "counter"
    }
    fn label(&self) -> &'static str {
        "pvp.fps.counter"
    }
    fn default_pos(&self) -> HudPos {
        HudPos {
            anchor: Anchor::TopLeft,
            x: 4.0,
            y: 4.0,
            scale: 1.0,
        }
    }
    fn size(&self, cx: &Cx, _: &dyn ClientHost, draw: &dyn DrawContext) -> [f32; 2] {
        // The widest it gets, so the box doesn't jitter as digits change.
        let w = draw.text_width(&text(cx, 888));
        [w + 2.0 * PAD, 9.0 + 2.0 * PAD]
    }
    fn draw(&self, cx: &Cx, host: &dyn ClientHost, draw: &mut dyn DrawContext, at: [f32; 2]) {
        let fps = self
            .window
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .frame(Instant::now());
        let [w, h] = self.size(cx, host, draw);
        if cx.bool(BACKGROUND) {
            draw.fill([at[0], at[1], w, h], 0x8000_0000);
        }
        let s = text(cx, fps);
        let tw = draw.text_width(&s);
        draw.text(
            &s,
            [at[0] + (w - tw) / 2.0, at[1] + PAD + 1.0],
            cx.color(COLOR),
            cx.bool(SHADOW),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_frames_per_second() {
        let mut w = Window::default();
        let t0 = Instant::now();
        for i in 0..60 {
            w.frame(t0 + Duration::from_millis(i * 1000 / 60));
        }
        assert_eq!(w.last, 0, "no full second yet");
        let fps = w.frame(t0 + Duration::from_secs(1));
        assert!((59..=62).contains(&fps), "{fps}");
    }
}
