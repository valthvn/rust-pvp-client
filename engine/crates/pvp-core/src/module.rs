//! The `Module` trait: one PvP feature compiled into the client, switched and tuned from the menu.

use crate::hud::HudPos;
use crate::setting::{Setting, Value};
use mod_api::{
    ClientHost, ComputeFovModifierEvent, DrawContext, Flow, GuiLayerPreEvent, InputKeyEvent,
    RenderLevelStageEvent, RenderNameTagEvent,
};

/// The menu's categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Category {
    Hud,
    Pvp,
    Chat,
    Visual,
    Performance,
}

impl Category {
    pub const ALL: [Category; 5] = [
        Category::Hud,
        Category::Pvp,
        Category::Chat,
        Category::Visual,
        Category::Performance,
    ];

    /// Translation key of its name.
    pub fn label(self) -> &'static str {
        match self {
            Category::Hud => "pvp.category.hud",
            Category::Pvp => "pvp.category.pvp",
            Category::Chat => "pvp.category.chat",
            Category::Visual => "pvp.category.visual",
            Category::Performance => "pvp.category.performance",
        }
    }
}

/// The game events a module listens to while it is on. A module that is off, or doesn't list an event, is
/// never called for it; an event no active module lists costs one atomic load.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Events(pub u32);

impl Events {
    pub const NONE: Events = Events(0);
    /// `tick`, once per client tick (before the game's tick).
    pub const TICK: Events = Events(1 << 0);
    /// `key`, each key press, repeat and release.
    pub const KEY: Events = Events(1 << 1);
    /// `gui_layer`, before each vanilla HUD layer is drawn (to hide or replace one).
    pub const GUI_LAYER: Events = Events(1 << 2);
    /// `name_tag`, when the game decides an entity's name tag.
    pub const NAME_TAG: Events = Events(1 << 3);
    /// `level_stage`, to draw world geometry.
    pub const LEVEL_STAGE: Events = Events(1 << 4);
    /// `fov`, the field-of-view modifier, once per tick.
    pub const FOV: Events = Events(1 << 5);
    /// Its HUD elements are drawn (implied by a non-empty `hud()`).
    pub const HUD: Events = Events(1 << 6);
    /// Its keybinds are polled each tick (implied by a non-empty `keybinds()`).
    pub const KEYBINDS: Events = Events(1 << 7);
    /// `mouse`, each mouse button press and release while playing (after the game handled it).
    pub const MOUSE: Events = Events(1 << 8);

    pub const COUNT: usize = 9;

    pub const fn union(self, other: Events) -> Events {
        Events(self.0 | other.0)
    }
    pub const fn contains(self, other: Events) -> bool {
        self.0 & other.0 == other.0
    }
    /// The index of a single-event set, for the registry's tables.
    pub(crate) fn index(self) -> usize {
        self.0.trailing_zeros() as usize
    }
}

impl std::ops::BitOr for Events {
    type Output = Events;
    fn bitor(self, rhs: Events) -> Events {
        self.union(rhs)
    }
}

/// A key mapping a module owns. It shows in the game's Controls screen under the client's heading.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keybind {
    /// Unique within the module; the mapping's id is `<module id>.<id>`.
    pub id: &'static str,
    /// Translation key of its label.
    pub label: &'static str,
    /// Default key, as the game names it (`key.keyboard.r`), or `key.keyboard.unknown` for unbound.
    pub default_key: &'static str,
}

/// What a module's callbacks get besides the game: its own settings, read under one lock for the call.
pub struct Cx<'a> {
    values: &'a [Value],
    /// True while the HUD editor draws elements (draw something placeholder-like even with nothing to show).
    pub editing: bool,
}

impl<'a> Cx<'a> {
    pub(crate) fn new(values: &'a [Value], editing: bool) -> Self {
        Self { values, editing }
    }
    /// The value of setting `i` (the index in `Module::settings()`).
    pub fn get(&self, i: usize) -> &Value {
        &self.values[i]
    }
    pub fn bool(&self, i: usize) -> bool {
        self.values[i].as_bool()
    }
    pub fn f32(&self, i: usize) -> f32 {
        self.values[i].as_f32()
    }
    pub fn choice(&self, i: usize) -> usize {
        self.values[i].as_choice()
    }
    pub fn color(&self, i: usize) -> u32 {
        self.values[i].as_color()
    }
    pub fn str(&self, i: usize) -> &str {
        self.values[i].as_str()
    }
}

/// A movable HUD element of a module.
pub trait HudElement: Sync {
    /// Unique within the module; the key of its placement in profiles.
    fn id(&self) -> &'static str;
    /// Translation key of its name in the editor.
    fn label(&self) -> &'static str;
    fn default_pos(&self) -> HudPos;
    /// Its size in GUI pixels at scale 1, this frame.
    fn size(&self, cx: &Cx, host: &dyn ClientHost, draw: &dyn DrawContext) -> [f32; 2];
    /// Draws it with its top-left at `at`.
    fn draw(&self, cx: &Cx, host: &dyn ClientHost, draw: &mut dyn DrawContext, at: [f32; 2]);
}

/// One PvP feature. Modules are `static`s; they keep any state of their own in atomics or locks, never in the
/// game's structures. Every module starts off: a fresh install is vanilla.
pub trait Module: Sync + 'static {
    /// Stable id: the profile key and the id horizon-ctl and the menu use. Never translated.
    fn id(&self) -> &'static str;
    /// Translation keys of the name and the one-line description.
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> Category;
    /// Its settings, in menu order. A module refers to them by index.
    fn settings(&self) -> &'static [Setting] {
        &[]
    }
    /// The events it listens to while on (besides HUD and keybinds, implied by `hud` and `keybinds`).
    fn events(&self) -> Events {
        Events::NONE
    }
    fn keybinds(&self) -> &'static [Keybind] {
        &[]
    }
    fn hud(&self) -> &'static [&'static dyn HudElement] {
        &[]
    }

    fn on_enable(&self, _cx: &Cx) {}
    fn on_disable(&self, _cx: &Cx) {}
    /// Setting `index` changed (also called for each setting after a profile loads, while on).
    fn on_setting(&self, _index: usize, _cx: &Cx) {}
    /// Keybind `id` was pressed (one call per press).
    fn on_keybind(&self, _id: &str, _cx: &Cx, _host: &mut dyn ClientHost) {}

    fn tick(&self, _cx: &Cx, _host: &mut dyn ClientHost) {}
    fn key(&self, _cx: &Cx, _host: &mut dyn ClientHost, _event: &InputKeyEvent<'_>) {}
    /// Mouse button `button` (0 left, 1 right, 2 middle) was pressed or released.
    fn mouse(&self, _cx: &Cx, _host: &mut dyn ClientHost, _button: i32, _pressed: bool) {}
    fn gui_layer(
        &self,
        _cx: &Cx,
        _host: &dyn ClientHost,
        _event: &mut GuiLayerPreEvent<'_>,
    ) -> Flow {
        Flow::Continue
    }
    fn name_tag(&self, _cx: &Cx, _host: &dyn ClientHost, _event: &mut RenderNameTagEvent<'_>) {}
    fn level_stage(
        &self,
        _cx: &Cx,
        _host: &dyn ClientHost,
        _event: &mut RenderLevelStageEvent<'_>,
    ) {
    }
    fn fov(&self, _cx: &Cx, _host: &dyn ClientHost, _event: &mut ComputeFovModifierEvent<'_>) {}
}
