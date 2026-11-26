//! Hitboxes (PolyHitbox, F3+B): entities' bounding boxes and where they look, drawn in the world.

use mod_api::{ClientHost, RenderLevelStage, RenderLevelStageEvent, WorldDepth, WorldLine};
use pvp_core::{Category, Cx, Events, Module, Setting};

pub struct Hitbox;
pub static HITBOX: Hitbox = Hitbox;

const COLOR: usize = 0;
const WIDTH: usize = 1;
const LOOK: usize = 2;
const LOOK_COLOR: usize = 3;
const RANGE: usize = 4;
const PLAYERS_ONLY: usize = 5;
const THROUGH_WALLS: usize = 6;

static SETTINGS: [Setting; 7] = [
    Setting::color("color", "pvp.hitbox.color", 0xFFFF_FFFF),
    Setting::slider("width", "pvp.hitbox.width", 1.0, 5.0, 0.5, 2.0),
    Setting::toggle("look", "pvp.hitbox.look", true),
    Setting::color("look_color", "pvp.hitbox.look_color", 0xFF00_00FF),
    Setting::slider("range", "pvp.hitbox.range", 8.0, 64.0, 1.0, 32.0),
    Setting::toggle("players_only", "pvp.hitbox.players_only", false),
    Setting::toggle("through_walls", "pvp.hitbox.through_walls", false),
];

/// The 12 edges of a box.
fn edges(b: [[f64; 3]; 2]) -> [([f64; 3], [f64; 3]); 12] {
    let [a, z] = b;
    let p = |x: bool, y: bool, w: bool| {
        [
            if x { z[0] } else { a[0] },
            if y { z[1] } else { a[1] },
            if w { z[2] } else { a[2] },
        ]
    };
    [
        (p(false, false, false), p(true, false, false)),
        (p(false, false, true), p(true, false, true)),
        (p(false, true, false), p(true, true, false)),
        (p(false, true, true), p(true, true, true)),
        (p(false, false, false), p(false, true, false)),
        (p(true, false, false), p(true, true, false)),
        (p(false, false, true), p(false, true, true)),
        (p(true, false, true), p(true, true, true)),
        (p(false, false, false), p(false, false, true)),
        (p(true, false, false), p(true, false, true)),
        (p(false, true, false), p(false, true, true)),
        (p(true, true, false), p(true, true, true)),
    ]
}

impl Module for Hitbox {
    fn id(&self) -> &'static str {
        "hitbox"
    }
    fn name(&self) -> &'static str {
        "Hitboxes"
    }
    fn description(&self) -> &'static str {
        "pvp.hitbox.description"
    }
    fn category(&self) -> Category {
        Category::Pvp
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::LEVEL_STAGE
    }
    fn level_stage(&self, cx: &Cx, host: &dyn ClientHost, event: &mut RenderLevelStageEvent<'_>) {
        if event.stage != RenderLevelStage::AfterLevel {
            return;
        }
        let Some(me) = host.local_player().map(|p| p.entity()) else {
            return;
        };
        let Some(level) = me.level(host) else {
            return;
        };
        let c = event.view.camera;
        let r = f64::from(cx.f32(RANGE));
        let Some(near) = level.entities_in(
            host,
            [c[0] - r, c[1] - r, c[2] - r],
            [c[0] + r, c[1] + r, c[2] + r],
        ) else {
            return;
        };
        let depth = if cx.bool(THROUGH_WALLS) {
            WorldDepth::ThroughWalls
        } else {
            WorldDepth::Tested
        };
        let width = cx.f32(WIDTH);
        let first_person = host.camera_type().is_first_person();
        let mut lines = Vec::with_capacity(near.len() * 13);
        for e in near {
            if e == me && first_person {
                continue;
            }
            if cx.bool(PLAYERS_ONLY)
                && e.kind(host)
                    .is_none_or(|k| k.id(host) != "minecraft:player")
            {
                continue;
            }
            let Some(b) = e.bounding_box(host) else {
                continue;
            };
            for (from, to) in edges(b) {
                lines.push(WorldLine::new(from, to, cx.color(COLOR), width, depth));
            }
            if cx.bool(LOOK)
                && let (Some(eye), Some(v)) = (e.eye_position(host), e.view_vector(host))
            {
                let to = [
                    eye[0] + v[0] * 2.0,
                    eye[1] + v[1] * 2.0,
                    eye[2] + v[2] * 2.0,
                ];
                lines.push(WorldLine::new(eye, to, cx.color(LOOK_COLOR), width, depth));
            }
        }
        let _ = event.draw.lines(&lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_box_has_twelve_axis_aligned_edges_of_its_sides() {
        let e = edges([[0.0, 0.0, 0.0], [1.0, 2.0, 3.0]]);
        let mut total = 0.0;
        for (a, b) in e {
            let changed = (0..3).filter(|&i| a[i] != b[i]).count();
            assert_eq!(changed, 1, "{a:?} {b:?}");
            total += (0..3).map(|i| (b[i] - a[i]).abs()).sum::<f64>();
        }
        assert_eq!(total, 4.0 * (1.0 + 2.0 + 3.0));
    }
}
