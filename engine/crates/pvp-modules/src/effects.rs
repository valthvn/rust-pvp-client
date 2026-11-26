//! Potion effects HUD (EvergreenHUD, VanillaHUD): each active effect with its level and time left.

use crate::text;
use mod_api::{ClientHost, DrawContext};
use pvp_core::{Anchor, Category, Cx, Events, HudElement, HudPos, Module, Setting};

pub struct Effects;
pub static EFFECTS: Effects = Effects;

const HIDE_AMBIENT: usize = 3;

static SETTINGS: [Setting; 4] = [
    text::BACKGROUND,
    text::COLOR,
    text::SHADOW,
    Setting::toggle("hide_ambient", "pvp.effects.hide_ambient", true),
];

/// `minecraft:speed` → `effect.minecraft.speed`, the game's translation key.
fn translation_key(id: &str) -> String {
    let (ns, path) = id.split_once(':').unwrap_or(("minecraft", id));
    format!("effect.{ns}.{path}")
}

fn lines(cx: &Cx, host: &dyn ClientHost) -> Vec<String> {
    let Some(me) = host.local_player() else {
        return Vec::new();
    };
    let Some(living) = host.living(me.entity()) else {
        return Vec::new();
    };
    let effects = living.effects(host).unwrap_or_default();
    effects
        .iter()
        .filter(|e| !(cx.bool(HIDE_AMBIENT) && e.ambient))
        .map(|e| {
            let id = e.id.id(host);
            let name = host.translate(&translation_key(&id)).unwrap_or(id);
            let level = if e.amplifier > 0 {
                format!(" {}", text::roman(e.amplifier + 1))
            } else {
                String::new()
            };
            format!("{name}{level} {}", text::ticks_to_clock(e.duration))
        })
        .collect()
}

fn shown(cx: &Cx, host: &dyn ClientHost) -> Vec<String> {
    let l = lines(cx, host);
    if l.is_empty() && cx.editing {
        vec!["Speed II 1:23".to_owned(), "Strength 0:45".to_owned()]
    } else {
        l
    }
}

struct List;
static HUD: [&dyn HudElement; 1] = [&List];

impl HudElement for List {
    fn id(&self) -> &'static str {
        "list"
    }
    fn label(&self) -> &'static str {
        "pvp.effects.list"
    }
    fn default_pos(&self) -> HudPos {
        HudPos::new(Anchor::Left, 4.0, 0.0)
    }
    fn size(&self, cx: &Cx, host: &dyn ClientHost, draw: &dyn DrawContext) -> [f32; 2] {
        text::size(draw, &shown(cx, host))
    }
    fn draw(&self, cx: &Cx, host: &dyn ClientHost, draw: &mut dyn DrawContext, at: [f32; 2]) {
        let l = shown(cx, host);
        let size = text::size(draw, &l);
        text::draw(cx, draw, at, size, &l);
    }
}

impl Module for Effects {
    fn id(&self) -> &'static str {
        "effects"
    }
    fn name(&self) -> &'static str {
        "Potion Effects"
    }
    fn description(&self) -> &'static str {
        "pvp.effects.description"
    }
    fn category(&self) -> Category {
        Category::Hud
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::NONE
    }
    fn hud(&self) -> &'static [&'static dyn HudElement] {
        &HUD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_ids_map_to_translation_keys() {
        assert_eq!(translation_key("minecraft:speed"), "effect.minecraft.speed");
        assert_eq!(translation_key("haste"), "effect.minecraft.haste");
    }
}
