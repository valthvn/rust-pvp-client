//! Where HUD elements sit: an anchor (a corner, an edge's middle or the centre) plus an offset in GUI pixels,
//! so a placement stays right when the window or the GUI scale changes. Snapping for the editor.

use serde::{Deserialize, Serialize};

/// One of nine points of the screen; the element's matching point sits on it (a `BottomRight` element's
/// bottom-right corner is at the screen's bottom-right, moved by the offset).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Anchor {
    /// The anchor's position as fractions of the width and height: 0, ½ or 1.
    fn fractions(self) -> [f32; 2] {
        let fx = match self {
            Anchor::TopLeft | Anchor::Left | Anchor::BottomLeft => 0.0,
            Anchor::Top | Anchor::Center | Anchor::Bottom => 0.5,
            _ => 1.0,
        };
        let fy = match self {
            Anchor::TopLeft | Anchor::Top | Anchor::TopRight => 0.0,
            Anchor::Left | Anchor::Center | Anchor::Right => 0.5,
            _ => 1.0,
        };
        [fx, fy]
    }

    fn from_fractions(fx: f32, fy: f32) -> Self {
        // The screen is split in thirds: an element in the left third anchors left, and so on.
        let col = if fx < 1.0 / 3.0 {
            0
        } else if fx < 2.0 / 3.0 {
            1
        } else {
            2
        };
        let row = if fy < 1.0 / 3.0 {
            0
        } else if fy < 2.0 / 3.0 {
            1
        } else {
            2
        };
        [
            [Anchor::TopLeft, Anchor::Top, Anchor::TopRight],
            [Anchor::Left, Anchor::Center, Anchor::Right],
            [Anchor::BottomLeft, Anchor::Bottom, Anchor::BottomRight],
        ][row][col]
    }
}

/// A placed HUD element.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HudPos {
    pub anchor: Anchor,
    /// Offset of the element's anchor point from the screen's, in GUI pixels.
    pub x: f32,
    pub y: f32,
    /// Size multiplier, 0.5 to 3.
    pub scale: f32,
}

impl HudPos {
    pub const fn new(anchor: Anchor, x: f32, y: f32) -> Self {
        Self {
            anchor,
            x,
            y,
            scale: 1.0,
        }
    }

    /// The element's top-left corner on a `screen` of `[w, h]` GUI pixels, for an element of `size` (unscaled),
    /// kept inside the screen.
    pub fn top_left(&self, size: [f32; 2], screen: [f32; 2]) -> [f32; 2] {
        let [fx, fy] = self.anchor.fractions();
        let (w, h) = (size[0] * self.scale, size[1] * self.scale);
        let x = screen[0] * fx + self.x - w * fx;
        let y = screen[1] * fy + self.y - h * fy;
        [
            x.clamp(0.0, (screen[0] - w).max(0.0)),
            y.clamp(0.0, (screen[1] - h).max(0.0)),
        ]
    }

    /// The placement of an element whose top-left is at `top_left`: anchored to the nearest of the nine points
    /// (by the element's centre), keeping `scale`.
    pub fn from_top_left(top_left: [f32; 2], size: [f32; 2], screen: [f32; 2], scale: f32) -> Self {
        let (w, h) = (size[0] * scale, size[1] * scale);
        let cx = (top_left[0] + w / 2.0) / screen[0].max(1.0);
        let cy = (top_left[1] + h / 2.0) / screen[1].max(1.0);
        let anchor = Anchor::from_fractions(cx, cy);
        let [fx, fy] = anchor.fractions();
        Self {
            anchor,
            x: top_left[0] + w * fx - screen[0] * fx,
            y: top_left[1] + h * fy - screen[1] * fy,
            scale,
        }
    }
}

/// A snapping guide the editor draws: a vertical line at `x` or a horizontal line at `y`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Guide {
    Vertical(f32),
    Horizontal(f32),
}

/// Snaps a dragged rectangle `[x, y, w, h]` to the screen's edges and centre lines and to the edges and centres
/// of `others`, within `threshold` GUI pixels on each axis. Returns the snapped top-left and the guides used.
pub fn snap(
    rect: [f32; 4],
    others: &[[f32; 4]],
    screen: [f32; 2],
    threshold: f32,
) -> ([f32; 2], Vec<Guide>) {
    let [x, y, w, h] = rect;
    let mut xs = vec![0.0, screen[0] / 2.0, screen[0]];
    let mut ys = vec![0.0, screen[1] / 2.0, screen[1]];
    for o in others {
        xs.extend([o[0], o[0] + o[2] / 2.0, o[0] + o[2]]);
        ys.extend([o[1], o[1] + o[3] / 2.0, o[1] + o[3]]);
    }
    let axis = |start: f32, len: f32, lines: &[f32]| -> Option<(f32, f32)> {
        // Each of the element's three lines (start, centre, end) against each target line: the closest wins.
        let mut best: Option<(f32, f32, f32)> = None; // (distance, new start, line)
        for (k, own) in [start, start + len / 2.0, start + len]
            .into_iter()
            .enumerate()
        {
            for &line in lines {
                let d = (line - own).abs();
                if d <= threshold && best.is_none_or(|b| d < b.0) {
                    best = Some((d, line - [0.0, len / 2.0, len][k], line));
                }
            }
        }
        best.map(|b| (b.1, b.2))
    };
    let mut guides = Vec::new();
    let nx = match axis(x, w, &xs) {
        Some((nx, line)) => {
            guides.push(Guide::Vertical(line));
            nx
        }
        None => x,
    };
    let ny = match axis(y, h, &ys) {
        Some((ny, line)) => {
            guides.push(Guide::Horizontal(line));
            ny
        }
        None => y,
    };
    ([nx, ny], guides)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: [f32; 2] = [480.0, 270.0];

    #[test]
    fn corners_follow_the_screen_when_it_is_resized() {
        let pos = HudPos::new(Anchor::BottomRight, -2.0, -2.0);
        assert_eq!(pos.top_left([40.0, 10.0], SCREEN), [438.0, 258.0]);
        assert_eq!(pos.top_left([40.0, 10.0], [640.0, 360.0]), [598.0, 348.0]);
    }

    #[test]
    fn a_dropped_element_anchors_to_the_nearest_point_and_stays_where_it_was_dropped() {
        for (tl, anchor) in [
            ([5.0, 5.0], Anchor::TopLeft),
            ([220.0, 120.0], Anchor::Center),
            ([430.0, 250.0], Anchor::BottomRight),
            ([220.0, 3.0], Anchor::Top),
        ] {
            let pos = HudPos::from_top_left(tl, [40.0, 10.0], SCREEN, 1.0);
            assert_eq!(pos.anchor, anchor);
            let back = pos.top_left([40.0, 10.0], SCREEN);
            assert!(
                (back[0] - tl[0]).abs() < 1e-3 && (back[1] - tl[1]).abs() < 1e-3,
                "{tl:?} → {back:?}"
            );
        }
    }

    #[test]
    fn elements_never_leave_the_screen() {
        let pos = HudPos::new(Anchor::TopLeft, -50.0, 900.0);
        assert_eq!(pos.top_left([40.0, 10.0], SCREEN), [0.0, 260.0]);
    }

    #[test]
    fn dragging_near_the_centre_line_or_another_element_snaps_to_it() {
        // Centre 2 px left of the screen's centre line: snaps onto it.
        let (tl, guides) = snap([218.0, 100.0, 40.0, 10.0], &[], SCREEN, 4.0);
        assert_eq!(tl, [220.0, 100.0]);
        assert_eq!(guides, vec![Guide::Vertical(240.0)]);
        // Top edge 3 px under another element's bottom edge: lines up with it.
        let (tl, _) = snap(
            [300.0, 33.0, 40.0, 10.0],
            &[[300.0, 20.0, 40.0, 10.0]],
            SCREEN,
            4.0,
        );
        assert_eq!(tl[1], 30.0);
        // Far from everything: unchanged.
        let (tl, guides) = snap([100.0, 100.0, 40.0, 10.0], &[], SCREEN, 4.0);
        assert_eq!((tl, guides.len()), ([100.0, 100.0], 0));
    }
}
