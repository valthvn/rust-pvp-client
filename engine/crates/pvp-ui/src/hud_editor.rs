//! The HUD editor (OneConfig's, `research/oneconfig/ui-spec.md` §8): the HUD elements of the modules that are
//! on, drawn where they are, outlined, dragged with snapping to the screen and to each other.
//!
//! Scaling is not offered: the game's GUI drawing has no scale (`DrawContext`), so a HUD element can only be
//! drawn at its own size (`pvp_core::HudPos::scale` is kept in profiles for when it can).

use crate::ctx::Input;
use crate::font::{Style, Weight};
use crate::gfx::Ui;
use crate::theme::{self as t, with_alpha};
use mod_api::{ClientHost, DrawContext};
use pvp_core::hud::{self, Guide};
use pvp_core::{Events, HudPos, Registry, registry};

/// Snap distance in GUI pixels (`HudDesignStudio.kt:313`).
const SNAP: f32 = 6.0;
/// The size badge: 16 px tall, 5 px padding, 10 px text, 9 px above the selection (§8.1-8.2).
const BADGE_H: f32 = 16.0;
const BADGE_TEXT: Style = Style::new(Weight::Medium, 10.0);

#[derive(Default)]
pub struct Editor {
    /// Input since the last frame, each with where the mouse was.
    events: Vec<(Input, [f32; 2])>,
    selected: Option<(usize, usize)>,
    /// The dragged element and where the mouse holds it (from its top left).
    drag: Option<((usize, usize), [f32; 2])>,
    guides: Vec<Guide>,
    /// Alt held: no snapping for this drag.
    free: bool,
}

/// One element on screen: (module, element), its rectangle.
struct Item {
    key: (usize, usize),
    rect: [f32; 4],
}

impl Editor {
    pub fn input(&mut self, input: Input, at: [f32; 2]) {
        if let Input::Key(k, _) = &input
            && (k == "key.keyboard.left.alt" || k == "key.keyboard.right.alt")
        {
            self.free = true;
        }
        self.events.push((input, at));
    }

    pub fn opened(&mut self) {
        self.events.clear();
        self.selected = None;
        self.drag = None;
        self.guides.clear();
    }

    fn items(reg: &Registry, host: &dyn ClientHost, draw: &dyn DrawContext) -> Vec<Item> {
        let screen = draw.size();
        let mut out = Vec::new();
        let mut mask = reg.mask(Events::HUD);
        while mask != 0 {
            let i = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            for (e, el) in reg.modules()[i].hud().iter().enumerate() {
                let pos = reg.hud_pos(i, e);
                let mut size = [0.0, 0.0];
                reg.call(i, true, |cx| size = el.size(cx, host, draw));
                let at = pos.top_left(size, screen);
                out.push(Item {
                    key: (i, e),
                    rect: [at[0], at[1], size[0], size[1]],
                });
            }
        }
        out
    }

    pub fn draw(
        &mut self,
        ui: &mut Ui,
        draw: &mut dyn DrawContext,
        host: &dyn ClientHost,
        mouse: [f32; 2],
    ) {
        let Some(reg) = registry::get() else { return };
        let screen = draw.size();
        let mut items = Self::items(reg, host, &*draw);
        // Input, in the order it came, each event where the mouse was then: a fast drag (several events
        // between two frames) moves the element as far as a slow one.
        for (input, at) in std::mem::take(&mut self.events) {
            match input {
                Input::Press(0) => {
                    self.selected = None;
                    self.free = false;
                    if let Some(it) = items
                        .iter()
                        .rev()
                        .find(|it| crate::ctx::inside(it.rect, at))
                    {
                        self.selected = Some(it.key);
                        self.drag = Some((it.key, [at[0] - it.rect[0], at[1] - it.rect[1]]));
                    }
                }
                Input::Drag(0, _) => self.move_to(reg, &mut items, at, screen),
                Input::Release(0) => {
                    self.move_to(reg, &mut items, at, screen);
                    self.drag = None;
                    self.guides.clear();
                }
                _ => {}
            }
        }
        if self.drag.is_some() {
            self.move_to(reg, &mut items, mouse, screen);
        }
        // The elements themselves, as the HUD draws them.
        for it in &items {
            let el = reg.modules()[it.key.0].hud()[it.key.1];
            reg.call(it.key.0, true, |cx| {
                el.draw(cx, host, draw, [it.rect[0], it.rect[1]])
            });
        }
        let mut p = ui.painter(draw);
        for g in &self.guides {
            match *g {
                Guide::Vertical(x) => p.fill([x.round(), 0.0, 1.0, screen[1]], t::ACCENT),
                Guide::Horizontal(y) => p.fill([0.0, y.round(), screen[0], 1.0], t::ACCENT),
            }
        }
        for it in &items {
            let selected = self.selected == Some(it.key);
            let hover = crate::ctx::inside(it.rect, mouse);
            let c = if selected {
                t::ACCENT
            } else if hover {
                with_alpha(0xFFFF_FFFF, 0.5)
            } else {
                with_alpha(0xFFFF_FFFF, 0.2)
            };
            let r = it.rect;
            p.border(
                [r[0] - 1.0, r[1] - 1.0, r[2] + 2.0, r[3] + 2.0],
                0.0,
                1.0,
                c,
            );
            if self.drag.is_some_and(|(k, _)| k == it.key) {
                p.fill(r, with_alpha(t::ACCENT, 0.10));
            }
            if selected {
                let label = format!("{} × {}", r[2].round(), r[3].round());
                let w = p.text_width(&label, BADGE_TEXT) + 10.0;
                let b = [r[0] + (r[2] - w) / 2.0, r[1] - 9.0 - BADGE_H, w, BADGE_H];
                p.rounded(b, 3.0, t::ACCENT);
                p.text_mid(&label, b[0] + 5.0, b[1], BADGE_H, BADGE_TEXT, t::ON_ACCENT);
            }
        }
        // A hint, top centre.
        let hint = if items.is_empty() {
            "No HUD element: turn a HUD module on in the menu"
        } else {
            "Drag to move · Alt: no snapping · Esc: done"
        };
        let st = Style::new(Weight::Regular, 11.0);
        let w = p.text_width(hint, st) + 16.0;
        let r = [(screen[0] - w) / 2.0, 6.0, w, 18.0];
        p.rounded(r, 6.0, with_alpha(t::POPUP, 0.9));
        p.text_mid(hint, r[0] + 8.0, r[1], r[3], st, t::TEXT);
    }

    /// Moves the dragged element so the mouse at `at` holds it where it was grabbed, snapped.
    fn move_to(&mut self, reg: &Registry, items: &mut [Item], at: [f32; 2], screen: [f32; 2]) {
        let Some((key, grab)) = self.drag else { return };
        let Some(idx) = items.iter().position(|it| it.key == key) else {
            return;
        };
        let r = items[idx].rect;
        let wanted = [at[0] - grab[0], at[1] - grab[1], r[2], r[3]];
        let others: Vec<[f32; 4]> = items
            .iter()
            .filter(|it| it.key != key)
            .map(|it| it.rect)
            .collect();
        let (tl, guides) = if self.free {
            ([wanted[0], wanted[1]], Vec::new())
        } else {
            hud::snap(wanted, &others, screen, SNAP)
        };
        let tl = [
            tl[0].clamp(0.0, (screen[0] - r[2]).max(0.0)),
            tl[1].clamp(0.0, (screen[1] - r[3]).max(0.0)),
        ];
        self.guides = guides;
        let scale = reg.hud_pos(key.0, key.1).scale;
        reg.set_hud_pos(
            key.0,
            key.1,
            HudPos::from_top_left(tl, [r[2], r[3]], screen, scale),
        );
        items[idx].rect = [tl[0], tl[1], r[2], r[3]];
    }
}
