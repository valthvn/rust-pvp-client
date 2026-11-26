//! The look every text HUD element shares (OneConfig's text HUD): a translucent box, a colour, a shadow.
//! A text module lists [`BACKGROUND`], [`COLOR`] and [`SHADOW`] first among its settings, in that order.

use mod_api::DrawContext;
use pvp_core::{Cx, Setting};

/// Padding around the text when the background is on.
pub const PAD: f32 = 4.0;
/// Height of one line of the game's font, and the step between lines.
pub const LINE: f32 = 10.0;
const BOX: u32 = 0x8000_0000;

pub const BACKGROUND: Setting = Setting::toggle("background", "pvp.hud.background", true);
pub const COLOR: Setting = Setting::color("color", "pvp.hud.text_color", 0xFFFF_FFFF);
pub const SHADOW: Setting = Setting::toggle("shadow", "pvp.hud.shadow", true);
/// Indices of the shared settings.
pub const I_BACKGROUND: usize = 0;
pub const I_COLOR: usize = 1;
pub const I_SHADOW: usize = 2;

/// The size of a box holding `lines`, left aligned.
pub fn size(draw: &dyn DrawContext, lines: &[String]) -> [f32; 2] {
    if lines.is_empty() {
        return [0.0, 0.0];
    }
    let w = lines
        .iter()
        .map(|l| draw.text_width(l))
        .fold(0.0_f32, f32::max);
    [w + 2.0 * PAD, lines.len() as f32 * LINE - 1.0 + 2.0 * PAD]
}

/// Draws `lines` in a box of `size` (from [`size`], or wider to keep it steady) at `at`.
pub fn draw(cx: &Cx, draw: &mut dyn DrawContext, at: [f32; 2], size: [f32; 2], lines: &[String]) {
    if lines.is_empty() {
        return;
    }
    if cx.bool(I_BACKGROUND) {
        draw.fill([at[0], at[1], size[0], size[1]], BOX);
    }
    for (i, l) in lines.iter().enumerate() {
        draw.text(
            l,
            [at[0] + PAD, at[1] + PAD + 1.0 + i as f32 * LINE],
            cx.color(I_COLOR),
            cx.bool(I_SHADOW),
        );
    }
}

/// Draws one line centred in a box of `size`.
pub fn draw_centered(cx: &Cx, draw: &mut dyn DrawContext, at: [f32; 2], size: [f32; 2], s: &str) {
    if cx.bool(I_BACKGROUND) {
        draw.fill([at[0], at[1], size[0], size[1]], BOX);
    }
    let tw = draw.text_width(s);
    draw.text(
        s,
        [at[0] + (size[0] - tw) / 2.0, at[1] + PAD + 1.0],
        cx.color(I_COLOR),
        cx.bool(I_SHADOW),
    );
}

/// A one-line text HUD element: `text` gives the line (`None` draws nothing, except a placeholder while the
/// editor is open). `widest` is a sample of the widest line, so the box doesn't jitter as values change.
pub struct Line {
    pub id: &'static str,
    pub label: &'static str,
    pub pos: pvp_core::HudPos,
    pub widest: fn(&Cx) -> String,
    pub text: fn(&Cx, &dyn mod_api::ClientHost) -> Option<String>,
}

impl pvp_core::HudElement for Line {
    fn id(&self) -> &'static str {
        self.id
    }
    fn label(&self) -> &'static str {
        self.label
    }
    fn default_pos(&self) -> pvp_core::HudPos {
        self.pos
    }
    fn size(&self, cx: &Cx, _: &dyn mod_api::ClientHost, draw: &dyn DrawContext) -> [f32; 2] {
        [
            draw.text_width(&(self.widest)(cx)) + 2.0 * PAD,
            LINE - 1.0 + 2.0 * PAD,
        ]
    }
    fn draw(
        &self,
        cx: &Cx,
        host: &dyn mod_api::ClientHost,
        draw: &mut dyn DrawContext,
        at: [f32; 2],
    ) {
        let s = match (self.text)(cx, host) {
            Some(s) => s,
            None if cx.editing => (self.widest)(cx),
            None => return,
        };
        let size = self.size(cx, host, draw);
        draw_centered(cx, draw, at, size, &s);
    }
}

/// `n` in Roman numerals (effect levels, as the game writes them), Arabic past 10.
pub fn roman(n: i32) -> String {
    const R: [&str; 10] = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
    match n {
        1..=10 => R[(n - 1) as usize].to_owned(),
        _ => n.to_string(),
    }
}

/// A tick count as `m:ss`, as the game shows effect durations.
pub fn ticks_to_clock(ticks: i32) -> String {
    if ticks < 0 {
        return "**:**".to_owned();
    }
    let s = ticks / 20;
    format!("{}:{:02}", s / 60, s % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numerals_and_durations() {
        assert_eq!(roman(1), "I");
        assert_eq!(roman(4), "IV");
        assert_eq!(roman(11), "11");
        assert_eq!(ticks_to_clock(20 * 83), "1:23");
        assert_eq!(ticks_to_clock(-1), "**:**");
        assert_eq!(ticks_to_clock(0), "0:00");
    }
}
