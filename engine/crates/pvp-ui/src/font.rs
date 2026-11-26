//! Poppins text, as OneConfig draws it: glyphs rasterized with `ab_glyph` at the screen's physical pixel size
//! (so text stays sharp at every GUI scale) into the UI atlas, then drawn as tinted sprites.
//!
//! Poppins is © The Poppins Project Authors, under the SIL Open Font License 1.1 (`assets/fonts/OFL.txt`).

use crate::atlas::{Atlas, Slot};
use ab_glyph::{Font as _, FontRef, PxScale, ScaleFont};
use std::collections::HashMap;

/// Poppins weights the UI uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Weight {
    Regular,
    Medium,
    SemiBold,
}

/// A text style: weight and size in UI units (GUI pixels at scale 1; OneConfig's `sp`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    pub weight: Weight,
    pub size: f32,
    /// Extra advance after each character, in UI units.
    pub tracking: f32,
}

impl Style {
    pub const fn new(weight: Weight, size: f32) -> Self {
        Self {
            weight,
            size,
            tracking: 0.0,
        }
    }
}

static REGULAR: &[u8] = include_bytes!("../assets/fonts/Poppins-Regular.ttf");
static MEDIUM: &[u8] = include_bytes!("../assets/fonts/Poppins-Medium.ttf");
static SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Poppins-SemiBold.ttf");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    weight: Weight,
    /// Physical pixel size, in quarter pixels.
    px4: u32,
    ch: char,
}

/// A rasterized glyph: its atlas slot (None for a blank one, a space) and its offset from the pen position at
/// the baseline, in physical pixels.
#[derive(Clone, Copy)]
struct Glyph {
    slot: Option<Slot>,
    offset: [f32; 2],
}

/// One glyph to draw: its atlas source and destination rectangle in GUI pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quad {
    pub src: [f32; 4],
    pub dst: [f32; 4],
}

pub struct Fonts {
    faces: [FontRef<'static>; 3],
    glyphs: HashMap<Key, Glyph>,
    generation: u32,
}

impl Default for Fonts {
    fn default() -> Self {
        let face = |b| FontRef::try_from_slice(b).expect("bundled Poppins parses");
        Self {
            faces: [face(REGULAR), face(MEDIUM), face(SEMIBOLD)],
            glyphs: HashMap::new(),
            generation: 0,
        }
    }
}

impl Fonts {
    fn face(&self, w: Weight) -> &FontRef<'static> {
        &self.faces[w as usize]
    }

    /// Ascent above the baseline, in UI units: where the baseline sits below a line's top.
    pub fn ascent(&self, style: Style) -> f32 {
        self.face(style.weight)
            .as_scaled(PxScale::from(style.size))
            .ascent()
    }

    /// Height of a line (ascent + descent), in UI units.
    pub fn line_height(&self, style: Style) -> f32 {
        let f = self.face(style.weight).as_scaled(PxScale::from(style.size));
        f.ascent() - f.descent()
    }

    /// Width of `text` in UI units. Measured at the UI size, independent of the screen's scale.
    pub fn width(&self, text: &str, style: Style) -> f32 {
        let f = self.face(style.weight).as_scaled(PxScale::from(style.size));
        let mut w = 0.0;
        let mut prev = None;
        for ch in text.chars() {
            let id = f.glyph_id(ch);
            if let Some(p) = prev {
                w += f.kern(p, id);
            }
            w += f.h_advance(id) + style.tracking;
            prev = Some(id);
        }
        w - if text.is_empty() { 0.0 } else { style.tracking }
    }

    /// The glyph quads that draw `text` with its line's top left at `at` (in any unit), with `scale` physical
    /// pixels per unit; the quads are in the same unit. Glyphs not yet in `atlas` are rasterized into it; one
    /// that finds no room is left out (the atlas is marked full, and its owner clears it before the next frame).
    pub fn layout(
        &mut self,
        atlas: &mut Atlas,
        text: &str,
        style: Style,
        at: [f32; 2],
        scale: f32,
    ) -> Vec<Quad> {
        if self.generation != atlas.generation {
            self.glyphs.clear();
            self.generation = atlas.generation;
        }

        let px = style.size * scale;
        let px4 = (px * 4.0).round() as u32;
        let face = self.faces[style.weight as usize].clone();
        let f = face.as_scaled(PxScale::from(px4 as f32 / 4.0));
        // Pen in physical pixels, baseline snapped to the pixel grid so glyphs land on whole pixels.
        let origin = [
            (at[0] * scale).round(),
            (at[1] * scale + f.ascent()).round(),
        ];
        let mut pen = 0.0f32;
        let mut prev = None;
        let mut quads = Vec::with_capacity(text.len());
        for ch in text.chars() {
            let id = f.glyph_id(ch);
            if let Some(p) = prev {
                pen += f.kern(p, id);
            }
            let key = Key {
                weight: style.weight,
                px4,
                ch,
            };
            let glyph = match self.glyphs.get(&key) {
                Some(g) => Some(*g),
                None => {
                    let g = rasterize(atlas, &face, px4 as f32 / 4.0, ch);
                    if let Some(g) = g {
                        self.glyphs.insert(key, g);
                    }
                    g
                }
            };
            let glyph = glyph.unwrap_or(Glyph {
                slot: None,
                offset: [0.0, 0.0],
            });
            if let Some(slot) = glyph.slot {
                let x = origin[0] + pen.round() + glyph.offset[0];
                let y = origin[1] + glyph.offset[1];
                quads.push(Quad {
                    src: slot.src(),
                    dst: [
                        x / scale,
                        y / scale,
                        slot.w as f32 / scale,
                        slot.h as f32 / scale,
                    ],
                });
            }
            pen += f.h_advance(id) + style.tracking * scale;
            prev = Some(id);
        }
        quads
    }
}

/// Rasterizes `ch` at `px` pixels into `atlas`; `None` when the atlas is full.
fn rasterize(atlas: &mut Atlas, face: &FontRef<'static>, px: f32, ch: char) -> Option<Glyph> {
    let f = face.as_scaled(PxScale::from(px));
    let glyph = f.scaled_glyph(ch);
    let Some(outline) = face.outline_glyph(glyph) else {
        return Some(Glyph {
            slot: None,
            offset: [0.0, 0.0],
        });
    };
    let b = outline.px_bounds();
    let (w, h) = (b.width() as u32, b.height() as u32);
    if w == 0 || h == 0 {
        return Some(Glyph {
            slot: None,
            offset: [0.0, 0.0],
        });
    }
    let slot = atlas.alloc(w, h)?;
    outline.draw(|x, y, c| atlas.put(slot, x, y, (c.clamp(0.0, 1.0) * 255.0).round() as u8));
    Some(Glyph {
        slot: Some(slot),
        offset: [b.min.x, b.min.y],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: Style = Style::new(Weight::Regular, 14.0);

    #[test]
    fn width_grows_with_text_and_size_and_ignores_scale() {
        let f = Fonts::default();
        let a = f.width("Sprint", BODY);
        assert!(a > 20.0 && a < 80.0, "{a}");
        assert!(f.width("Toggle Sprint", BODY) > a);
        assert!(f.width("Sprint", Style::new(Weight::Regular, 28.0)) > a * 1.9);
        assert_eq!(f.width("", BODY), 0.0);
    }

    #[test]
    fn layout_reuses_glyphs_and_skips_blank_ones() {
        let mut f = Fonts::default();
        let mut atlas = Atlas::new(256, 128);
        let q = f.layout(&mut atlas, "a a", BODY, [10.0, 20.0], 2.0);
        assert_eq!(q.len(), 2, "the space draws nothing");
        assert_eq!(q[0].src, q[1].src, "the second 'a' reuses the first's slot");
        assert!(q[1].dst[0] > q[0].dst[0]);
        // Glyph quads sit inside the line: below its top and above its bottom.
        let lh = f.line_height(BODY);
        for g in &q {
            assert!(
                g.dst[1] >= 20.0 - 0.5 && g.dst[1] + g.dst[3] <= 20.0 + lh + 0.5,
                "{g:?}"
            );
        }
    }

    #[test]
    fn a_full_atlas_drops_glyphs_and_is_flagged_and_a_clear_brings_them_back() {
        let mut f = Fonts::default();
        let mut atlas = Atlas::new(48, 48);
        let text: String = ('A'..='Z').collect();
        let style = Style::new(Weight::SemiBold, 14.0);
        let q = f.layout(&mut atlas, &text, style, [0.0, 0.0], 1.0);
        assert!(q.len() < 26 && atlas.full, "{}", q.len());
        atlas.clear();
        let q = f.layout(&mut atlas, "WW", style, [0.0, 0.0], 1.0);
        assert_eq!(q.len(), 2);
        assert!(!atlas.full);
    }
}
