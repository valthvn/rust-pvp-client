//! The UI's own mod (`pvp_ui`): Right Shift opens its screen over the world, the screen hooks feed the
//! menu its input, and the UI atlas is sent to the game once a frame before drawing.

use crate::ctx::Input;
use crate::gfx::Ui;
use crate::hud_editor::Editor;
use crate::menu::Menu;
use mod_api::{
    ClientHost, ClientTickPreEvent, Flow, Hooks, KeyRegistration, ModEntry, Registrar,
    RegistrationContext, RenderFramePreEvent, ScreenCharTypedPreEvent, ScreenKeyPressedPreEvent,
    ScreenMouseButtonPressedPreEvent, ScreenMouseButtonReleasedPreEvent,
    ScreenMouseDraggedPreEvent, ScreenMouseScrolledPreEvent, ScreenRenderPostEvent,
};
use std::sync::Mutex;

pub const MOD_ID: &str = "pvp_ui";
/// The key mapping that opens the menu (`pvp_ui:menu`), Right Shift as in OneConfig.
const MENU_KEY: &str = "menu";
const MENU_DEFAULT: &str = "key.keyboard.right.shift";

pub fn entry() -> ModEntry {
    ModEntry::new(MOD_ID, Box::new(UiMod::default()))
}

#[derive(Default)]
struct UiMod {
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    ui: Ui,
    menu: Menu,
    editor: Editor,
    /// The screen shows the HUD editor instead of the menu.
    editing: bool,
}

impl UiMod {
    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
}

fn ours(host: &dyn ClientHost) -> bool {
    host.mod_screen() == Some(MOD_ID)
}

#[mod_api::horizon_mod]
impl Hooks for UiMod {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        let mut mapping = KeyRegistration::new(MENU_KEY, MENU_DEFAULT);
        mapping.label = "pvp.key.menu".to_owned();
        mapping.category = "pvp.controls".to_owned();
        r.register_key_mapping(mapping);
    }

    fn client_tick_pre(&self, host: &mut dyn ClientHost, _: &mut ClientTickPreEvent) {
        let mut opened = false;
        while host.consume_click(MOD_ID, MENU_KEY) {
            opened = true;
        }
        let mut st = self.state();
        // Our screen closed (Escape, another screen): leave the editor.
        if st.editing && !ours(host) {
            st.editing = false;
            pvp_core::runtime::set_editing(false);
        }
        if opened && host.mod_screen().is_none() && host.mod_screen_open(MOD_ID) {
            let State { ui, menu, .. } = &mut *st;
            menu.opened(ui);
        }
        if st.menu.take_edit_hud() {
            st.editing = true;
            st.editor.opened();
            pvp_core::runtime::set_editing(true);
        }
        if st.menu.take_close() {
            drop(st);
            host.mod_screen_close(MOD_ID);
        }
    }

    fn render_frame_pre(&self, host: &mut dyn ClientHost, _: &mut RenderFramePreEvent) {
        self.state().ui.sync(host);
    }

    fn screen_render_post(&self, host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
        if !ours(host) {
            return;
        }
        let mouse = event.screen.mouse;
        let mut st = self.state();
        let State {
            ui,
            menu,
            editor,
            editing,
        } = &mut *st;
        if *editing {
            editor.draw(ui, event.draw, host, mouse);
        } else {
            menu.draw(ui, event.draw, host, mouse);
        }
    }

    fn screen_mouse_button_pressed_pre(
        &self,
        host: &dyn ClientHost,
        e: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        self.input(host, e.screen.mouse, Input::Press(e.button))
    }

    fn screen_mouse_button_released_pre(
        &self,
        host: &dyn ClientHost,
        e: &mut ScreenMouseButtonReleasedPreEvent<'_>,
    ) -> Flow {
        self.input(host, e.screen.mouse, Input::Release(e.button))
    }

    fn screen_mouse_dragged_pre(
        &self,
        host: &dyn ClientHost,
        e: &mut ScreenMouseDraggedPreEvent<'_>,
    ) -> Flow {
        self.input(host, e.screen.mouse, Input::Drag(e.button, e.delta))
    }

    fn screen_mouse_scrolled_pre(
        &self,
        host: &dyn ClientHost,
        e: &mut ScreenMouseScrolledPreEvent<'_>,
    ) -> Flow {
        let s = [e.scroll[0] as f32, e.scroll[1] as f32];
        self.input(host, e.screen.mouse, Input::Scroll(s))
    }

    fn screen_key_pressed_pre(
        &self,
        host: &dyn ClientHost,
        e: &mut ScreenKeyPressedPreEvent<'_>,
    ) -> Flow {
        // Escape is left to the game, which closes the screen.
        if e.key == "key.keyboard.escape" {
            return Flow::Continue;
        }
        self.input(
            host,
            e.screen.mouse,
            Input::Key(e.key.to_owned(), e.modifiers),
        )
    }

    fn screen_char_typed_pre(
        &self,
        host: &dyn ClientHost,
        e: &mut ScreenCharTypedPreEvent<'_>,
    ) -> Flow {
        self.input(host, e.screen.mouse, Input::Char(e.c))
    }
}

impl UiMod {
    /// Gives the menu `input`; the game sees none of it while our screen is open.
    fn input(&self, host: &dyn ClientHost, mouse: [f32; 2], input: Input) -> Flow {
        if !ours(host) {
            return Flow::Continue;
        }
        let mut st = self.state();
        if st.editing {
            st.editor.input(input, mouse);
        } else {
            st.menu.input(input);
        }
        Flow::Cancel
    }
}
