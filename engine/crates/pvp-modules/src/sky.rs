//! Client time and weather (PolyTime, PolyWeather): the time of day and the weather this player sees. Only
//! the picture changes; the world, gameplay and the server keep theirs, and nothing is sent.
//!
//! The game asks for them only when connected to a server: in single player it draws the world it ticks
//! (Horizon's own PvP & HUD page covers single player).

use mod_api::{ClientHost, SkyEnvironmentEvent};
use pvp_core::{Category, Cx, Events, Module, Setting};

pub struct Sky;
pub static SKY: Sky = Sky;

const TIME: usize = 0;
const TIME_OF_DAY: usize = 1;
const WEATHER: usize = 2;

static SETTINGS: [Setting; 3] = [
    Setting::toggle("time", "pvp.sky.time", true),
    Setting::slider(
        "time_of_day",
        "pvp.sky.time_of_day",
        0.0,
        23_900.0,
        100.0,
        6_000.0,
    ),
    Setting::choice(
        "weather",
        "pvp.sky.weather",
        &["world", "clear", "rain", "thunder"],
        1,
    ),
];

/// `time` (0..24000) on the same day as `world` (the level's day time, which counts days), so the moon
/// phase stays the world's.
fn same_day(world: i64, time: i64) -> i64 {
    world.div_euclid(24_000) * 24_000 + time.rem_euclid(24_000)
}

/// The rain and thunder levels of a weather choice; `None` keeps the world's.
fn levels(choice: usize) -> Option<(f32, f32)> {
    match choice {
        1 => Some((0.0, 0.0)),
        2 => Some((1.0, 0.0)),
        3 => Some((1.0, 1.0)),
        _ => None,
    }
}

impl Module for Sky {
    fn id(&self) -> &'static str {
        "sky"
    }
    fn name(&self) -> &'static str {
        "Time & Weather"
    }
    fn description(&self) -> &'static str {
        "pvp.sky.description"
    }
    fn category(&self) -> Category {
        Category::Visual
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::SKY
    }
    fn sky(&self, cx: &Cx, _: &dyn ClientHost, event: &mut SkyEnvironmentEvent) {
        if cx.bool(TIME) {
            event.day_time = same_day(event.day_time, cx.f32(TIME_OF_DAY) as i64);
        }
        if let Some((rain, thunder)) = levels(cx.choice(WEATHER)) {
            event.rain = rain;
            event.thunder = thunder;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_keeps_the_day_and_weather_maps_to_levels() {
        assert_eq!(same_day(42_000, 6_000), 30_000);
        assert_eq!(same_day(5, 18_000), 18_000);
        assert_eq!(levels(0), None);
        assert_eq!(levels(3), Some((1.0, 1.0)));
    }
}
