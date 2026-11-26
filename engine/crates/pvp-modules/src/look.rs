//! Render colours the game draws with each frame: the enchantment glint (GlintColorizer) and the hurt tint
//! (DamageTint). The game has no mod hook for them; the client executable installs Horizon's frame override
//! and asks [`colours`] (see `pvp-client`). Both modules are plain settings: no event of their own.

use pvp_core::{Category, Module, Setting, registry};
use std::sync::OnceLock;

/// What the frame should use instead of the game's own, `None` keeping it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Colours {
    /// The glint's `0xAARRGGBB` colour.
    pub glint: Option<u32>,
    /// The hurt tint's `0xAARRGGBB` colour and how much of it covers the entity, in percent.
    pub hurt: Option<(u32, u8)>,
}

pub struct Glint;
pub static GLINT: Glint = Glint;
static GLINT_SETTINGS: [Setting; 1] = [Setting::color("color", "pvp.glint.color", 0xFFFF_5555)];

impl Module for Glint {
    fn id(&self) -> &'static str {
        "glint"
    }
    fn name(&self) -> &'static str {
        "Glint Colour"
    }
    fn description(&self) -> &'static str {
        "pvp.glint.description"
    }
    fn category(&self) -> Category {
        Category::Visual
    }
    fn settings(&self) -> &'static [Setting] {
        &GLINT_SETTINGS
    }
}

pub struct DamageTint;
pub static DAMAGE_TINT: DamageTint = DamageTint;
static TINT_SETTINGS: [Setting; 2] = [
    Setting::color("color", "pvp.damage_tint.color", 0xFF55_55FF),
    Setting::slider("opacity", "pvp.damage_tint.opacity", 0.0, 100.0, 1.0, 30.0),
];

impl Module for DamageTint {
    fn id(&self) -> &'static str {
        "damage_tint"
    }
    fn name(&self) -> &'static str {
        "Damage Tint"
    }
    fn description(&self) -> &'static str {
        "pvp.damage_tint.description"
    }
    fn category(&self) -> Category {
        Category::Pvp
    }
    fn settings(&self) -> &'static [Setting] {
        &TINT_SETTINGS
    }
}

/// The registry indices of the two modules, looked up once.
fn indices() -> Option<(usize, usize)> {
    static I: OnceLock<Option<(usize, usize)>> = OnceLock::new();
    *I.get_or_init(|| {
        let reg = registry::get()?;
        Some((reg.index_of("glint")?, reg.index_of("damage_tint")?))
    })
}

/// The colours of the modules that are on. Cheap: two atomic loads with both off.
pub fn colours() -> Colours {
    let (Some(reg), Some((g, t))) = (registry::get(), indices()) else {
        return Colours::default();
    };
    let mut out = Colours::default();
    if reg.is_enabled(g) {
        out.glint = Some(reg.value(g, 0).as_color());
    }
    if reg.is_enabled(t) {
        let opacity = reg.value(t, 1).as_f32().clamp(0.0, 100.0) as u8;
        out.hurt = Some((reg.value(t, 0).as_color(), opacity));
    }
    out
}

/// `0xAARRGGBB` as Horizon's glint shader reads it: RGB, opaque, in little-endian bytes.
pub fn glint_texel(argb: u32) -> u32 {
    let [_, r, g, b] = argb.to_be_bytes();
    u32::from_le_bytes([r, g, b, 255])
}

/// Horizon's hurt overlay texel: RGB, and as alpha what of the entity's own colour stays.
pub fn hurt_texel(argb: u32, opacity: u8) -> u32 {
    let [_, r, g, b] = argb.to_be_bytes();
    let cover = (u32::from(opacity.min(100)) * 255 + 50) / 100;
    u32::from_le_bytes([r, g, b, (255 - cover) as u8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texels_match_horizons_encoding() {
        assert_eq!(
            glint_texel(0xFF11_2233),
            u32::from_le_bytes([0x11, 0x22, 0x33, 255])
        );
        // 30 % of red: 77 of 255 covers, 178 of the entity stays (Horizon's `Hit::texel`).
        assert_eq!(
            hurt_texel(0xFFFF_0000, 30),
            u32::from_le_bytes([255, 0, 0, 178])
        );
    }
}
