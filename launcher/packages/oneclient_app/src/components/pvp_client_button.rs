//! The home page's PvP client row: the module profile the game starts with, and "Play PvP Client"
//! (`crate::pvp_client`). Profiles are edited in game; here one is only picked.

use freya::prelude::*;

use crate::components::{Button, Dropdown};
use crate::hooks::use_dispatch;
use crate::theme::text_size;

#[derive(PartialEq)]
pub struct PvpClientButton;

impl Component for PvpClientButton {
    fn render(&self) -> impl IntoElement {
        let dispatch = use_dispatch();
        let instance = crate::pvp_client::instance_dir();
        let mut active = use_state(|| {
            instance.as_deref().map_or_else(
                || pvp_launch::profiles::DEFAULT.to_owned(),
                pvp_launch::profiles::active,
            )
        });
        let names = instance
            .as_deref()
            .map(pvp_launch::profiles::list)
            .unwrap_or_default();
        let picked = names.clone();
        rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .spacing(8.)
            .child(
                Dropdown::new(active.read().clone(), names)
                    .width(Size::px(160.))
                    .height(Size::px(36.))
                    .tooltip("Module profile")
                    .on_select(move |i: usize| {
                        if let (Some(dir), Some(name)) = (instance.as_deref(), picked.get(i)) {
                            match pvp_launch::profiles::set_active(dir, name) {
                                Ok(()) => active.set(name.clone()),
                                Err(e) => tracing::warn!("choosing profile {name}: {e}"),
                            }
                        }
                    }),
            )
            .child(
                Button::new()
                    .primary()
                    .medium()
                    .font_size(text_size(16.))
                    .font_weight(FontWeight::from(450))
                    .padding(Gaps::new_symmetric(8., 24.))
                    .on_press(move |_| dispatch.launch_pvp_client())
                    .text("Play PvP Client"),
            )
    }
}
