//! Chat (Chatting): timestamps, repeated messages compacted with their count, a click copies a message,
//! Ctrl+F searches the chat. Horizon's chat already does all four behind its own options; this module
//! turns them on from our menu through the engine's chat override (installed by `pvp-client`).

use pvp_core::{Category, Module, Setting, registry};
use std::sync::OnceLock;

pub struct Chat;
pub static CHAT: Chat = Chat;

static SETTINGS: [Setting; 4] = [
    Setting::toggle("timestamps", "pvp.chat.timestamps", true),
    Setting::toggle("compact", "pvp.chat.compact", true),
    Setting::toggle("copy", "pvp.chat.copy", true),
    Setting::toggle("search", "pvp.chat.search", true),
];

/// The chat behaviours the module asks for, `None` while it is off (the game's own options apply).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChatWanted {
    pub timestamps: bool,
    pub compact: bool,
    pub copy: bool,
    pub search: bool,
}

pub fn wanted() -> Option<ChatWanted> {
    static I: OnceLock<Option<usize>> = OnceLock::new();
    let reg = registry::get()?;
    let i = (*I.get_or_init(|| reg.index_of("chat")))?;
    if !reg.is_enabled(i) {
        return None;
    }
    let b = |s| reg.value(i, s).as_bool();
    Some(ChatWanted {
        timestamps: b(0),
        compact: b(1),
        copy: b(2),
        search: b(3),
    })
}

impl Module for Chat {
    fn id(&self) -> &'static str {
        "chat"
    }
    fn name(&self) -> &'static str {
        "Chat"
    }
    fn description(&self) -> &'static str {
        "pvp.chat.description"
    }
    fn category(&self) -> Category {
        Category::Chat
    }
    fn settings(&self) -> &'static [Setting] {
        &SETTINGS
    }
}
