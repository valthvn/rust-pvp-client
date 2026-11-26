//! Custom crosshair (PolyCrosshair): hides the game's crosshair layer and draws a cross, T or dot of a chosen
//! size, gap, thickness and colour, with an optional outline and centre dot.

use mod_api::{ClientHost, DrawContext, Flow, GuiLayerPreEvent};
use pvp_core::{Category, Cx, Events, Module, Setting};

pub struct Crosshair;
pub static CROSSHAIR: Crosshair = Crosshair;

const STYLE: usize = 0;
const LENGTH: usize = 1;
const GAP: usize = 2;
const THICKNESS: usize = 3;
const COLOR: usize = 4;
const DOT: usize = 5;
const OUTLINE: usize = 6;
const OUTLINE_COLOR: usize = 7;
const THIRD_PERSON: usize = 8;

static SETTINGS: [Setting; 9] = [
    Setting::choice("style", "pvp.crosshair.style", &["cross", "t", "dot"], 0),
    Setting::slider("length", "pvp.crosshair.length", 1.0, 15.0, 1.0, 5.0),
    Setting::slider("gap", "pvp.crosshair.gap", 0.0, 10.0, 1.0, 1.0),
    Setting::slider("thickness", "pvp.crosshair.thickness", 1.0, 5.0, 1.0, 1.0),
    Setting::color("color", "pvp.crosshair.color", 0xFFFF_FFFF),
    Setting::toggle("dot", "pvp.crosshair.dot", false),
    Setting::toggle("outline", "pvp.crosshair.outline", false),
    Setting::color("outline_color", "pvp.crosshair.outline_color", 0xFF00_0000),
    Setting::toggle("third_person", "pvp.crosshair.third_person", false),
];

#[derive(Clone, Copy, Debug, PartialEq)]
struct Shape {
    style: usize,
    length: f32,
    gap: f32,
    thickness: f32,
    dot: bool,
}

/// The rectangles of the crosshair centred on `c`, in GUI pixels.
fn rects(s: Shape, c: [f32; 2]) -> Vec<[f32; 4]> {
    let t = s.thickness;
    // The centre square, thickness wide, its top left on whole pixels.
    let x0 = (c[0] - t / 2.0).floor();
    let y0 = (c[1] - t / 2.0).floor();
    let mut out = Vec::with_capacity(5);
    if s.style != 2 {
        let (l, g) = (s.length, s.gap);
        out.push([x0 - g - l, y0, l, t]); // left
        out.push([x0 + t + g, y0, l, t]); // right
        out.push([x0, y0 + t + g, t, l]); // bottom
        if s.style == 0 {
            out.push([x0, y0 - g - l, t, l]); // top
        }
    }
    if s.dot || s.style == 2 {
        out.push([x0, y0, t, t]);
    }
    out
}

fn paint(cx: &Cx, draw: &mut dyn DrawContext) {
    let [w, h] = draw.size();
    let shape = Shape {
        style: cx.choice(STYLE),
        length: cx.f32(LENGTH),
        gap: cx.f32(GAP),
        thickness: cx.f32(THICKNESS),
        dot: cx.bool(DOT),
    };
    let rects = rects(shape, [w / 2.0, h / 2.0]);
    if cx.bool(OUTLINE) {
        for r in &rects {
            draw.fill(
                [r[0] - 1.0, r[1] - 1.0, r[2] + 2.0, r[3] + 2.0],
                cx.color(OUTLINE_COLOR),
            );
        }
    }
    for r in &rects {
        draw.fill(*r, cx.color(COLOR));
    }
}

impl Module for Crosshair {
    fn id(&self) -> &'static str {
        "crosshair"
    }
    fn name(&self) -> &'static str {
        "Crosshair"
    }
    fn description(&self) -> &'static str {
        "pvp.crosshair.description"
    }
    fn category(&self) -> Category {
        Category::Pvp
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::GUI_LAYER
    }
    fn gui_layer(&self, cx: &Cx, host: &dyn ClientHost, event: &mut GuiLayerPreEvent<'_>) -> Flow {
        if event.layer != "minecraft:crosshair" {
            return Flow::Continue;
        }
        if host.camera_type().is_first_person() || cx.bool(THIRD_PERSON) {
            paint(cx, &mut *event.draw);
        }
        Flow::Cancel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(style: usize) -> Shape {
        Shape {
            style,
            length: 5.0,
            gap: 1.0,
            thickness: 1.0,
            dot: false,
        }
    }

    #[test]
    fn a_cross_has_four_arms_around_the_centre() {
        let r = rects(shape(0), [100.0, 50.0]);
        assert_eq!(r.len(), 4);
        // The centre pixel is 99..100: the left arm ends a gap before it, the right one starts a gap after.
        assert_eq!(r[0], [93.0, 49.0, 5.0, 1.0]);
        assert_eq!(r[1], [101.0, 49.0, 5.0, 1.0]);
    }

    #[test]
    fn t_has_no_top_and_dot_is_one_square() {
        assert_eq!(rects(shape(1), [10.0, 10.0]).len(), 3);
        assert_eq!(rects(shape(2), [10.0, 10.0]), vec![[9.0, 9.0, 1.0, 1.0]]);
        let dotted = Shape {
            dot: true,
            ..shape(0)
        };
        assert_eq!(rects(dotted, [10.0, 10.0]).len(), 5);
    }
}
