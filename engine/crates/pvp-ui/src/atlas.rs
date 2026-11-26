//! A CPU-side image the UI rasterizes into (glyphs, rounded corners, icons) and uploads to the game as one
//! dynamic GUI texture. Space is handed out by a shelf packer; nothing is ever freed one by one (the whole
//! atlas is cleared when it fills, which happens only after a GUI scale change or with unusual text).

/// Where an image landed in the atlas, in atlas pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Slot {
    /// `[x, y, w, h]` as `DrawContext::sprite` takes its source rectangle.
    pub fn src(&self) -> [f32; 4] {
        [self.x as f32, self.y as f32, self.w as f32, self.h as f32]
    }
}

pub struct Atlas {
    pub width: u32,
    pub height: u32,
    /// RGBA, row by row from the top left.
    pub pixels: Vec<u8>,
    /// Shelves: (top, height, next free x).
    shelves: Vec<(u32, u32, u32)>,
    /// Pixels changed since the last upload.
    pub dirty: bool,
    /// Bumped by [`clear`](Self::clear): slots handed out before it are stale.
    pub generation: u32,
    /// An allocation failed: the owner clears the atlas before the next frame draws (never mid-frame, when
    /// sprites already emitted still point into it).
    pub full: bool,
}

/// One empty pixel around every image, so linear filtering never bleeds a neighbour in.
const PAD: u32 = 1;

impl Atlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
            shelves: Vec::new(),
            dirty: true,
            generation: 0,
            full: false,
        }
    }

    /// Room for a `w` x `h` image, or `None` when the atlas is full.
    pub fn alloc(&mut self, w: u32, h: u32) -> Option<Slot> {
        let (pw, ph) = (w + PAD * 2, h + PAD * 2);
        if pw > self.width || ph > self.height {
            self.full = true;
            return None;
        }
        // The lowest shelf it fits on, wasting the least height.
        let mut best: Option<usize> = None;
        for (i, &(_, sh, next)) in self.shelves.iter().enumerate() {
            if sh >= ph && self.width - next >= pw && best.is_none_or(|b| self.shelves[b].1 > sh) {
                best = Some(i);
            }
        }
        let i = match best {
            Some(i) => i,
            None => {
                let top = self.shelves.last().map_or(0, |&(t, h, _)| t + h);
                if top + ph > self.height {
                    self.full = true;
                    return None;
                }
                self.shelves.push((top, ph, 0));
                self.shelves.len() - 1
            }
        };
        let (top, _, next) = &mut self.shelves[i];
        let slot = Slot {
            x: *next + PAD,
            y: *top + PAD,
            w,
            h,
        };
        *next += pw;
        Some(slot)
    }

    /// Writes coverage `alpha` (0..=255) as white at `(x, y)` inside `slot`.
    pub fn put(&mut self, slot: Slot, x: u32, y: u32, alpha: u8) {
        if x >= slot.w || y >= slot.h {
            return;
        }
        let i = (((slot.y + y) * self.width + slot.x + x) * 4) as usize;
        self.pixels[i..i + 4].copy_from_slice(&[255, 255, 255, alpha]);
        self.dirty = true;
    }

    /// Writes an RGBA pixel at `(x, y)` inside `slot`.
    pub fn put_rgba(&mut self, slot: Slot, x: u32, y: u32, rgba: [u8; 4]) {
        if x >= slot.w || y >= slot.h {
            return;
        }
        let i = (((slot.y + y) * self.width + slot.x + x) * 4) as usize;
        self.pixels[i..i + 4].copy_from_slice(&rgba);
        self.dirty = true;
    }

    /// Forgets every image.
    pub fn clear(&mut self) {
        self.pixels.fill(0);
        self.shelves.clear();
        self.dirty = true;
        self.full = false;
        self.generation = self.generation.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_never_overlap_and_the_atlas_reports_when_it_is_full() {
        let mut a = Atlas::new(64, 32);
        let mut slots = Vec::new();
        while let Some(s) = a.alloc(10, 7) {
            slots.push(s);
        }
        assert!(a.full);
        assert!(slots.len() >= 8, "{}", slots.len());
        for (i, p) in slots.iter().enumerate() {
            assert!(p.x + p.w <= 64 && p.y + p.h <= 32);
            for q in &slots[i + 1..] {
                let apart =
                    p.x + p.w < q.x || q.x + q.w < p.x || p.y + p.h < q.y || q.y + q.h < p.y;
                assert!(apart, "{p:?} {q:?}");
            }
        }
        a.clear();
        assert!(a.alloc(10, 7).is_some());
        assert_eq!(a.generation, 1);
    }

    #[test]
    fn put_writes_white_with_the_coverage_as_alpha() {
        let mut a = Atlas::new(8, 8);
        let s = a.alloc(2, 2).unwrap();
        a.put(s, 1, 1, 200);
        let i = (((s.y + 1) * 8 + s.x + 1) * 4) as usize;
        assert_eq!(&a.pixels[i..i + 4], &[255, 255, 255, 200]);
    }
}
