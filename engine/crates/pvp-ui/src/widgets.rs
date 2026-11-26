//! OneConfig's controls, drawn to its measurements (`research/oneconfig/ui-spec.md` §6.5) in dp.

use crate::ctx::{self, Anims, Frame, STANDARD};
use crate::font::Style;
use crate::gfx::Painter;
use crate::theme::{self as t, alpha, over, with_alpha};

/// What one frame's widgets draw with.
pub struct W<'p, 'a> {
    pub p: &'p mut Painter<'a>,
    pub f: &'p mut Frame,
    pub a: &'p mut Anims,
}

impl W<'_, '_> {
    fn anim(&mut self, id: u64, on: bool) -> f32 {
        let dt = self.f.dt;
        self.a.get(id, on, dt, STANDARD)
    }

    /// The hover/pressed underlay every interactive surface has (§1.4).
    fn underlay(&mut self, id: u64, r: [f32; 4], radius: f32) -> bool {
        let hover = self.f.hover(r);
        let h = self.anim(id ^ 0x55, hover);
        let pressed = hover && self.f.down;
        let a = if pressed {
            t::PRESSED_ALPHA
        } else {
            t::HOVER_ALPHA * h
        };
        self.p.rounded(r, radius, with_alpha(t::TEXT, a));
        hover
    }

    /// Switch, 42 x 21 dp with its top left at `at`; true when clicked (§6.5.1).
    pub fn switch(&mut self, id: u64, at: [f32; 2], on: bool) -> bool {
        let r = [at[0], at[1], 42.0, 21.0];
        let hover = self.f.hover(r);
        let clicked = self.f.click(r);
        let on_now = on != clicked;
        let k = self.anim(id, on_now);
        self.p.rounded(r, 10.5, t::TRACK);
        self.p.rounded(r, 10.5, alpha(t::ACCENT, k));
        if hover {
            self.p.border(r, 10.5, 1.0, with_alpha(t::TEXT, 0.15));
        }
        self.p
            .circle([at[0] + 3.0 + 21.0 * k, at[1] + 3.0], 15.0, t::THUMB);
        clicked
    }

    /// Checkbox, 24 dp (§6.5.2).
    pub fn checkbox(&mut self, id: u64, at: [f32; 2], on: bool) -> bool {
        let r = [at[0], at[1], 24.0, 24.0];
        let hover = self.f.hover(r);
        let clicked = self.f.click(r);
        let k = self.anim(id, on != clicked);
        self.p.rounded(r, t::R_SMALL, t::COMPONENT);
        self.p.rounded(r, t::R_SMALL, alpha(t::ACCENT, k));
        let b = if k > 0.5 {
            t::ACCENT
        } else if hover {
            t::TEXT_2
        } else {
            t::BORDER
        };
        self.p.border(r, t::R_SMALL, 1.5, b);
        self.p.icon(
            "tick",
            [at[0] - 1.0, at[1] - 1.0, 26.0, 26.0],
            alpha(t::TEXT, k),
        );
        clicked
    }

    /// Slider in `[x, y, w]` (its 19 dp thumb's row), value in `min..=max`; the new value while dragged
    /// (§6.5.3).
    pub fn slider(
        &mut self,
        id: u64,
        x: f32,
        y: f32,
        w: f32,
        v: f32,
        min: f32,
        max: f32,
    ) -> Option<f32> {
        let thumb = 19.0;
        let track_h = 5.0;
        let r = [x, y, w, thumb];
        let active = self.f.drag(id, r);
        let frac = if max > min {
            ((v - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let track = [x, y + (thumb - track_h) / 2.0, w, track_h];
        self.p.rounded(track, t::R_SMALL, t::TRACK);
        self.p.rounded(
            [track[0], track[1], frac * w, track_h],
            t::R_SMALL,
            t::ACCENT,
        );
        self.p.circle([x + frac * (w - thumb), y], thumb, t::THUMB);
        if active {
            let f = ((self.f.mouse[0] - x - thumb / 2.0) / (w - thumb)).clamp(0.0, 1.0);
            return Some(min + f * (max - min));
        }
        None
    }

    /// The 87 dp number box beside a slider, showing `text` (§6.5.3).
    pub fn number_box(&mut self, at: [f32; 2], text: &str) {
        let r = [at[0], at[1], 87.0, t::CONTROL_H];
        self.p.rounded(r, t::R_FIELD, t::COMPONENT);
        let b = if self.f.hover(r) {
            t::TEXT_2
        } else {
            t::BORDER
        };
        self.p.border(r, t::R_FIELD, 1.0, b);
        let st = Style::new(crate::font::Weight::Regular, 13.0);
        self.p.text_mid(text, r[0] + 8.0, r[1], r[3], st, t::TEXT);
    }

    /// Select field (min 160 dp) showing `text`; true when clicked to open its menu (§6.5.4).
    pub fn select(&mut self, id: u64, r: [f32; 4], text: &str, open: bool) -> bool {
        let hover = self.f.hover(r);
        let clicked = self.f.click(r);
        let k = self.anim(id, open);
        self.p.rounded(
            r,
            t::R_FIELD,
            over(with_alpha(t::ACCENT, 0.2 * k), t::COMPONENT),
        );
        let b = if hover { t::TEXT_2 } else { t::BORDER };
        self.p.border(r, t::R_FIELD, 1.0, b);
        let c = if hover || open { t::TEXT } else { t::TEXT_2 };
        let label = self.p.ellipsize(text, t::BODY, r[2] - 12.0 - 28.0);
        self.p.text_mid(&label, r[0] + 12.0, r[1], r[3], t::BODY, c);
        // The chevron points down closed, up open.
        let icon = if open { "up" } else { "down" };
        self.p.icon(
            icon,
            [
                r[0] + r[2] - 12.0 - 16.0,
                r[1] + (r[3] - 16.0) / 2.0,
                16.0,
                16.0,
            ],
            c,
        );
        clicked
    }

    /// Keybind field: `label` (the key's name, or the recording prompt); true when clicked (§6.5.6).
    pub fn keybind(&mut self, id: u64, r: [f32; 4], label: &str, recording: bool) -> bool {
        let hover = self.f.hover(r);
        let clicked = self.f.click(r);
        let k = self.anim(id, recording);
        let fill = if hover && !recording {
            alpha(t::COMPONENT, 0.8)
        } else {
            t::COMPONENT
        };
        self.p
            .rounded(r, t::R_FIELD, over(with_alpha(t::ACCENT, 0.2 * k), fill));
        let b = if recording {
            t::ACCENT
        } else if hover {
            with_alpha(t::TEXT, 0.2)
        } else {
            t::BORDER
        };
        self.p.border(r, t::R_FIELD, 1.0, b);
        let c = if recording { t::ACCENT } else { t::TEXT };
        self.p.icon(
            "keyboard",
            [r[0] + 12.0, r[1] + (r[3] - 16.0) / 2.0, 16.0, 16.0],
            c,
        );
        let st = Style::new(crate::font::Weight::Regular, 13.0);
        self.p
            .text_mid(label, r[0] + 12.0 + 16.0 + 8.0, r[1], r[3], st, c);
        clicked
    }

    /// Text field showing `text` (a caret when focused); true when clicked (§6.5.7).
    pub fn text_field(
        &mut self,
        r: [f32; 4],
        text: &str,
        placeholder: &str,
        focused: bool,
    ) -> bool {
        let hover = self.f.hover(r);
        let clicked = self.f.click(r);
        self.p.rounded(r, t::R_FIELD, t::CARD);
        let b = if focused {
            t::ACCENT
        } else if hover {
            t::TEXT_2
        } else {
            t::BORDER
        };
        self.p.border(r, t::R_FIELD, 1.0, b);
        let x = r[0] + 12.0;
        if text.is_empty() && !focused {
            self.p
                .text_mid(placeholder, x, r[1], r[3], t::BODY, t::TEXT_2);
        } else {
            self.p.text_mid(text, x, r[1], r[3], t::BODY, t::TEXT);
        }
        if focused && (self.p.ui.now * 2.0).fract() < 0.5 {
            let cx = x + self.p.text_width(text, t::BODY) + 1.0;
            self.p.fill([cx, r[1] + 8.0, 1.0, r[3] - 16.0], t::ACCENT);
        }
        clicked
    }

    /// The search field of the header (256 x 32 dp, chip fill, search icon); true when clicked (§4.3).
    pub fn search(&mut self, r: [f32; 4], text: &str, focused: bool) -> bool {
        let hover = self.f.hover(r);
        let clicked = self.f.click(r);
        self.p.rounded(r, t::R_FIELD, t::CHIP);
        let b = if focused {
            t::ACCENT
        } else if hover {
            t::TEXT_2
        } else {
            t::BORDER
        };
        self.p.border(r, t::R_FIELD, 1.0, b);
        let ic = if hover || focused { t::TEXT } else { t::TEXT_2 };
        self.p.icon(
            "search",
            [r[0] + 12.0, r[1] + (r[3] - 16.0) / 2.0, 16.0, 16.0],
            ic,
        );
        let x = r[0] + 12.0 + 16.0 + 10.0;
        if text.is_empty() {
            self.p
                .text_mid("Search...", x, r[1], r[3], t::BODY, t::TEXT_2);
        } else {
            self.p.text_mid(text, x, r[1], r[3], t::BODY, t::TEXT);
        }
        if focused && (self.p.ui.now * 2.0).fract() < 0.5 {
            let cx = x + self.p.text_width(text, t::BODY) + 1.0;
            self.p.fill([cx, r[1] + 8.0, 1.0, r[3] - 16.0], t::ACCENT);
        }
        clicked
    }

    /// A ghost icon button (32 dp by default); true when clicked (§1.4).
    pub fn icon_button(&mut self, id: u64, r: [f32; 4], icon: &str, icon_size: f32) -> bool {
        let hover = self.underlay(id, r, t::R_BUTTON);
        if hover {
            self.p.border(r, t::R_BUTTON, 1.0, t::BORDER);
        }
        let c = if hover { t::TEXT } else { t::TEXT_2 };
        let o = (r[2] - icon_size) / 2.0;
        self.p.icon(
            icon,
            [
                r[0] + o,
                r[1] + (r[3] - icon_size) / 2.0,
                icon_size,
                icon_size,
            ],
            c,
        );
        self.f.click(r)
    }

    /// A text button; `primary` is the solid accent one, else the secondary one (§6.5.8).
    pub fn button(&mut self, id: u64, r: [f32; 4], label: &str, primary: bool) -> bool {
        let hover = self.f.hover(r);
        let k = self.anim(id, hover);
        if primary {
            self.p
                .rounded(r, t::R_BUTTON, alpha(t::ACCENT, 1.0 - 0.25 * k));
        } else {
            self.p.rounded(r, t::R_BUTTON, t::COMPONENT);
            let b = if hover { t::TEXT_2 } else { t::BORDER };
            self.p.border(r, t::R_BUTTON, 1.0, b);
        }
        let st = Style::new(crate::font::Weight::Regular, 14.0);
        let w = self.p.text_width(label, st);
        let c = if primary { t::ON_ACCENT } else { t::TEXT };
        self.p
            .text_mid(label, r[0] + (r[2] - w) / 2.0, r[1], r[3], st, c);
        self.f.click(r)
    }

    /// A category chip (32 dp tall), sized to its label; returns its width and whether it was clicked
    /// (§5.1, `Chips.kt`).
    pub fn chip(
        &mut self,
        id: u64,
        at: [f32; 2],
        label: &str,
        icon: Option<&str>,
        selected: bool,
    ) -> (f32, bool) {
        let st = t::BODY;
        let icon_w = if icon.is_some() { 16.0 + 8.0 } else { 0.0 };
        let w = 16.0 + icon_w + self.p.text_width(label, st) + 16.0;
        let r = [at[0], at[1], w, t::CONTROL_H];
        let hover = self.f.hover(r);
        let h = self.anim(id ^ 0x77, hover);
        let s = self.anim(id, selected);
        let fill = over(with_alpha(t::ACCENT, (0.5 * h).max(s)), t::CHIP);
        self.p.rounded(r, t::R_FIELD, fill);
        self.p.border(r, t::R_FIELD, 1.0, t::BORDER);
        let c = if s > 0.5 { t::ON_ACCENT } else { t::TEXT };
        let mut x = r[0] + 16.0;
        if let Some(i) = icon {
            self.p.icon(i, [x, r[1] + 8.0, 16.0, 16.0], c);
            x += icon_w;
        }
        self.p.text_mid(label, x, r[1], r[3], st, c);
        (w, self.f.click(r))
    }

    /// A sidebar entry (225 x 33 dp): solid accent when selected (§4.4, `NavItem.kt`).
    pub fn nav(&mut self, id: u64, at: [f32; 2], icon: &str, label: &str, selected: bool) -> bool {
        let r = [at[0], at[1], t::NAV_W, t::NAV_H];
        let hover = self.f.hover(r);
        let s = self.anim(id, selected);
        if s < 1.0 {
            self.underlay(id, r, t::R_FIELD);
        }
        self.p.rounded(r, t::R_FIELD, alpha(t::ACCENT, s));
        if hover || s > 0.0 {
            self.p.border(r, t::R_FIELD, 1.0, t::BORDER);
        }
        let rest = if hover { t::TEXT } else { alpha(t::TEXT, 0.8) };
        let c = t::lerp(rest, t::ON_ACCENT, s);
        self.p.icon(
            icon,
            [r[0] + 12.0, r[1] + (r[3] - 18.0) / 2.0, 18.0, 18.0],
            c,
        );
        self.p
            .text_mid(label, r[0] + 12.0 + 18.0 + 16.0, r[1], r[3], t::BODY, c);
        self.f.click(r)
    }
}

/// A key's name as the menu shows it: `key.keyboard.right.shift` → `Right Shift`; empty → `None`.
pub fn key_name(key: &str) -> String {
    let Some(rest) = key
        .strip_prefix("key.keyboard.")
        .or_else(|| key.strip_prefix("key.mouse."))
    else {
        return if key.is_empty() {
            "None".to_owned()
        } else {
            key.to_owned()
        };
    };
    let mouse = key.starts_with("key.mouse.");
    let words: Vec<String> = rest
        .split('.')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().chain(c).collect(),
                None => String::new(),
            }
        })
        .collect();
    let name = words.join(" ");
    if mouse { format!("Mouse {name}") } else { name }
}

/// Hit test helper re-exported for pages.
pub use ctx::inside;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_names_read_like_the_game_shows_them() {
        assert_eq!(key_name("key.keyboard.right.shift"), "Right Shift");
        assert_eq!(key_name("key.keyboard.r"), "R");
        assert_eq!(key_name("key.mouse.left"), "Mouse Left");
        assert_eq!(key_name(""), "None");
    }
}
