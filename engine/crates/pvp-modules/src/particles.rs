//! Particles (OverflowParticles): more or fewer hit particles, and hiding sweeps, damage hearts and
//! explosions. The engine asks [`scale`] for each particle it creates (Horizon's particle scale, installed by
//! `pvp-client`).

use pvp_core::{Category, Module, Setting, registry};
use std::sync::OnceLock;

pub struct Particles;
pub static PARTICLES: Particles = Particles;

const CRIT: usize = 0;
const ENCHANTED_HIT: usize = 1;
const SWEEP: usize = 2;
const DAMAGE_INDICATOR: usize = 3;
const EXPLOSIONS: usize = 4;
const OTHERS: usize = 5;

static SETTINGS: [Setting; 6] = [
    Setting::slider("crit", "pvp.particles.crit", 0.0, 10.0, 0.5, 1.0),
    Setting::slider(
        "enchanted_hit",
        "pvp.particles.enchanted_hit",
        0.0,
        10.0,
        0.5,
        1.0,
    ),
    Setting::toggle("sweep", "pvp.particles.sweep", true),
    Setting::toggle("damage_indicator", "pvp.particles.damage_indicator", true),
    Setting::toggle("explosions", "pvp.particles.explosions", true),
    Setting::slider("others", "pvp.particles.others", 0.0, 2.0, 0.1, 1.0),
];

/// Which setting a particle type follows, and whether it is a toggle (shown or not) or a multiplier.
fn setting_of(name: &str) -> (usize, bool) {
    match name {
        "crit" => (CRIT, false),
        "enchanted_hit" => (ENCHANTED_HIT, false),
        "sweep_attack" => (SWEEP, true),
        "damage_indicator" => (DAMAGE_INDICATOR, true),
        "explosion" | "explosion_emitter" => (EXPLOSIONS, true),
        _ => (OTHERS, false),
    }
}

/// How many of particle type `name` to show for one: 1.0 while the module is off.
pub fn scale(name: &str) -> f32 {
    static I: OnceLock<Option<usize>> = OnceLock::new();
    let Some(reg) = registry::get() else {
        return 1.0;
    };
    let Some(i) = *I.get_or_init(|| reg.index_of("particles")) else {
        return 1.0;
    };
    if !reg.is_enabled(i) {
        return 1.0;
    }
    let (s, toggle) = setting_of(name);
    let v = reg.value(i, s);
    if toggle {
        if v.as_bool() { 1.0 } else { 0.0 }
    } else {
        v.as_f32()
    }
}

impl Module for Particles {
    fn id(&self) -> &'static str {
        "particles"
    }
    fn name(&self) -> &'static str {
        "Particles"
    }
    fn description(&self) -> &'static str {
        "pvp.particles.description"
    }
    fn category(&self) -> Category {
        Category::Visual
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particle_types_follow_their_setting() {
        assert_eq!(setting_of("crit"), (CRIT, false));
        assert_eq!(setting_of("explosion_emitter"), (EXPLOSIONS, true));
        assert_eq!(setting_of("rain"), (OTHERS, false));
    }
}
