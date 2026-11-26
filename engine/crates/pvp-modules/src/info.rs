//! One-line information HUD elements (EvergreenHUD, VanillaHUD): ping, coordinates with facing, and clock.

use crate::text::{self, Line};
use mod_api::ClientHost;
use pvp_core::{Anchor, Category, Cx, Events, HudElement, HudPos, Module, Setting};

/// A module made of one [`Line`] and the shared text settings plus its own.
pub struct Info {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    settings: &'static [Setting],
    hud: &'static [&'static dyn HudElement],
}

impl Module for Info {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn description(&self) -> &'static str {
        self.description
    }
    fn category(&self) -> Category {
        Category::Hud
    }
    fn settings(&self) -> &'static [Setting] {
        self.settings
    }
    fn events(&self) -> Events {
        Events::NONE
    }
    fn hud(&self) -> &'static [&'static dyn HudElement] {
        self.hud
    }
}

// --- Ping ---

static PING_SETTINGS: [Setting; 3] = [text::BACKGROUND, text::COLOR, text::SHADOW];
static PING_LINE: Line = Line {
    id: "ping",
    label: "pvp.ping.line",
    pos: HudPos::new(Anchor::TopLeft, 4.0, 22.0),
    widest: |_| "888 ms".to_owned(),
    // Unknown (single player before the first keep-alive) shows nothing.
    text: |_, host| {
        host.client_session()
            .client_latency()
            .map(|ms| format!("{ms} ms"))
    },
};
static PING_HUD: [&dyn HudElement; 1] = [&PING_LINE];
pub static PING: Info = Info {
    id: "ping",
    name: "Ping",
    description: "pvp.ping.description",
    settings: &PING_SETTINGS,
    hud: &PING_HUD,
};

// --- Coordinates ---

const FACING: usize = 3;
const DECIMALS: usize = 4;
static COORDS_SETTINGS: [Setting; 5] = [
    text::BACKGROUND,
    text::COLOR,
    text::SHADOW,
    Setting::toggle("facing", "pvp.coords.facing", true),
    Setting::slider("decimals", "pvp.coords.decimals", 0.0, 3.0, 1.0, 0.0),
];

/// The facing of a yaw in degrees, as the game names it (0 is south, 90 west).
fn facing(yaw: f32) -> &'static str {
    match (yaw.rem_euclid(360.0) / 90.0).round() as i32 % 4 {
        0 => "south",
        1 => "west",
        2 => "north",
        _ => "east",
    }
}

/// The axis a facing name walks along.
fn axis(facing: &str) -> &'static str {
    match facing {
        "north" => "-Z",
        "south" => "+Z",
        "east" => "+X",
        "west" => "-X",
        _ => "",
    }
}

fn coords_text(pos: [f64; 3], decimals: usize, facing: Option<&str>) -> String {
    let n = decimals;
    let mut s = format!("{:.n$} {:.n$} {:.n$}", pos[0], pos[1], pos[2]);
    if let Some(f) = facing {
        s.push_str(&format!(" {f} ({})", axis(f)));
    }
    s
}

static COORDS_LINE: Line = Line {
    id: "coords",
    label: "pvp.coords.line",
    pos: HudPos::new(Anchor::TopLeft, 4.0, 40.0),
    widest: |cx| {
        let n = cx.f32(DECIMALS) as usize;
        coords_text(
            [-30_000.0, 320.0, -30_000.0],
            n,
            cx.bool(FACING).then_some("north"),
        )
    },
    text: |cx, host| {
        let me = host.local_player()?.entity();
        let pos = me.position(host)?;
        let facing = if cx.bool(FACING) {
            me.rotation(host).map(|r| facing(r[0]))
        } else {
            None
        };
        Some(coords_text(pos, cx.f32(DECIMALS) as usize, facing))
    },
};
static COORDS_HUD: [&dyn HudElement; 1] = [&COORDS_LINE];
pub static COORDS: Info = Info {
    id: "coords",
    name: "Coordinates",
    description: "pvp.coords.description",
    settings: &COORDS_SETTINGS,
    hud: &COORDS_HUD,
};

// --- Clock ---

const TWELVE_HOUR: usize = 3;
const SECONDS: usize = 4;
static CLOCK_SETTINGS: [Setting; 5] = [
    text::BACKGROUND,
    text::COLOR,
    text::SHADOW,
    Setting::toggle("twelve_hour", "pvp.clock.twelve_hour", false),
    Setting::toggle("seconds", "pvp.clock.seconds", false),
];

fn clock_format(cx: &Cx) -> &'static str {
    match (cx.bool(TWELVE_HOUR), cx.bool(SECONDS)) {
        (false, false) => "%H:%M",
        (false, true) => "%H:%M:%S",
        (true, false) => "%I:%M %p",
        (true, true) => "%I:%M:%S %p",
    }
}

static CLOCK_LINE: Line = Line {
    id: "clock",
    label: "pvp.clock.line",
    pos: HudPos::new(Anchor::TopLeft, 4.0, 58.0),
    widest: |cx| {
        let f = clock_format(cx);
        chrono::NaiveTime::from_hms_opt(20, 58, 58)
            .map(|t| t.format(f).to_string())
            .unwrap_or_default()
    },
    text: |cx, _: &dyn ClientHost| Some(chrono::Local::now().format(clock_format(cx)).to_string()),
};
static CLOCK_HUD: [&dyn HudElement; 1] = [&CLOCK_LINE];
pub static CLOCK: Info = Info {
    id: "clock",
    name: "Clock",
    description: "pvp.clock.description",
    settings: &CLOCK_SETTINGS,
    hud: &CLOCK_HUD,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_round_and_name_the_axis() {
        assert_eq!(
            coords_text([1.26, 64.0, -3.5], 1, Some("east")),
            "1.3 64.0 -3.5 east (+X)"
        );
        assert_eq!(coords_text([1.26, 64.0, -3.5], 0, None), "1 64 -4");
        assert_eq!(facing(0.0), "south");
        assert_eq!(facing(-90.0), "east");
        assert_eq!(facing(181.0), "north");
        assert_eq!(facing(89.0), "west");
    }
}
