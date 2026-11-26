//! Armour HUD (EvergreenHUD, VanillaHUD): the worn armour and optionally the held item, each with its
//! durability, in a column.

use crate::text;
use mod_api::{ClientHost, DrawContext, ItemStack};
use pvp_core::{Anchor, Category, Cx, Events, HudElement, HudPos, Module, Setting};

pub struct Armour;
pub static ARMOUR: Armour = Armour;

const DURABILITY: usize = 3;
const HAND: usize = 4;

static SETTINGS: [Setting; 5] = [
    text::BACKGROUND,
    text::COLOR,
    text::SHADOW,
    Setting::choice(
        "durability",
        "pvp.armour.durability",
        &["remaining", "percent", "none"],
        0,
    ),
    Setting::toggle("hand", "pvp.armour.hand", true),
];

/// One item's row: 16 px icon, the text beside it.
const ROW: f32 = 16.0;
const ROW_GAP: f32 = 2.0;
const TEXT_GAP: f32 = 4.0;

/// The durability text of a stack that wears: `remaining` of `max`, as chosen.
fn durability(mode: usize, damage: u32, max: u32) -> Option<String> {
    if max == 0 {
        return None;
    }
    let left = max.saturating_sub(damage);
    match mode {
        0 => Some(left.to_string()),
        1 => Some(format!("{}%", (left * 100 + max / 2) / max)),
        _ => None,
    }
}

/// The stacks shown, top to bottom.
fn stacks(cx: &Cx, host: &dyn ClientHost) -> Vec<ItemStack> {
    use mod_api::EquipmentSlot as S;
    let Some(me) = host.local_player() else {
        return Vec::new();
    };
    let Some(living) = host.living(me.entity()) else {
        return Vec::new();
    };
    let mut slots = vec![S::Head, S::Chest, S::Legs, S::Feet];
    if cx.bool(HAND) {
        slots.push(S::MainHand);
    }
    slots
        .into_iter()
        .filter_map(|s| living.equipment(host, s))
        .collect()
}

fn label(cx: &Cx, host: &dyn ClientHost, stack: &ItemStack) -> String {
    let max = host.max_damage(stack.item()).unwrap_or(0);
    durability(cx.choice(DURABILITY), stack.damage(), max).unwrap_or_else(|| {
        if stack.count() > 1 {
            stack.count().to_string()
        } else {
            String::new()
        }
    })
}

struct Column;
static HUD: [&dyn HudElement; 1] = [&Column];

impl HudElement for Column {
    fn id(&self) -> &'static str {
        "column"
    }
    fn label(&self) -> &'static str {
        "pvp.armour.column"
    }
    fn default_pos(&self) -> HudPos {
        HudPos::new(Anchor::BottomRight, -4.0, -4.0)
    }
    fn size(&self, cx: &Cx, host: &dyn ClientHost, draw: &dyn DrawContext) -> [f32; 2] {
        let mut rows = stacks(cx, host).len();
        if rows == 0 && cx.editing {
            rows = 4;
        }
        if rows == 0 {
            return [0.0, 0.0];
        }
        let text_w = if cx.choice(DURABILITY) == 2 {
            draw.text_width("64")
        } else {
            draw.text_width("2031")
        };
        let n = rows as f32;
        [
            ROW + TEXT_GAP + text_w + 2.0 * text::PAD,
            n * ROW + (n - 1.0) * ROW_GAP + 2.0 * text::PAD,
        ]
    }
    fn draw(&self, cx: &Cx, host: &dyn ClientHost, draw: &mut dyn DrawContext, at: [f32; 2]) {
        let stacks = stacks(cx, host);
        let size = self.size(cx, host, draw);
        if size[0] == 0.0 {
            return;
        }
        if cx.bool(text::I_BACKGROUND) {
            draw.fill([at[0], at[1], size[0], size[1]], 0x8000_0000);
        }
        if stacks.is_empty() {
            // The editor's placeholder.
            draw.text(
                "Armour",
                [at[0] + text::PAD, at[1] + text::PAD],
                cx.color(text::I_COLOR),
                cx.bool(text::I_SHADOW),
            );
            return;
        }
        for (i, stack) in stacks.iter().enumerate() {
            let y = at[1] + text::PAD + i as f32 * (ROW + ROW_GAP);
            draw.item_stack(stack, [at[0] + text::PAD, y], ROW);
            let s = label(cx, host, stack);
            draw.text(
                &s,
                [at[0] + text::PAD + ROW + TEXT_GAP, y + 4.0],
                cx.color(text::I_COLOR),
                cx.bool(text::I_SHADOW),
            );
        }
    }
}

impl Module for Armour {
    fn id(&self) -> &'static str {
        "armour"
    }
    fn name(&self) -> &'static str {
        "Armour"
    }
    fn description(&self) -> &'static str {
        "pvp.armour.description"
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
    fn durability_modes() {
        assert_eq!(durability(0, 100, 363).as_deref(), Some("263"));
        assert_eq!(durability(1, 100, 363).as_deref(), Some("72%"));
        assert_eq!(durability(2, 100, 363), None);
        assert_eq!(durability(0, 0, 0), None, "an item that does not wear");
    }
}
