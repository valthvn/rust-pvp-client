//! The registry of compiled-in modules: their on/off state, settings and HUD placements, and, per event,
//! a bitmask of the modules that are on and listen to it. Dispatch reads one atomic; switching a module
//! rewrites the masks (never per frame).

use crate::hud::HudPos;
use crate::module::{Cx, Events, Module};
use crate::setting::Value;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{OnceLock, RwLock};

/// At most 64 modules (one bit each in the event masks).
pub const MAX_MODULES: usize = 64;

pub struct ModuleState {
    enabled: AtomicBool,
    values: RwLock<Vec<Value>>,
    hud: RwLock<Vec<HudPos>>,
}

pub struct Registry {
    modules: &'static [&'static dyn Module],
    states: Vec<ModuleState>,
    masks: [AtomicU64; Events::COUNT],
    /// A setting, a switch or a placement changed since the profile was last saved.
    dirty: AtomicBool,
}

static REGISTRY: OnceLock<Registry> = OnceLock::new();

/// Builds the registry from the client's modules, all off. Once, before the game starts.
pub fn init(modules: &'static [&'static dyn Module]) -> &'static Registry {
    REGISTRY.get_or_init(|| Registry::new(modules))
}

/// The registry, once `init` ran.
pub fn get() -> Option<&'static Registry> {
    REGISTRY.get()
}

impl Registry {
    pub fn new(modules: &'static [&'static dyn Module]) -> Self {
        assert!(
            modules.len() <= MAX_MODULES,
            "at most {MAX_MODULES} modules"
        );
        for (i, m) in modules.iter().enumerate() {
            assert!(
                modules[..i].iter().all(|o| o.id() != m.id()),
                "two modules have the id {}",
                m.id()
            );
        }
        let states = modules
            .iter()
            .map(|m| ModuleState {
                enabled: AtomicBool::new(false),
                values: RwLock::new(m.settings().iter().map(|s| s.default_value()).collect()),
                hud: RwLock::new(m.hud().iter().map(|h| h.default_pos()).collect()),
            })
            .collect();
        Self {
            modules,
            states,
            masks: Default::default(),
            dirty: AtomicBool::new(false),
        }
    }

    pub fn modules(&self) -> &'static [&'static dyn Module] {
        self.modules
    }

    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.modules.iter().position(|m| m.id() == id)
    }

    pub fn is_enabled(&self, i: usize) -> bool {
        self.states[i].enabled.load(Ordering::Acquire)
    }

    /// The events module `i` takes while on, HUD and keybinds included.
    fn events_of(&self, i: usize) -> Events {
        let m = self.modules[i];
        let mut e = m.events();
        if !m.hud().is_empty() {
            e = e | Events::HUD;
        }
        if !m.keybinds().is_empty() {
            e = e | Events::KEYBINDS;
        }
        e
    }

    /// Switches module `i`, calling its `on_enable` or `on_disable`. A module that panics there stays off.
    pub fn set_enabled(&self, i: usize, on: bool) {
        if self.states[i].enabled.swap(on, Ordering::AcqRel) == on {
            return;
        }
        self.dirty.store(true, Ordering::Release);
        let m = self.modules[i];
        let ok = self.call(i, false, |cx| {
            if on {
                m.on_enable(cx)
            } else {
                m.on_disable(cx)
            }
        });
        if on && !ok {
            self.states[i].enabled.store(false, Ordering::Release);
        }
        self.rebuild_masks();
    }

    fn rebuild_masks(&self) {
        for (bit, mask) in self.masks.iter().enumerate() {
            let event = Events(1 << bit);
            let mut v = 0u64;
            for i in 0..self.modules.len() {
                if self.is_enabled(i) && self.events_of(i).contains(event) {
                    v |= 1 << i;
                }
            }
            mask.store(v, Ordering::Release);
        }
    }

    /// The modules that are on and listen to `event`, as a bitmask (0: nothing to do).
    #[inline]
    pub fn mask(&self, event: Events) -> u64 {
        self.masks[event.index()].load(Ordering::Acquire)
    }

    /// Calls `f` for each module that is on and listens to `event`, in registration order.
    #[inline]
    pub fn each(&self, event: Events, mut f: impl FnMut(&'static dyn Module, &Cx)) {
        let mut mask = self.mask(event);
        while mask != 0 {
            let i = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            let m = self.modules[i];
            self.call(i, false, |cx| f(m, cx));
        }
    }

    /// Runs `f` with module `i`'s settings. A panic switches the module off (logged) instead of
    /// taking the game down. Returns whether `f` finished.
    pub fn call(&self, i: usize, editing: bool, f: impl FnOnce(&Cx)) -> bool {
        let values = self.states[i]
            .values
            .read()
            .unwrap_or_else(|e| e.into_inner());
        let cx = Cx::new(&values, editing);
        let ok = catch_unwind(AssertUnwindSafe(|| f(&cx))).is_ok();
        drop(values);
        if !ok {
            log::error!(
                "pvp: module {} panicked and was switched off",
                self.modules[i].id()
            );
            self.states[i].enabled.store(false, Ordering::Release);
            self.rebuild_masks();
        }
        ok
    }

    pub fn value(&self, i: usize, setting: usize) -> Value {
        self.states[i]
            .values
            .read()
            .unwrap_or_else(|e| e.into_inner())[setting]
            .clone()
    }

    pub fn values(&self, i: usize) -> Vec<Value> {
        self.states[i]
            .values
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Sets a setting (made valid for it) and tells the module if it is on.
    pub fn set_value(&self, i: usize, setting: usize, value: Value) {
        let spec = &self.modules[i].settings()[setting];
        let value = spec.sanitize(value);
        {
            let mut values = self.states[i]
                .values
                .write()
                .unwrap_or_else(|e| e.into_inner());
            if values[setting] == value {
                return;
            }
            values[setting] = value;
        }
        self.dirty.store(true, Ordering::Release);
        if self.is_enabled(i) {
            let m = self.modules[i];
            self.call(i, false, |cx| m.on_setting(setting, cx));
        }
    }

    pub fn hud_pos(&self, i: usize, element: usize) -> HudPos {
        self.states[i].hud.read().unwrap_or_else(|e| e.into_inner())[element]
    }

    pub fn set_hud_pos(&self, i: usize, element: usize, pos: HudPos) {
        let pos = HudPos {
            scale: pos.scale.clamp(0.5, 3.0),
            ..pos
        };
        self.states[i]
            .hud
            .write()
            .unwrap_or_else(|e| e.into_inner())[element] = pos;
        self.dirty.store(true, Ordering::Release);
    }

    /// Back to the defaults: every module off, every setting and placement default.
    pub fn reset(&self) {
        for i in 0..self.modules.len() {
            self.set_enabled(i, false);
            let m = self.modules[i];
            *self.states[i]
                .values
                .write()
                .unwrap_or_else(|e| e.into_inner()) =
                m.settings().iter().map(|s| s.default_value()).collect();
            *self.states[i]
                .hud
                .write()
                .unwrap_or_else(|e| e.into_inner()) =
                m.hud().iter().map(|h| h.default_pos()).collect();
        }
        self.dirty.store(true, Ordering::Release);
    }

    /// Whether something changed since the last call (and clears the flag).
    pub fn take_dirty(&self) -> bool {
        self.dirty.swap(false, Ordering::AcqRel)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::module::{Category, Keybind};
    use crate::setting::Setting;
    use std::sync::atomic::AtomicU32;

    pub struct Probe {
        pub id: &'static str,
        pub events: Events,
        pub ticks: AtomicU32,
        pub enables: AtomicU32,
        pub last_setting: AtomicU32,
        pub panic_on_tick: bool,
    }

    impl Probe {
        pub const fn new(id: &'static str, events: Events, panic_on_tick: bool) -> Self {
            Self {
                id,
                events,
                ticks: AtomicU32::new(0),
                enables: AtomicU32::new(0),
                last_setting: AtomicU32::new(u32::MAX),
                panic_on_tick,
            }
        }
    }

    pub static PROBE_SETTINGS: [Setting; 2] = [
        Setting::toggle("flag", "x", false),
        Setting::slider("size", "x", 1.0, 10.0, 1.0, 4.0),
    ];
    static KEYS: [Keybind; 1] = [Keybind {
        id: "k",
        label: "x",
        default_key: "key.keyboard.r",
    }];

    impl Module for Probe {
        fn id(&self) -> &'static str {
            self.id
        }
        fn name(&self) -> &'static str {
            "x"
        }
        fn description(&self) -> &'static str {
            "x"
        }
        fn category(&self) -> Category {
            Category::Pvp
        }
        fn settings(&self) -> &'static [Setting] {
            &PROBE_SETTINGS
        }
        fn events(&self) -> Events {
            self.events
        }
        fn keybinds(&self) -> &'static [Keybind] {
            if self.id == "keyed" { &KEYS } else { &[] }
        }
        fn on_enable(&self, _: &Cx) {
            self.enables.fetch_add(1, Ordering::SeqCst);
        }
        fn on_setting(&self, i: usize, _: &Cx) {
            self.last_setting.store(i as u32, Ordering::SeqCst);
        }
    }

    fn tick_all(r: &Registry) {
        r.each(Events::TICK, |m, _| {
            let p = probe(m);
            if p.panic_on_tick {
                panic!("boom");
            }
            p.ticks.fetch_add(1, Ordering::SeqCst);
        });
    }

    fn probe(m: &'static dyn Module) -> &'static Probe {
        // Tests only register `Probe`s; find the static by id.
        for p in [&A, &B, &C, &D, &E] {
            if p.id == m.id() {
                return p;
            }
        }
        unreachable!()
    }

    static A: Probe = Probe::new("a", Events::TICK, false);
    static B: Probe = Probe::new("b", Events::KEY, false);
    static C: Probe = Probe::new("c", Events::TICK, true);
    static D: Probe = Probe::new("d", Events::TICK, false);
    static E: Probe = Probe::new("keyed", Events::NONE, false);

    #[test]
    fn nothing_is_called_while_every_module_is_off() {
        static MODS: [&dyn Module; 1] = [&D];
        let r = Registry::new(&MODS);
        tick_all(&r);
        assert_eq!(r.mask(Events::TICK), 0);
        assert_eq!(D.ticks.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn only_modules_that_are_on_and_listen_are_called() {
        static MODS: [&dyn Module; 2] = [&A, &B];
        let r = Registry::new(&MODS);
        r.set_enabled(0, true);
        r.set_enabled(1, true);
        assert_eq!(A.enables.load(Ordering::SeqCst), 1);
        tick_all(&r);
        tick_all(&r);
        assert_eq!(A.ticks.load(Ordering::SeqCst), 2);
        assert_eq!(B.ticks.load(Ordering::SeqCst), 0);
        r.set_enabled(0, false);
        tick_all(&r);
        assert_eq!(A.ticks.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn a_module_that_panics_is_switched_off_and_the_game_goes_on() {
        static MODS: [&dyn Module; 1] = [&C];
        let r = Registry::new(&MODS);
        r.set_enabled(0, true);
        tick_all(&r);
        assert!(!r.is_enabled(0));
        assert_eq!(r.mask(Events::TICK), 0);
    }

    #[test]
    fn settings_are_made_valid_and_reach_the_module() {
        static MODS: [&dyn Module; 1] = [&E];
        let r = Registry::new(&MODS);
        assert_ne!(
            r.mask(Events::KEYBINDS),
            1,
            "keybinds are polled only while on"
        );
        r.set_enabled(0, true);
        assert_eq!(r.mask(Events::KEYBINDS), 1);
        assert!(r.take_dirty());
        r.set_value(0, 1, Value::Num(42.0));
        assert_eq!(r.value(0, 1), Value::Num(10.0));
        assert_eq!(E.last_setting.load(Ordering::SeqCst), 1);
        assert!(r.take_dirty());
        r.set_value(0, 1, Value::Num(10.0));
        assert!(!r.take_dirty(), "an unchanged value is not a change");
    }
}
