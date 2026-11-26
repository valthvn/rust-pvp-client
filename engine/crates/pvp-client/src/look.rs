//! Glue between the glint and damage tint modules and Horizon's frame override (the only engine knob for
//! these colours): the modules' colours become the texels the renderer reads.

use horizon_client_render::pvp::{Frame, set_frame_override};
use pvp_modules::look::{colours, glint_texel, hurt_texel};

fn apply(mut frame: Frame) -> Frame {
    let c = colours();
    if let Some(argb) = c.glint {
        frame.glint = glint_texel(argb);
    }
    if let Some((argb, opacity)) = c.hurt {
        frame.hurt_texel = hurt_texel(argb, opacity);
    }
    frame
}

pub fn install() {
    set_frame_override(apply);
}
