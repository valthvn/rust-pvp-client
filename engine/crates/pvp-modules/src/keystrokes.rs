//! Keystrokes and CPS (Keystrokes revamp): W A S D, the mouse buttons with their clicks per second and the
//! space bar, lit while held.

use crate::text;
use mod_api::{ClientHost, DrawContext};
use pvp_core::{Anchor, Category, Cx, Events, HudElement, HudPos, Module, Setting};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct Keystrokes;
pub static KEYSTROKES: Keystrokes = Keystrokes;

const PRESSED: usize = 3;
const PRESSED_TEXT: usize = 4;
const MOUSE: usize = 5;
const CPS: usize = 6;
const SPACE: usize = 7;

static SETTINGS: [Setting; 8] = [
    text::BACKGROUND,
    text::COLOR,
    text::SHADOW,
    Setting::color("pressed", "pvp.keystrokes.pressed", 0x80FF_FFFF),
    Setting::color("pressed_text", "pvp.keystrokes.pressed_text", 0xFF00_0000),
    Setting::toggle("mouse", "pvp.keystrokes.mouse", true),
    Setting::toggle("cps", "pvp.keystrokes.cps", true),
    Setting::toggle("space", "pvp.keystrokes.space", true),
];

/// One key's square, the gap between keys, the space bar's height.
const KEY: f32 = 22.0;
const GAP: f32 = 2.0;
const BAR: f32 = 10.0;
const WIDTH: f32 = 3.0 * KEY + 2.0 * GAP;

/// Click times of the left and right buttons over the last second.
#[derive(Default)]
pub(crate) struct Clicks {
    times: [VecDeque<Instant>; 2],
}

impl Clicks {
    pub(crate) fn click(&mut self, button: usize, now: Instant) {
        if let Some(q) = self.times.get_mut(button) {
            q.push_back(now);
            Self::trim(q, now);
        }
    }
    pub(crate) fn cps(&mut self, button: usize, now: Instant) -> usize {
        self.times.get_mut(button).map_or(0, |q| {
            Self::trim(q, now);
            q.len()
        })
    }
    fn trim(q: &mut VecDeque<Instant>, now: Instant) {
        while q
            .front()
            .is_some_and(|t| now.duration_since(*t) >= Duration::from_secs(1))
        {
            q.pop_front();
        }
    }
}

static CLICKS: Mutex<Clicks> = Mutex::new(Clicks {
    times: [VecDeque::new(), VecDeque::new()],
});

fn cps(button: usize) -> usize {
    CLICKS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .cps(button, Instant::now())
}

/// The layout as (label, mapping, rect relative to the element), rows top to bottom.
fn layout(cx: &Cx) -> Vec<(String, &'static str, [f32; 4])> {
    let step = KEY + GAP;
    let mut keys = vec![
        ("W".to_owned(), "key.forward", [step, 0.0, KEY, KEY]),
        ("A".to_owned(), "key.left", [0.0, step, KEY, KEY]),
        ("S".to_owned(), "key.back", [step, step, KEY, KEY]),
        ("D".to_owned(), "key.right", [2.0 * step, step, KEY, KEY]),
    ];
    let mut y = 2.0 * step;
    if cx.bool(MOUSE) {
        let half = (WIDTH - GAP) / 2.0;
        let (l, r) = if cx.bool(CPS) {
            (format!("{} CPS", cps(0)), format!("{} CPS", cps(1)))
        } else {
            ("LMB".to_owned(), "RMB".to_owned())
        };
        keys.push((l, "key.attack", [0.0, y, half, KEY]));
        keys.push((r, "key.use", [half + GAP, y, half, KEY]));
        y += step;
    }
    if cx.bool(SPACE) {
        keys.push((String::new(), "key.jump", [0.0, y, WIDTH, BAR]));
    }
    keys
}

struct Grid;
static HUD: [&dyn HudElement; 1] = [&Grid];

impl HudElement for Grid {
    fn id(&self) -> &'static str {
        "keys"
    }
    fn label(&self) -> &'static str {
        "pvp.keystrokes.keys"
    }
    fn default_pos(&self) -> HudPos {
        HudPos::new(Anchor::TopRight, -4.0, 22.0)
    }
    fn size(&self, cx: &Cx, _: &dyn ClientHost, _: &dyn DrawContext) -> [f32; 2] {
        let rows = 2.0 + f32::from(u8::from(cx.bool(MOUSE)));
        let mut h = rows * KEY + (rows - 1.0) * GAP;
        if cx.bool(SPACE) {
            h += GAP + BAR;
        }
        [WIDTH, h]
    }
    fn draw(&self, cx: &Cx, host: &dyn ClientHost, draw: &mut dyn DrawContext, at: [f32; 2]) {
        for (label, mapping, r) in layout(cx) {
            let down = host.mapping(mapping).ok().flatten().is_some_and(|m| m.down);
            let rect = [at[0] + r[0], at[1] + r[1], r[2], r[3]];
            if down {
                draw.fill(rect, cx.color(PRESSED));
            } else if cx.bool(text::I_BACKGROUND) {
                draw.fill(rect, 0x8000_0000);
            }
            if mapping == "key.jump" {
                // A line across the bar, as the space bar's mark.
                let c = if down {
                    cx.color(PRESSED_TEXT)
                } else {
                    cx.color(text::I_COLOR)
                };
                draw.fill([rect[0] + 12.0, rect[1] + 4.0, rect[2] - 24.0, 1.0], c);
                continue;
            }
            let color = if down {
                cx.color(PRESSED_TEXT)
            } else {
                cx.color(text::I_COLOR)
            };
            let tw = draw.text_width(&label);
            draw.text(
                &label,
                [
                    rect[0] + (rect[2] - tw) / 2.0,
                    rect[1] + (rect[3] - 8.0) / 2.0,
                ],
                color,
                cx.bool(text::I_SHADOW) && !down,
            );
        }
    }
}

impl Module for Keystrokes {
    fn id(&self) -> &'static str {
        "keystrokes"
    }
    fn name(&self) -> &'static str {
        "Keystrokes"
    }
    fn description(&self) -> &'static str {
        "pvp.keystrokes.description"
    }
    fn category(&self) -> Category {
        Category::Hud
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::MOUSE
    }
    fn hud(&self) -> &'static [&'static dyn HudElement] {
        &HUD
    }
    fn mouse(&self, _: &Cx, _: &mut dyn ClientHost, button: i32, pressed: bool) {
        if pressed && (0..2).contains(&button) {
            CLICKS
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .click(button as usize, Instant::now());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clicks_count_over_the_last_second() {
        let mut c = Clicks::default();
        let t0 = Instant::now();
        for i in 0..10 {
            c.click(0, t0 + Duration::from_millis(i * 100));
        }
        c.click(1, t0);
        assert_eq!(c.cps(0, t0 + Duration::from_millis(950)), 10);
        assert_eq!(c.cps(0, t0 + Duration::from_millis(1450)), 5);
        assert_eq!(c.cps(1, t0 + Duration::from_secs(1)), 0);
        assert_eq!(c.cps(2, t0), 0);
    }
}
