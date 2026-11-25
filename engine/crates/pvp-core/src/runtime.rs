//! The bridge to Horizon's mod API: one statically linked mod (`pvp`) whose hooks hand each game event to the
//! modules that are on and listen to it. With every module off, each hook is one atomic load.

use crate::config;
use crate::module::Events;
use crate::registry::{self, Registry};
use mod_api::{
    ClientHost, ClientStoppingEvent, ClientTickPreEvent, ComputeFovModifierEvent, Flow,
    GuiLayerPreEvent, GuiRenderPostEvent, Hooks, InputKeyEvent, KeyRegistration, ModEntry,
    Registrar, RegistrationContext, RenderLevelStageEvent, RenderNameTagEvent,
};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// The mod id the client's modules run under (key mappings are `pvp:<module>.<key>`).
pub const MOD_ID: &str = "pvp";

/// How long after the last change the profile is written.
const SAVE_DELAY: Duration = Duration::from_millis(500);

/// The game directory, known from the first tick.
static GAME_DIR: OnceLock<PathBuf> = OnceLock::new();
/// The loaded profile's name.
static PROFILE: Mutex<String> = Mutex::new(String::new());
/// When the pending save is due.
static SAVE_AT: Mutex<Option<Instant>> = Mutex::new(None);
/// While true the HUD editor draws the elements itself.
static EDITING: AtomicBool = AtomicBool::new(false);

pub fn game_dir() -> Option<&'static PathBuf> {
    GAME_DIR.get()
}

pub fn profile() -> String {
    PROFILE.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

pub fn set_editing(on: bool) {
    EDITING.store(on, Ordering::Release);
}

pub fn editing() -> bool {
    EDITING.load(Ordering::Acquire)
}

/// Switches to profile `name` (saving the current one first).
pub fn switch_profile(name: &str) -> Result<(), config::ConfigError> {
    let (Some(reg), Some(dir)) = (registry::get(), GAME_DIR.get()) else {
        return Err(config::ConfigError::Missing);
    };
    save_now(reg);
    config::load(reg, dir, name)?;
    *PROFILE.lock().unwrap_or_else(|e| e.into_inner()) = name.to_owned();
    Ok(())
}

fn save_now(reg: &Registry) {
    let Some(dir) = GAME_DIR.get() else { return };
    let name = profile();
    if name.is_empty() {
        return;
    }
    reg.take_dirty();
    if let Err(e) = config::save(reg, dir, &name) {
        log::warn!("pvp: saving profile {name}: {e}");
    }
}

/// Saves off the render thread, `SAVE_DELAY` after the last change.
fn save_when_due(reg: &'static Registry) {
    let mut at = SAVE_AT.lock().unwrap_or_else(|e| e.into_inner());
    if reg.take_dirty() {
        *at = Some(Instant::now() + SAVE_DELAY);
    }
    if at.is_some_and(|t| Instant::now() >= t) {
        *at = None;
        let (Some(dir), name) = (GAME_DIR.get(), profile()) else {
            return;
        };
        let text = config::to_table(reg).to_string();
        let path = dir.clone();
        std::thread::spawn(move || {
            let file = config::root(&path)
                .join("profiles")
                .join(format!("{name}.toml"));
            let tmp = file.with_extension("toml.tmp");
            let result = std::fs::create_dir_all(file.parent().unwrap_or(&path))
                .and_then(|()| std::fs::write(&tmp, text))
                .and_then(|()| std::fs::rename(&tmp, &file));
            if let Err(e) = result {
                log::warn!("pvp: saving profile {name}: {e}");
            }
        });
    }
}

/// The mod the client executable passes to `horizon_mod_host::main_with`.
pub fn entry() -> ModEntry {
    ModEntry::new(MOD_ID, Box::new(Runtime))
}

#[derive(Default)]
struct Runtime;

impl Runtime {
    fn reg() -> Option<&'static Registry> {
        registry::get()
    }
}

#[mod_api::horizon_mod]
impl Hooks for Runtime {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        let Some(reg) = Self::reg() else { return };
        for m in reg.modules() {
            for k in m.keybinds() {
                let mut mapping =
                    KeyRegistration::new(&format!("{}.{}", m.id(), k.id), k.default_key);
                mapping.label = k.label.to_owned();
                mapping.category = "pvp.controls".to_owned();
                r.register_key_mapping(mapping);
            }
        }
    }

    fn client_tick_pre(&self, host: &mut dyn ClientHost, _: &mut ClientTickPreEvent) {
        let Some(reg) = Self::reg() else { return };
        if GAME_DIR.get().is_none() {
            let dir = host.instance_dir();
            let _ = GAME_DIR.set(dir.clone());
            let name = config::active(&dir);
            if let Err(e) = config::load(reg, &dir, &name) {
                log::warn!("pvp: loading profile {name}: {e}");
            }
            *PROFILE.lock().unwrap_or_else(|e| e.into_inner()) = name;
        }
        if reg.mask(Events::KEYBINDS) != 0 {
            let mut mask = reg.mask(Events::KEYBINDS);
            while mask != 0 {
                let i = mask.trailing_zeros() as usize;
                mask &= mask - 1;
                let m = reg.modules()[i];
                for k in m.keybinds() {
                    let id = format!("{}.{}", m.id(), k.id);
                    while host.consume_click(MOD_ID, &id) {
                        reg.call(i, false, |cx| m.on_keybind(k.id, cx, &mut *host));
                    }
                }
            }
        }
        reg.each(Events::TICK, |m, cx| m.tick(cx, &mut *host));
        save_when_due(reg);
    }

    fn input_key(&self, host: &mut dyn ClientHost, event: &mut InputKeyEvent<'_>) {
        let Some(reg) = Self::reg() else { return };
        reg.each(Events::KEY, |m, cx| m.key(cx, &mut *host, event));
    }

    fn gui_layer_pre(&self, host: &dyn ClientHost, event: &mut GuiLayerPreEvent<'_>) -> Flow {
        let Some(reg) = Self::reg() else {
            return Flow::Continue;
        };
        let mut flow = Flow::Continue;
        reg.each(Events::GUI_LAYER, |m, cx| {
            if matches!(m.gui_layer(cx, host, event), Flow::Cancel) {
                flow = Flow::Cancel;
            }
        });
        flow
    }

    fn gui_render_post(&self, host: &dyn ClientHost, event: &mut GuiRenderPostEvent<'_>) {
        let Some(reg) = Self::reg() else { return };
        let mut mask = reg.mask(Events::HUD);
        if mask == 0 || event.gui_hidden || editing() {
            return;
        }
        let screen = event.batch.size();
        while mask != 0 {
            let i = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            let m = reg.modules()[i];
            for (e, el) in m.hud().iter().enumerate() {
                let pos = reg.hud_pos(i, e);
                let batch = &mut *event.batch;
                reg.call(i, false, |cx| {
                    let size = el.size(cx, host, batch);
                    let at = pos.top_left(size, screen);
                    el.draw(cx, host, batch, at);
                });
            }
        }
    }

    fn render_name_tag(&self, host: &dyn ClientHost, event: &mut RenderNameTagEvent<'_>) {
        let Some(reg) = Self::reg() else { return };
        reg.each(Events::NAME_TAG, |m, cx| m.name_tag(cx, host, event));
    }

    fn render_level_stage(&self, host: &dyn ClientHost, event: &mut RenderLevelStageEvent<'_>) {
        let Some(reg) = Self::reg() else { return };
        reg.each(Events::LEVEL_STAGE, |m, cx| m.level_stage(cx, host, event));
    }

    fn compute_fov_modifier(&self, host: &dyn ClientHost, event: &mut ComputeFovModifierEvent<'_>) {
        let Some(reg) = Self::reg() else { return };
        reg.each(Events::FOV, |m, cx| m.fov(cx, host, event));
    }

    fn client_stopping(&self, _: &mut dyn ClientHost, _: &mut ClientStoppingEvent) {
        if let Some(reg) = Self::reg() {
            save_now(reg);
        }
    }
}
