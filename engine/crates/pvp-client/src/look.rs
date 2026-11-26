//! Glue between modules and the engine knobs that have no mod API (Horizon's frame, chat and particle
//! overrides): each knob asks the modules on every use, and gives the game's own value back while they are off.

use horizon_client_render::particles::ParticleEngine;
use horizon_client_render::pvp::{
    ChatOptions, Combat, Frame, set_chat_override, set_combat_override, set_frame_override,
};
use pvp_modules::look::{colours, glint_texel, hurt_texel};

fn frame(mut frame: Frame) -> Frame {
    let c = colours();
    if let Some(argb) = c.glint {
        frame.glint = glint_texel(argb);
    }
    if let Some((argb, opacity)) = c.hurt {
        frame.hurt_texel = hurt_texel(argb, opacity);
    }
    frame
}

fn chat(chat: ChatOptions) -> ChatOptions {
    match pvp_modules::chat::wanted() {
        Some(w) => ChatOptions {
            timestamps: w.timestamps,
            compact: w.compact,
            copy: w.copy,
            search: w.search,
        },
        None => chat,
    }
}

fn combat(combat: Combat) -> Combat {
    match pvp_modules::animations::wanted() {
        Some(w) => Combat {
            legacy: w.legacy,
            indicator: w.indicator,
            sounds: w.sounds,
            ..combat
        },
        None => combat,
    }
}

fn particles(name: &'static str) -> f32 {
    pvp_modules::particles::scale(name)
}

pub fn install() {
    set_frame_override(frame);
    set_chat_override(chat);
    set_combat_override(combat);
    ParticleEngine::set_scale(particles);
}
