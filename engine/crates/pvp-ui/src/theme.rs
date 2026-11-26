//! OneConfig's default theme, "Polyfrosted Dark" (Flat style), and its type scale, radii and spacing. Every
//! value comes from `research/oneconfig/ui-spec.md` (§1-§3), which cites OneConfig's sources line by line.
//! Units are OneConfig's dp/sp; the shell maps them to screen pixels (`shell::Layout`).

use crate::font::{Style, Weight};

// Palette (ui-spec §1.2; OneConfig `internal/ui/themes/Impls.kt:84-98`, `ThemeConfig.java:51`).
pub const PAGE: u32 = 0xC411_171C;
pub const SIDEBAR: u32 = 0xB315_1C22;
pub const CHIP: u32 = 0xB223_2D32;
pub const CARD: u32 = 0x5923_2D32;
pub const COMPONENT: u32 = 0xFF1A_2229;
pub const POPUP: u32 = 0xFF1A_2229;
pub const BORDER: u32 = 0x1AFF_FFFF;
pub const TEXT: u32 = 0xFFD5_DBFF;
pub const TEXT_2: u32 = 0xFF75_7883;
pub const ON_ACCENT: u32 = 0xFFFF_FFFF;
pub const THUMB: u32 = 0xFFFF_FFFF;
pub const TRACK: u32 = 0xFF74_777F;
pub const FAVORITE: u32 = 0xFFFF_D700;
pub const ACCENT: u32 = 0xFF2B_4BFF;
pub const ERROR: u32 = 0xFFE3_5B5B;
pub const WARNING: u32 = 0xFFFF_AB1D;
pub const SUCCESS: u32 = 0xFF3C_AF77;
pub const SCRIM: u32 = 0x8000_0000;
/// Hover and pressed overlays: `TEXT` at these alphas (§1.2).
pub const HOVER_ALPHA: f32 = 0.08;
pub const PRESSED_ALPHA: f32 = 0.14;
pub const DISABLED_ALPHA: f32 = 0.5;

// Type scale (§2.2; `api/ui/v1/theme/ThemeTokens.kt:45-62`).
pub const TITLE: Style = Style::new(Weight::SemiBold, 18.0);
pub const HEADING: Style = Style::new(Weight::SemiBold, 15.0);
pub const HEADING_MEDIUM: Style = Style::new(Weight::Medium, 15.0);
pub const BODY: Style = Style::new(Weight::Regular, 14.0);
pub const BODY_MEDIUM: Style = Style::new(Weight::Medium, 14.0);
pub const LABEL: Style = Style::new(Weight::Medium, 13.0);
pub const LABEL_REGULAR: Style = Style::new(Weight::Regular, 13.0);
pub const SMALL: Style = Style::new(Weight::Regular, 12.0);
pub const CAPTION: Style = Style::new(Weight::Regular, 11.0);
pub const EYEBROW: Style = Style::new(Weight::Medium, 11.0);
pub const TITLE_SMALL: Style = Style::new(Weight::Medium, 16.0);
pub const TITLE_LARGE: Style = Style::new(Weight::Medium, 22.0);
/// The header's page title: `display` (24 sp) turned Medium (`Header.kt:217`).
pub const DISPLAY: Style = Style::new(Weight::Medium, 24.0);

// Radii (§3.1).
pub const R_SMALL: f32 = 6.0;
pub const R_BUTTON: f32 = 8.0;
pub const R_FIELD: f32 = 8.0;
pub const R_CARD: f32 = 16.0;
pub const R_POPUP: f32 = 16.0;
pub const R_WINDOW: f32 = 20.0;

// Shell geometry (§4).
pub const WINDOW_W: f32 = 1391.0;
pub const WINDOW_H: f32 = 700.0;
pub const SIDEBAR_W: f32 = 264.0;
pub const GUTTER: f32 = 25.0;
pub const PAGE_PAD_Y: f32 = 19.0;
pub const HEADER_H: f32 = 36.0;
pub const NAV_W: f32 = 225.0;
pub const NAV_H: f32 = 33.0;
pub const NAV_GAP: f32 = 8.0;
pub const SECTION_GAP: f32 = 18.0;
pub const CONTROL_H: f32 = 32.0;
pub const OPTION_W: f32 = 300.0;

// Mod (module) cards (§5.2-5.3).
pub const CARD_W: f32 = 255.0;
pub const CARD_H: f32 = 140.0;
pub const CARD_GAP: f32 = 19.0;
pub const CARD_COLUMNS: usize = 4;
pub const CARD_FOOTER_H: f32 = 36.0;

/// `argb` with its alpha multiplied by `a`.
pub fn alpha(argb: u32, a: f32) -> u32 {
    let base = (argb >> 24) as f32 / 255.0;
    let na = (base * a.clamp(0.0, 1.0) * 255.0).round() as u32;
    (na << 24) | (argb & 0x00FF_FFFF)
}

/// `argb` at alpha `a` (replacing its own).
pub fn with_alpha(argb: u32, a: f32) -> u32 {
    (((a.clamp(0.0, 1.0) * 255.0).round() as u32) << 24) | (argb & 0x00FF_FFFF)
}

/// Linear blend of two ARGB colours, channel by channel (`t` 0 = `a`, 1 = `b`).
pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let ch = |s: u32| {
        let x = ((a >> s) & 0xFF) as f32;
        let y = ((b >> s) & 0xFF) as f32;
        ((x + (y - x) * t).round() as u32) << s
    };
    ch(24) | ch(16) | ch(8) | ch(0)
}

/// `top` composited over opaque-or-not `bottom` (source-over), the result's alpha that of the pair.
pub fn over(top: u32, bottom: u32) -> u32 {
    let ta = (top >> 24) as f32 / 255.0;
    let ba = (bottom >> 24) as f32 / 255.0;
    let oa = ta + ba * (1.0 - ta);
    if oa <= 0.0 {
        return 0;
    }
    let ch = |s: u32| {
        let t = ((top >> s) & 0xFF) as f32;
        let b = ((bottom >> s) & 0xFF) as f32;
        (((t * ta + b * ba * (1.0 - ta)) / oa).round() as u32) << s
    };
    (((oa * 255.0).round() as u32) << 24) | ch(16) | ch(8) | ch(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colour_helpers() {
        assert_eq!(alpha(0x80FF_FFFF, 0.5), 0x40FF_FFFF);
        assert_eq!(with_alpha(TEXT, HOVER_ALPHA), 0x14D5_DBFF);
        assert_eq!(lerp(0xFF00_0000, 0xFFFF_FFFF, 0.5), 0xFF80_8080);
        assert_eq!(over(0xFF12_3456, 0xFF00_0000), 0xFF12_3456);
        assert_eq!(over(0x0000_0000, COMPONENT), COMPONENT);
        // The accent at half alpha over the component fill lands between them.
        let mid = over(with_alpha(ACCENT, 0.5), COMPONENT);
        assert_eq!(mid >> 24, 0xFF);
    }
}
