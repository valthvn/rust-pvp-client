//! Name tags (PolyNametag): your own tag in third person, and hiding other players' or other entities' tags.

use mod_api::{ClientHost, RenderNameTagEvent};
use pvp_core::{Category, Cx, Events, Module, Setting};

pub struct Nametags;
pub static NAMETAGS: Nametags = Nametags;

const OWN: usize = 0;
const HIDE_PLAYERS: usize = 1;
const HIDE_OTHERS: usize = 2;

static SETTINGS: [Setting; 3] = [
    Setting::toggle("own", "pvp.nametags.own", true),
    Setting::toggle("hide_players", "pvp.nametags.hide_players", false),
    Setting::toggle("hide_others", "pvp.nametags.hide_others", false),
];

/// Whether a tag shows, given what the game decided.
fn visible(
    vanilla: bool,
    own: bool,
    player: bool,
    show_own: bool,
    hide_players: bool,
    hide_others: bool,
) -> bool {
    if own {
        return show_own || vanilla;
    }
    if player && hide_players || !player && hide_others {
        return false;
    }
    vanilla
}

impl Module for Nametags {
    fn id(&self) -> &'static str {
        "nametags"
    }
    fn name(&self) -> &'static str {
        "Name Tags"
    }
    fn description(&self) -> &'static str {
        "pvp.nametags.description"
    }
    fn category(&self) -> Category {
        Category::Visual
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
    fn events(&self) -> Events {
        Events::NAME_TAG
    }
    fn name_tag(&self, cx: &Cx, host: &dyn ClientHost, event: &mut RenderNameTagEvent<'_>) {
        let own = host
            .local_player()
            .is_some_and(|p| p.entity().id() == event.entity_id);
        event.visible = visible(
            event.visible,
            own,
            event.kind == "minecraft:player",
            cx.bool(OWN),
            cx.bool(HIDE_PLAYERS),
            cx.bool(HIDE_OTHERS),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules() {
        assert!(
            visible(false, true, true, true, true, true),
            "own tag shown"
        );
        assert!(
            !visible(true, false, true, false, true, false),
            "players hidden"
        );
        assert!(visible(true, false, false, false, true, false), "mobs kept");
        assert!(
            !visible(true, false, false, false, false, true),
            "mobs hidden"
        );
        assert!(
            !visible(false, false, true, false, false, false),
            "vanilla kept"
        );
    }
}
