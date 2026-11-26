//! Rounded rectangles and their borders, which `DrawContext` cannot draw: a disc (or ring) of the corner
//! radius is rasterized once at the screen's physical pixel size, its four quarters become the corners, and
//! plain fills make the rest.

use crate::atlas::{Atlas, Slot};
use std::collections::HashMap;

/// Samples per axis per pixel for edge coverage.
const SS: u32 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    /// Radius in physical pixels.
    r: u32,
    /// Ring width in physical pixels; 0 for a filled disc.
    ring: u32,
}

#[derive(Default)]
pub struct Shapes {
    discs: HashMap<Key, Slot>,
    generation: u32,
}

impl Shapes {
    /// A `2r` x `2r` disc (or ring of width `ring`) in `atlas`; `None` when the atlas is full.
    pub fn disc(&mut self, atlas: &mut Atlas, r: u32, ring: u32) -> Option<Slot> {
        if self.generation != atlas.generation {
            self.discs.clear();
            self.generation = atlas.generation;
        }
        let key = Key { r, ring };
        if let Some(s) = self.discs.get(&key) {
            return Some(*s);
        }
        let d = r * 2;
        let slot = atlas.alloc(d, d)?;
        for y in 0..d {
            for x in 0..d {
                let a = coverage(x, y, r as f32, ring as f32);
                if a > 0 {
                    atlas.put(slot, x, y, a);
                }
            }
        }
        self.discs.insert(key, slot);
        Some(slot)
    }
}

/// Coverage (0..=255) of pixel `(x, y)` by a disc of radius `r` centred at `(r, r)`, or by its outer ring of
/// width `ring` when `ring > 0`.
fn coverage(x: u32, y: u32, r: f32, ring: f32) -> u8 {
    let mut n = 0;
    for sy in 0..SS {
        for sx in 0..SS {
            let px = x as f32 + (sx as f32 + 0.5) / SS as f32 - r;
            let py = y as f32 + (sy as f32 + 0.5) / SS as f32 - r;
            let d = (px * px + py * py).sqrt();
            if d <= r && (ring <= 0.0 || d >= r - ring) {
                n += 1;
            }
        }
    }
    (n * 255 / (SS * SS)) as u8
}

/// The pieces that make a rounded rectangle, in GUI pixels: corner sprites (atlas source, destination) and
/// plain fills.
#[derive(Debug, Default, PartialEq)]
pub struct Pieces {
    pub corners: Vec<([f32; 4], [f32; 4])>,
    pub fills: Vec<[f32; 4]>,
}

/// Splits `rect` (`[x, y, w, h]`, GUI pixels) with corner radius `radius` (GUI pixels) into pieces, given the
/// disc `slot` of radius `r_px` physical pixels and `scale` physical pixels per GUI pixel. With `ring` (a
/// border of that many GUI pixels), the fills are the four edges instead of the inside.
pub fn pieces(rect: [f32; 4], slot: Slot, r_px: u32, scale: f32, ring: Option<f32>) -> Pieces {
    let [x, y, w, h] = rect;
    let r = r_px as f32 / scale;
    let rp = r_px as f32;
    let (sx, sy) = (slot.x as f32, slot.y as f32);
    let corners = vec![
        ([sx, sy, rp, rp], [x, y, r, r]),
        ([sx + rp, sy, rp, rp], [x + w - r, y, r, r]),
        ([sx, sy + rp, rp, rp], [x, y + h - r, r, r]),
        ([sx + rp, sy + rp, rp, rp], [x + w - r, y + h - r, r, r]),
    ];
    let fills = match ring {
        None => vec![
            [x + r, y, w - 2.0 * r, h],
            [x, y + r, r, h - 2.0 * r],
            [x + w - r, y + r, r, h - 2.0 * r],
        ],
        Some(b) => vec![
            [x + r, y, w - 2.0 * r, b],
            [x + r, y + h - b, w - 2.0 * r, b],
            [x, y + r, b, h - 2.0 * r],
            [x + w - b, y + r, b, h - 2.0 * r],
        ],
    };
    Pieces {
        corners,
        fills: fills
            .into_iter()
            .filter(|f| f[2] > 0.0 && f[3] > 0.0)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_disc_is_full_inside_empty_outside_and_soft_on_its_edge() {
        assert_eq!(coverage(5, 5, 6.0, 0.0), 255);
        assert_eq!(coverage(0, 0, 6.0, 0.0), 0);
        let edge = coverage(1, 1, 6.0, 0.0);
        assert!(edge > 0 && edge < 255, "{edge}");
        // A ring is empty at its centre.
        assert_eq!(coverage(5, 5, 6.0, 1.0), 0);
    }

    #[test]
    fn discs_are_cached_per_radius() {
        let mut atlas = Atlas::new(64, 64);
        let mut s = Shapes::default();
        let a = s.disc(&mut atlas, 4, 0).unwrap();
        assert_eq!(s.disc(&mut atlas, 4, 0), Some(a));
        assert_ne!(s.disc(&mut atlas, 4, 1), Some(a));
    }

    #[test]
    fn pieces_cover_the_rectangle_without_overlap() {
        let slot = Slot {
            x: 0,
            y: 0,
            w: 8,
            h: 8,
        };
        let p = pieces([10.0, 10.0, 40.0, 20.0], slot, 8, 2.0, None);
        let area: f32 = p.fills.iter().map(|f| f[2] * f[3]).sum::<f32>()
            + p.corners.iter().map(|c| c.1[2] * c.1[3]).sum::<f32>();
        assert!((area - 800.0).abs() < 1e-3, "{area}");
        assert_eq!(p.corners[3].1, [46.0, 26.0, 4.0, 4.0]);
    }
}
