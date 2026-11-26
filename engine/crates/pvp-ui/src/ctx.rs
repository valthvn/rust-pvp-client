//! Immediate-mode plumbing: the input that arrived since the last frame, hit testing against what is drawn
//! this frame, and OneConfig's 150 ms transitions.

use std::collections::HashMap;

/// Input the screen hooks hand the UI, queued until the next frame draws.
#[derive(Clone, Debug, PartialEq)]
pub enum Input {
    Press(i32),
    Release(i32),
    Drag(i32, [f32; 2]),
    Scroll([f32; 2]),
    Key(String, u32),
    Char(char),
}

/// OneConfig's standard transition (`Motion.standard`: 150 ms, FastOutSlowIn).
pub const STANDARD: f32 = 0.15;

/// Compose's `FastOutSlowInEasing`, cubic-bezier(0.4, 0, 0.2, 1).
pub fn fast_out_slow_in(t: f32) -> f32 {
    cubic_bezier(0.4, 0.0, 0.2, 1.0, t)
}

/// `1 - 2^(-10t)`, the shell's opening curve.
pub fn ease_out_expo(t: f32) -> f32 {
    if t >= 1.0 {
        1.0
    } else {
        1.0 - 2f32.powf(-10.0 * t)
    }
}

pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

/// The y of a CSS-style cubic bezier at x = `t`.
fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let bez = |a: f32, b: f32, s: f32| {
        3.0 * a * s * (1.0 - s).powi(2) + 3.0 * b * s * s * (1.0 - s) + s.powi(3)
    };
    // Solve x(s) = t by bisection (monotone in s for these curves).
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        if bez(x1, x2, mid) < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    bez(y1, y2, (lo + hi) / 2.0)
}

/// Animated values, by widget id: each moves toward its target at a fixed speed and is read eased.
#[derive(Default)]
pub struct Anims {
    values: HashMap<u64, f32>,
}

impl Anims {
    /// The eased progress (0..=1) of `id` toward `on`, after `dt` seconds more of a `duration` transition.
    pub fn get(&mut self, id: u64, on: bool, dt: f32, duration: f32) -> f32 {
        let target = if on { 1.0 } else { 0.0 };
        let v = self.values.entry(id).or_insert(target);
        let step = if duration > 0.0 { dt / duration } else { 1.0 };
        *v = if *v < target {
            (*v + step).min(target)
        } else {
            (*v - step).max(target)
        };
        fast_out_slow_in(*v)
    }

    /// Like [`get`](Self::get), starting from `from` the first time `id` is seen.
    pub fn get_from(&mut self, id: u64, from: bool, on: bool, dt: f32, duration: f32) -> f32 {
        self.values
            .entry(id)
            .or_insert(if from { 1.0 } else { 0.0 });
        self.get(id, on, dt, duration)
    }
}

/// A widget id from a label and a number.
pub fn id(label: &str, n: usize) -> u64 {
    // FNV-1a: stable across runs, no allocation.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in label.bytes().chain(n.to_le_bytes()) {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// This frame's input, in the painter's units.
#[derive(Default)]
pub struct Frame {
    pub mouse: [f32; 2],
    /// A left press this frame not yet taken by a widget.
    pub press: bool,
    /// A right press this frame not yet taken.
    pub press_right: bool,
    pub release: bool,
    /// The left button is held.
    pub down: bool,
    pub scroll: f32,
    pub keys: Vec<(String, u32)>,
    pub chars: Vec<char>,
    /// The widget the held button belongs to (a slider being dragged).
    pub active: Option<u64>,
    /// Seconds since the last frame.
    pub dt: f32,
}

impl Frame {
    /// Folds the queued input in; `mouse` is already in units.
    pub fn begin(&mut self, queue: &mut Vec<Input>, mouse: [f32; 2], dt: f32) {
        self.mouse = mouse;
        self.dt = dt.clamp(0.0, 0.1);
        self.press = false;
        self.press_right = false;
        self.release = false;
        self.scroll = 0.0;
        self.keys.clear();
        self.chars.clear();
        for input in queue.drain(..) {
            match input {
                Input::Press(0) => {
                    self.press = true;
                    self.down = true;
                }
                Input::Press(1) => self.press_right = true,
                Input::Release(0) => {
                    self.release = true;
                    self.down = false;
                }
                Input::Scroll(s) => self.scroll += s[1],
                Input::Key(k, m) => self.keys.push((k, m)),
                Input::Char(c) => self.chars.push(c),
                _ => {}
            }
        }
    }

    /// Drops the active widget once the button is up (after the frame's widgets saw the release).
    pub fn end(&mut self) {
        if !self.down {
            self.active = None;
        }
    }

    pub fn hover(&self, r: [f32; 4]) -> bool {
        inside(r, self.mouse)
    }

    /// A left press inside `r` this frame; taken, so widgets under it don't also see it.
    pub fn click(&mut self, r: [f32; 4]) -> bool {
        if self.press && inside(r, self.mouse) {
            self.press = false;
            true
        } else {
            false
        }
    }

    /// A right press inside `r` this frame; taken.
    pub fn click_right(&mut self, r: [f32; 4]) -> bool {
        if self.press_right && inside(r, self.mouse) {
            self.press_right = false;
            true
        } else {
            false
        }
    }

    /// A press inside `r` makes widget `id` active (it follows the mouse until release); true while it is.
    pub fn drag(&mut self, id: u64, r: [f32; 4]) -> bool {
        if self.click(r) {
            self.active = Some(id);
        }
        self.active == Some(id)
    }
}

pub fn inside(r: [f32; 4], p: [f32; 2]) -> bool {
    p[0] >= r[0] && p[0] < r[0] + r[2] && p[1] >= r[1] && p[1] < r[1] + r[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easings_run_from_zero_to_one() {
        for f in [fast_out_slow_in, ease_out_expo, ease_out_cubic] {
            assert!(f(0.0).abs() < 1e-3);
            assert!((f(1.0) - 1.0).abs() < 1e-3);
        }
        // FastOutSlowIn is past halfway at half time.
        assert!(fast_out_slow_in(0.5) > 0.7);
    }

    #[test]
    fn an_animation_reaches_its_target_after_its_duration() {
        let mut a = Anims::default();
        assert_eq!(a.get(1, false, 0.0, STANDARD), 0.0);
        let mid = a.get(1, true, 0.075, STANDARD);
        assert!(mid > 0.0 && mid < 1.0);
        assert_eq!(a.get(1, true, 0.1, STANDARD), 1.0);
    }

    #[test]
    fn a_click_is_taken_by_the_first_widget_under_it() {
        let mut f = Frame::default();
        f.begin(&mut vec![Input::Press(0)], [5.0, 5.0], 0.016);
        assert!(f.click([0.0, 0.0, 10.0, 10.0]));
        assert!(!f.click([0.0, 0.0, 10.0, 10.0]));
        assert!(f.down);
        f.begin(&mut vec![Input::Release(0)], [5.0, 5.0], 0.016);
        assert!(f.release && !f.down);
    }

    #[test]
    fn ids_are_stable_and_distinct() {
        assert_eq!(id("switch", 3), id("switch", 3));
        assert_ne!(id("switch", 3), id("switch", 4));
    }
}
