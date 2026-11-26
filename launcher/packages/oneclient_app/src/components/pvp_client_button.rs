//! The home page's "Play PvP Client" button: starts the native client (`crate::pvp_client`).

use freya::prelude::*;

use crate::components::Button;
use crate::hooks::use_dispatch;
use crate::theme::text_size;

#[derive(PartialEq)]
pub struct PvpClientButton;

impl Component for PvpClientButton {
    fn render(&self) -> impl IntoElement {
        let dispatch = use_dispatch();
        Button::new()
            .primary()
            .medium()
            .font_size(text_size(16.))
            .font_weight(FontWeight::from(450))
            .padding(Gaps::new_symmetric(8., 24.))
            .on_press(move |_| dispatch.launch_pvp_client())
            .text("Play PvP Client")
    }
}
