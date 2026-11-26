//! OneConfig's icon set (single-colour SVGs, `assets/icons/`, from Polyfrost's OneConfig under LGPL-3.0),
//! rasterized with `resvg` at the size they are drawn in physical pixels and kept as coverage in the UI
//! atlas, so one image serves every tint.

use crate::atlas::{Atlas, Slot};
use std::collections::HashMap;

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        /// Every bundled icon, by OneConfig's file name.
        static ICONS: &[(&str, &[u8])] = &[
            $(($name, include_bytes!(concat!("../assets/icons/", $name, ".svg")))),*
        ];
    };
}

icons![
    "alert-circle",
    "bell",
    "check-circle",
    "close",
    "cog",
    "combat",
    "copy",
    "dots-vertical",
    "down",
    "eye",
    "eye-off",
    "folder",
    "hud",
    "hypixel",
    "info",
    "keyboard",
    "left-arrow",
    "lightning-01",
    "lock",
    "paintbrush",
    "plus",
    "profiles",
    "qol",
    "refresh",
    "right-arrow",
    "search",
    "settings",
    "star",
    "star-filled",
    "tick",
    "trash",
    "up",
    "x-circle",
];

#[derive(Default)]
pub struct Icons {
    slots: HashMap<(&'static str, u32), Option<Slot>>,
    generation: u32,
}

impl Icons {
    /// Icon `name` as a `px` x `px` coverage image in `atlas`. `None` for an unknown icon, one that does not
    /// parse, or a full atlas.
    pub fn get(&mut self, atlas: &mut Atlas, name: &str, px: u32) -> Option<Slot> {
        if self.generation != atlas.generation {
            self.slots.clear();
            self.generation = atlas.generation;
        }
        let &(key, svg) = ICONS.iter().find(|(n, _)| *n == name)?;
        if let Some(s) = self.slots.get(&(key, px)) {
            return *s;
        }
        // A failure (a full atlas) is not remembered: the next frame tries again.
        let slot = rasterize(atlas, svg, px)?;
        self.slots.insert((key, px), Some(slot));
        Some(slot)
    }
}

fn rasterize(atlas: &mut Atlas, svg: &[u8], px: u32) -> Option<Slot> {
    if px == 0 {
        return None;
    }
    let tree = resvg::usvg::Tree::from_data(svg, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(px, px)?;
    let size = tree.size();
    let s = px as f32 / size.width().max(size.height());
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(s, s),
        &mut pixmap.as_mut(),
    );
    let slot = atlas.alloc(px, px)?;
    for (i, p) in pixmap.pixels().iter().enumerate() {
        let a = p.alpha();
        if a > 0 {
            atlas.put(slot, i as u32 % px, i as u32 / px, a);
        }
    }
    Some(slot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_bundled_icon_rasterizes_with_some_ink() {
        let mut atlas = Atlas::new(1024, 512);
        let mut icons = Icons::default();
        for (name, _) in ICONS {
            let slot = icons
                .get(&mut atlas, name, 32)
                .unwrap_or_else(|| panic!("{name}"));
            let mut ink = 0;
            for y in 0..slot.h {
                for x in 0..slot.w {
                    ink += atlas.pixels
                        [(((slot.y + y) * atlas.width + slot.x + x) * 4 + 3) as usize]
                        as u32;
                }
            }
            assert!(ink > 0, "{name} drew nothing");
        }
        assert!(icons.get(&mut atlas, "no-such-icon", 16).is_none());
    }
}
