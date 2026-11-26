//! The module menu: OneConfig's shell (`research/oneconfig/ui-spec.md` §4-§6) with our modules as its mods.
//!
//! The window is OneConfig's fixed 1391 x 700 dp, scaled to fit 90 % of the screen (never above 1 dp per
//! physical pixel), opened with its 600 ms ease-out-expo fade and zoom. The sidebar holds the pages (Mods,
//! Profiles, Keybinds) and Edit HUD; the Mods page is the card grid; a card opens its module's page of
//! option cards.

use crate::ctx::{self, Anims, Frame, Input, ease_out_expo, id};
use crate::gfx::Ui;
use crate::theme::{self as t, alpha, over, with_alpha};
use crate::widgets::{W, key_name};
use mod_api::ClientHost;
use pvp_core::{Category, Kind, Registry, Setting, Value, registry, runtime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Page {
    Mods,
    Module(usize),
    Profiles,
    Keybinds,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Focus {
    Search,
    /// A text setting (module, setting).
    Text(usize, usize),
    /// A key setting being recorded.
    Key(usize, usize),
    /// The new profile's name.
    ProfileName,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Popup {
    /// A choice setting's menu, under its field `anchor` (dp).
    Select {
        module: usize,
        setting: usize,
        anchor: [f32; 4],
    },
    /// A colour setting's picker, under its field.
    Color {
        module: usize,
        setting: usize,
        anchor: [f32; 4],
    },
}

pub struct Menu {
    queue: Vec<Input>,
    frame: Frame,
    anims: Anims,
    page: Page,
    history: Vec<Page>,
    category: Option<Category>,
    search: String,
    text: String,
    focus: Option<Focus>,
    popup: Option<Popup>,
    scroll: f32,
    scroll_max: f32,
    opened_at: f32,
    last: f32,
    close: bool,
    /// The colour picker's dragged part (0 pane, 1 hue, 2 alpha).
    picking: Option<u8>,
    /// Hue kept while the picked colour is grey (where hue is lost).
    hue: f32,
    edit_hud: bool,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            queue: Vec::new(),
            frame: Frame::default(),
            anims: Anims::default(),
            page: Page::Mods,
            history: Vec::new(),
            category: None,
            search: String::new(),
            text: String::new(),
            focus: None,
            popup: None,
            scroll: 0.0,
            scroll_max: 0.0,
            opened_at: -1.0,
            last: 0.0,
            close: false,
            picking: None,
            hue: 0.0,
            edit_hud: false,
        }
    }
}

/// Category chips and the icon a module card shows for its category.
fn category_icon(c: Category) -> &'static str {
    match c {
        Category::Hud => "hud",
        Category::Pvp => "combat",
        Category::Chat => "info",
        Category::Visual => "paintbrush",
        Category::Performance => "lightning-01",
    }
}

/// A translation key's text, or a readable fallback made from its last segment.
fn tr(host: &dyn ClientHost, key: &str) -> String {
    if let Some(s) = host.translate(key) {
        return s;
    }
    let last = key.rsplit('.').next().unwrap_or(key).replace('_', " ");
    let mut c = last.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

impl Menu {
    pub fn opened(&mut self, ui: &Ui) {
        self.close = false;
        self.opened_at = ui.now;
        self.popup = None;
        self.focus = None;
        self.queue.clear();
    }

    pub fn take_close(&mut self) -> bool {
        std::mem::take(&mut self.close)
    }

    /// The menu asked for the HUD editor (Edit HUD); taken.
    pub fn take_edit_hud(&mut self) -> bool {
        std::mem::take(&mut self.edit_hud)
    }

    pub fn input(&mut self, input: Input) {
        self.queue.push(input);
    }

    /// Whether the open screen's keys belong to a field (Escape still closes the menu then).
    pub fn typing(&self) -> bool {
        self.focus.is_some()
    }

    /// Physical pixels per dp for a window of `window` physical pixels (`OneConfigInterface.kt:166-175`).
    pub fn scale_for(window: [f32; 2]) -> f32 {
        (0.9 * window[0] / t::WINDOW_W)
            .min(0.9 * window[1] / t::WINDOW_H)
            .clamp(0.25, 1.0)
    }

    fn navigate(&mut self, page: Page) {
        if page != self.page {
            self.history.push(self.page);
            self.page = page;
            self.scroll = 0.0;
            self.popup = None;
            self.focus = None;
        }
    }

    /// Draws the menu over a screen of `gui` GUI pixels, the mouse at `mouse` (GUI pixels).
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        draw: &mut dyn mod_api::DrawContext,
        host: &dyn ClientHost,
        mouse: [f32; 2],
    ) {
        let Some(reg) = registry::get() else { return };
        let gui = draw.size();
        let window = [gui[0] * ui.gui_scale, gui[1] * ui.gui_scale];
        let open = ease_out_expo(((ui.now - self.opened_at) / 0.6).clamp(0.0, 1.0));
        let phys = Self::scale_for(window) * (0.9 + 0.1 * open);
        let k = phys / ui.gui_scale;
        let origin = [
            (gui[0] - t::WINDOW_W * k) / 2.0,
            (gui[1] - t::WINDOW_H * k) / 2.0,
        ];
        let dt = ui.now - self.last;
        self.last = ui.now;
        self.record_keys(reg);
        draw.blur_behind();
        let mut p = ui.painter_scaled(draw, phys, origin);
        p.opacity = open;
        let mouse = p.to_units(mouse);
        let mut frame = std::mem::take(&mut self.frame);
        let mut anims = std::mem::take(&mut self.anims);
        frame.begin(&mut self.queue, mouse, dt);
        let mut w = W {
            p: &mut p,
            f: &mut frame,
            a: &mut anims,
        };
        // Popups are on top: they take the clicks first, then draw last.
        let popup_rect = self.popup.map(|pop| popup_rect(&pop, reg));
        let over_popup = popup_rect.is_some_and(|r| w.f.hover(r));
        let (press, press_right) = (w.f.press, w.f.press_right);
        if over_popup {
            w.f.press = false;
            w.f.press_right = false;
        } else if w.f.press && self.popup.is_some() {
            // A click outside the popup closes it and does nothing else.
            self.popup = None;
            w.f.press = false;
        }
        let scroll_in = std::mem::take(&mut w.f.scroll);
        shell(self, &mut w, reg, host);
        if let Some(pop) = self.popup {
            w.f.press = press && over_popup;
            w.f.press_right = press_right && over_popup;
            draw_popup(self, &mut w, reg, pop);
        }
        if !over_popup {
            self.scroll = (self.scroll - scroll_in * 40.0).clamp(0.0, self.scroll_max);
        }
        if w.f.press && self.focus.is_some() {
            // A click on nothing ends typing.
            self.focus = None;
        }
        frame.end();
        self.frame = frame;
        self.anims = anims;
    }

    /// A key setting being recorded takes the next key or mouse button.
    fn record_keys(&mut self, reg: &Registry) {
        let Some(Focus::Key(m, s)) = self.focus else {
            return;
        };
        let mut taken = None;
        self.queue.retain(|i| match i {
            Input::Key(k, _) if taken.is_none() => {
                taken = Some(
                    if k == "key.keyboard.backspace" || k == "key.keyboard.delete" {
                        String::new()
                    } else {
                        k.clone()
                    },
                );
                false
            }
            Input::Press(b) if taken.is_none() => {
                taken = Some(match b {
                    0 => "key.mouse.left".to_owned(),
                    1 => "key.mouse.right".to_owned(),
                    2 => "key.mouse.middle".to_owned(),
                    n => format!("key.mouse.{}", n + 1),
                });
                false
            }
            Input::Release(_) => taken.is_none(),
            _ => true,
        });
        if let Some(k) = taken {
            reg.set_value(m, s, Value::Key(k));
            self.focus = None;
        }
    }
}

/// The window: shadow, the two surfaces, the sidebar, the header and the page.
fn shell(m: &mut Menu, w: &mut W<'_, '_>, reg: &Registry, host: &dyn ClientHost) {
    let win = [0.0, 0.0, t::WINDOW_W, t::WINDOW_H];
    // The drop shadow (50 dp, offset 13 dp, 0.42): a few soft rings outside the window.
    for i in 1..=8 {
        let g = i as f32 * 5.0;
        w.p.rounded(
            [
                -g,
                13.0 - g + 4.0,
                t::WINDOW_W + 2.0 * g,
                t::WINDOW_H + 2.0 * g,
            ],
            t::R_WINDOW + g,
            with_alpha(t::SCRIM, 0.42 / 8.0 * (1.0 - i as f32 / 9.0)),
        );
    }
    let side = [0.0, 0.0, t::SIDEBAR_W, t::WINDOW_H];
    let content = [t::SIDEBAR_W, 0.0, t::WINDOW_W - t::SIDEBAR_W, t::WINDOW_H];
    w.p.push_clip(side);
    w.p.rounded(win, t::R_WINDOW, t::SIDEBAR);
    w.p.pop_clip();
    w.p.push_clip(content);
    w.p.rounded(win, t::R_WINDOW, t::PAGE);
    w.p.pop_clip();
    w.p.fill([t::SIDEBAR_W - 1.0, 0.0, 1.0, t::WINDOW_H], t::BORDER);
    w.p.border(win, t::R_WINDOW, 1.0, t::BORDER);
    sidebar(m, w, host);
    header(m, w, reg, host);
    let page = [
        t::SIDEBAR_W,
        t::PAGE_PAD_Y + t::HEADER_H + t::PAGE_PAD_Y,
        t::WINDOW_W - t::SIDEBAR_W,
        t::WINDOW_H - (t::PAGE_PAD_Y + t::HEADER_H + t::PAGE_PAD_Y) - t::PAGE_PAD_Y,
    ];
    match m.page {
        Page::Mods => mods_page(m, w, reg, host, page),
        Page::Module(i) => module_page(m, w, reg, host, page, i),
        Page::Profiles => profiles_page(m, w, page),
        Page::Keybinds => keybinds_page(m, w, reg, host, page),
    }
}

fn sidebar(m: &mut Menu, w: &mut W<'_, '_>, host: &dyn ClientHost) {
    let x = (t::SIDEBAR_W - t::NAV_W) / 2.0;
    // The logo's place (167 x 19 dp, accent): the product's name until it has a mark of its own.
    w.p.text("PvP Client", [x + 6.0, 28.0 - 3.0], t::TITLE, t::ACCENT);
    let mut y = 28.0 + 19.0 + 28.0;
    w.p.text("MODS & OPTIONS", [x + 6.0, y], t::EYEBROW, t::TEXT);
    y += w.p.line_height(t::EYEBROW) + 2.0 + 6.0;
    let entries = [
        ("settings", "Mods", Page::Mods),
        ("profiles", "Profiles", Page::Profiles),
        ("keyboard", "Keybinds", Page::Keybinds),
    ];
    for (n, (icon, label, page)) in entries.into_iter().enumerate() {
        let selected = m.page == page || (page == Page::Mods && matches!(m.page, Page::Module(_)));
        if w.nav(id("nav", n), [x, y], icon, label, selected) {
            m.history.clear();
            m.navigate(page);
        }
        y += t::NAV_H + t::NAV_GAP;
    }
    // Edit HUD, pinned above the account row.
    let account_y = t::WINDOW_H - 73.0;
    let hud_y = account_y - t::SECTION_GAP - t::NAV_H;
    if w.nav(id("nav-hud", 0), [x, hud_y], "hud", "Edit HUD", false) {
        m.edit_hud = true;
    }
    w.p.fill([0.0, account_y, t::SIDEBAR_W - 1.0, 1.0], t::BORDER);
    let _ = host;
    // The player head's place (32 dp, 8 dp radius).
    w.p.rounded(
        [18.0 + 2.0, account_y + 20.5, 32.0, 32.0],
        t::R_FIELD,
        t::COMPONENT,
    );
    w.p.icon(
        "profiles",
        [18.0 + 2.0 + 7.0, account_y + 27.5, 18.0, 18.0],
        t::TEXT_2,
    );
    let tx = 18.0 + 2.0 + 32.0 + 12.0;
    let profile = runtime::profile();
    w.p.text(&profile, [tx, account_y + 17.0], t::BODY, t::TEXT);
    w.p.text(
        "Profile",
        [tx, account_y + 17.0 + w.p.line_height(t::BODY) + 2.0],
        t::CAPTION,
        t::TEXT_2,
    );
}

fn page_title(m: &Menu, reg: &Registry) -> (Option<&'static str>, String) {
    match m.page {
        Page::Mods => (None, "Mods".to_owned()),
        Page::Module(i) => (Some("Mods"), reg.modules()[i].name().to_owned()),
        Page::Profiles => (None, "Profiles".to_owned()),
        Page::Keybinds => (None, "Keybinds".to_owned()),
    }
}

fn header(m: &mut Menu, w: &mut W<'_, '_>, reg: &Registry, host: &dyn ClientHost) {
    let x0 = t::SIDEBAR_W + t::GUTTER;
    let y = t::PAGE_PAD_Y;
    let can_back = !m.history.is_empty();
    w.p.opacity *= if can_back { 1.0 } else { t::DISABLED_ALPHA };
    let back = w.icon_button(id("back", 0), [x0, y + 2.0, 32.0, 32.0], "left-arrow", 18.0);
    w.p.opacity /= if can_back { 1.0 } else { t::DISABLED_ALPHA };
    if back && let Some(prev) = m.history.pop() {
        m.page = prev;
        m.scroll = 0.0;
        m.popup = None;
    }
    w.p.opacity *= t::DISABLED_ALPHA;
    w.icon_button(
        id("fwd", 0),
        [x0 + 40.0, y + 2.0, 32.0, 32.0],
        "right-arrow",
        18.0,
    );
    w.p.opacity /= t::DISABLED_ALPHA;
    let mut tx = x0 + 32.0 + 8.0 + 32.0 + 24.0;
    let (parent, title) = page_title(m, reg);
    if let Some(parent) = parent {
        let pw = w.p.text_width(parent, t::DISPLAY);
        let r = [tx, y, pw, t::HEADER_H];
        let c = if w.f.hover(r) { t::TEXT } else { t::TEXT_2 };
        w.p.text_mid(parent, tx, y, t::HEADER_H, t::DISPLAY, c);
        if w.f.click(r) {
            m.navigate(Page::Mods);
        }
        tx += pw + 6.0;
        w.p.text_mid("/", tx, y, t::HEADER_H, t::DISPLAY, t::TEXT_2);
        tx += w.p.text_width("/", t::DISPLAY) + 6.0;
    }
    w.p.text_mid(&title, tx, y, t::HEADER_H, t::DISPLAY, t::TEXT);
    if let Page::Module(i) = m.page {
        tx += w.p.text_width(&title, t::DISPLAY) + 8.0;
        let desc = tr(host, reg.modules()[i].description());
        let max = t::WINDOW_W - t::GUTTER - 32.0 - 26.0 - 256.0 - 16.0 - tx;
        let desc = w.p.ellipsize(&desc, t::BODY_MEDIUM, max);
        w.p.text_mid(&desc, tx, y + 3.0, t::HEADER_H, t::BODY_MEDIUM, t::TEXT_2);
    }
    let close = [t::WINDOW_W - t::GUTTER - 32.0, y + 2.0, 32.0, 32.0];
    if w.icon_button(id("close", 0), close, "close", 18.0) {
        m.close = true;
    }
    let search = [close[0] - 26.0 - 256.0, y + 2.0, 256.0, t::CONTROL_H];
    let focused = m.focus == Some(Focus::Search);
    if w.search(search, &m.search, focused) {
        m.focus = Some(Focus::Search);
    }
    if focused {
        type_into(&mut m.search, w.f, 64);
        if take_key(w.f, "key.keyboard.enter") {
            m.focus = None;
        }
    }
}

/// Typed characters and Backspace into `s` (at most `max` characters).
fn type_into(s: &mut String, f: &mut Frame, max: usize) {
    for c in f.chars.drain(..) {
        if !c.is_control() && s.chars().count() < max {
            s.push(c);
        }
    }
    f.keys.retain(|(k, _)| {
        if k == "key.keyboard.backspace" {
            s.pop();
            false
        } else {
            true
        }
    });
}

fn take_key(f: &mut Frame, key: &str) -> bool {
    let n = f.keys.len();
    f.keys.retain(|(k, _)| k != key);
    f.keys.len() != n
}

fn matches_search(m: &Menu, text: &str) -> bool {
    m.search.is_empty() || text.to_lowercase().contains(&m.search.to_lowercase())
}

/// The scrolling page area: clips, offsets by the scroll and records how far it can go.
fn begin_scroll(m: &Menu, w: &mut W<'_, '_>, area: [f32; 4]) -> f32 {
    w.p.push_clip(area);
    area[1] - m.scroll
}

fn end_scroll(m: &mut Menu, w: &mut W<'_, '_>, area: [f32; 4], content_h: f32) {
    w.p.pop_clip();
    m.scroll_max = (content_h - area[3]).max(0.0);
    m.scroll = m.scroll.min(m.scroll_max);
    // The themed scrollbar: 8 dp, textColorSecondary at 0.4 (§4.6).
    if m.scroll_max > 0.0 {
        let track_h = area[3];
        let thumb = (track_h * area[3] / content_h).max(24.0);
        let ty = area[1] + (track_h - thumb) * (m.scroll / m.scroll_max);
        let x = area[0] + area[2] - (t::GUTTER + 8.0) / 2.0;
        let r = [x, ty, 8.0, thumb];
        let a = if w.f.hover(r) { 0.7 } else { 0.4 };
        w.p.rounded(r, 4.0, alpha(t::TEXT_2, a));
    }
}

fn mods_page(
    m: &mut Menu,
    w: &mut W<'_, '_>,
    reg: &Registry,
    host: &dyn ClientHost,
    area: [f32; 4],
) {
    let x0 = area[0] + t::GUTTER;
    // Category chips: All, then each category that has modules.
    let mut x = x0;
    let (cw, all) = w.chip(
        id("chip", 99),
        [x, area[1]],
        "All",
        None,
        m.category.is_none(),
    );
    if all {
        m.category = None;
        m.scroll = 0.0;
    }
    x += cw + 8.0;
    for (n, c) in Category::ALL.into_iter().enumerate() {
        if !reg.modules().iter().any(|md| md.category() == c) {
            continue;
        }
        let label = tr(host, c.label());
        let (cw, clicked) = w.chip(
            id("chip", n),
            [x, area[1]],
            &label,
            Some(category_icon(c)),
            m.category == Some(c),
        );
        if clicked {
            m.category = Some(c);
            m.scroll = 0.0;
        }
        x += cw + 8.0;
    }
    let grid = [
        area[0],
        area[1] + t::CONTROL_H + 19.0,
        area[2],
        area[3] - t::CONTROL_H - 19.0,
    ];
    let top = begin_scroll(m, w, grid);
    let shown: Vec<usize> = (0..reg.modules().len())
        .filter(|&i| {
            let md = reg.modules()[i];
            m.category.is_none_or(|c| md.category() == c) && matches_search(m, md.name())
        })
        .collect();
    if shown.is_empty() {
        let msg = if reg.modules().is_empty() {
            "No modules yet"
        } else {
            "No module matches"
        };
        let mw = w.p.text_width(msg, t::BODY);
        w.p.icon(
            "search",
            [grid[0] + (grid[2] - 32.0) / 2.0, grid[1] + 60.0, 32.0, 32.0],
            t::TEXT_2,
        );
        w.p.text(
            msg,
            [grid[0] + (grid[2] - mw) / 2.0, grid[1] + 100.0],
            t::BODY,
            t::TEXT_2,
        );
    }
    for (n, &i) in shown.iter().enumerate() {
        let col = n % t::CARD_COLUMNS;
        let row = n / t::CARD_COLUMNS;
        let r = [
            x0 + col as f32 * (t::CARD_W + t::CARD_GAP),
            top + row as f32 * (t::CARD_H + t::CARD_GAP),
            t::CARD_W,
            t::CARD_H,
        ];
        if r[1] + r[3] < grid[1] || r[1] > grid[1] + grid[3] {
            continue;
        }
        module_card(m, w, reg, i, r, grid);
    }
    let rows = shown.len().div_ceil(t::CARD_COLUMNS);
    let h = rows as f32 * (t::CARD_H + t::CARD_GAP);
    end_scroll(m, w, grid, h);
}

/// One module card (§5.3): its category icon, the toggle ring top right, the name in the footer that turns
/// accent when on. The footer toggles; the rest opens the module's page.
fn module_card(
    m: &mut Menu,
    w: &mut W<'_, '_>,
    reg: &Registry,
    i: usize,
    r: [f32; 4],
    clip: [f32; 4],
) {
    let md = reg.modules()[i];
    let on = reg.is_enabled(i);
    let k = w.a.get(id("card-on", i), on, w.f.dt, ctx::STANDARD);
    w.p.rounded(r, t::R_CARD, t::CARD);
    w.p.border(r, t::R_CARD, 1.0, t::BORDER);
    let footer = [r[0], r[1] + r[3] - t::CARD_FOOTER_H, r[2], t::CARD_FOOTER_H];
    w.p.push_clip(footer);
    let off_fill = with_alpha(t::TEXT, 0.08);
    w.p.rounded(r, t::R_CARD, t::lerp(off_fill, t::ACCENT, k));
    // The accent sheen (white 0.2 → 0.04) when on.
    w.p.gradient_v(
        footer,
        with_alpha(0xFFFF_FFFF, 0.2 * k),
        with_alpha(0xFFFF_FFFF, 0.04 * k),
    );
    w.p.pop_clip();
    let name = w.p.ellipsize(md.name(), t::TITLE_SMALL, r[2] - 16.0);
    let nw = w.p.text_width(&name, t::TITLE_SMALL);
    let tc = t::lerp(alpha(t::TEXT, 0.75), t::ON_ACCENT, k);
    w.p.text_mid(
        &name,
        r[0] + (r[2] - nw) / 2.0,
        footer[1],
        footer[3],
        t::TITLE_SMALL,
        tc,
    );
    // Body: the category icon, 48 dp, centred.
    let body_h = r[3] - t::CARD_FOOTER_H;
    w.p.icon(
        category_icon(md.category()),
        [
            r[0] + (r[2] - 48.0) / 2.0,
            r[1] + (body_h - 48.0) / 2.0,
            48.0,
            48.0,
        ],
        alpha(t::TEXT, 0.9),
    );
    // Toggle ring, top right: 12 dp ring, 6 dp dot when on.
    let ring = [r[0] + r[2] - 11.0 - 12.0, r[1] + 11.0];
    let ring_c = t::lerp(t::ACCENT, 0xFF18_1600, 0.42);
    w.p.border(
        [ring[0], ring[1], 12.0, 12.0],
        6.0,
        1.0,
        if on { t::ACCENT } else { ring_c },
    );
    w.p.circle([ring[0] + 3.0, ring[1] + 3.0], 6.0, alpha(t::ACCENT, k));
    let visible = w.f.hover(clip);
    if visible && w.f.click(footer) {
        reg.set_enabled(i, !on);
    }
    if visible && w.f.click(r) {
        m.navigate(Page::Module(i));
    }
}

fn module_page(
    m: &mut Menu,
    w: &mut W<'_, '_>,
    reg: &Registry,
    host: &dyn ClientHost,
    area: [f32; 4],
    i: usize,
) {
    let md = reg.modules()[i];
    let x0 = area[0] + t::GUTTER;
    let width = area[2] - 2.0 * t::GUTTER;
    let mut y = begin_scroll(m, w, area);
    let start = y;
    // The module's own switch first, as the card's footer does.
    let row = [x0, y, width, t::CONTROL_H + 24.0];
    option_card(w, row, "Enabled", None);
    let on = reg.is_enabled(i);
    if w.switch(
        id("enabled", i),
        [
            row[0] + row[2] - 16.0 - 42.0,
            row[1] + (row[3] - 21.0) / 2.0,
        ],
        on,
    ) {
        reg.set_enabled(i, !on);
    }
    y += row[3] + 8.0;
    for (s, setting) in md.settings().iter().enumerate() {
        let label = tr(host, setting.label);
        if !matches_search(m, &label) {
            continue;
        }
        let h = t::CONTROL_H + 24.0;
        let row = [x0, y, width, h];
        if row[1] + row[3] >= area[1] && row[1] <= area[1] + area[3] {
            option_card(w, row, &label, None);
            control(m, w, reg, i, s, setting, row);
        }
        y += h + 8.0;
    }
    end_scroll(m, w, area, y - start);
}

/// An option card (§6.3): the card surface and its title.
fn option_card(w: &mut W<'_, '_>, r: [f32; 4], title: &str, description: Option<&str>) {
    w.p.rounded(r, t::R_CARD, t::CARD);
    w.p.border(r, t::R_CARD, 1.0, t::BORDER);
    match description {
        None => {
            w.p.text_mid(title, r[0] + 16.0, r[1], r[3], t::HEADING_MEDIUM, t::TEXT)
        }
        Some(d) => {
            let lh = w.p.line_height(t::HEADING_MEDIUM);
            let dh = w.p.line_height(t::LABEL_REGULAR);
            let y = r[1] + (r[3] - lh - 2.0 - dh) / 2.0;
            w.p.text(title, [r[0] + 16.0, y], t::HEADING_MEDIUM, t::TEXT);
            w.p.text(d, [r[0] + 16.0, y + lh + 2.0], t::LABEL_REGULAR, t::TEXT_2);
        }
    }
}

/// The control of setting `s` of module `i`, right-aligned in its row.
#[allow(clippy::too_many_arguments)]
fn control(
    m: &mut Menu,
    w: &mut W<'_, '_>,
    reg: &Registry,
    i: usize,
    s: usize,
    setting: &Setting,
    row: [f32; 4],
) {
    let right = row[0] + row[2] - 16.0;
    let cy = row[1] + (row[3] - t::CONTROL_H) / 2.0;
    let wid = id(setting.id, i * 64 + s);
    let value = reg.value(i, s);
    match setting.kind {
        Kind::Toggle { .. } => {
            let on = value.as_bool();
            if w.switch(wid, [right - 42.0, row[1] + (row[3] - 21.0) / 2.0], on) {
                reg.set_value(i, s, Value::Bool(!on));
            }
        }
        Kind::Slider { min, max, step, .. } => {
            let v = value.as_f32();
            let nx = right - 87.0;
            let text = if step >= 1.0 || step == 0.0 && (max - min) > 20.0 {
                format!("{v:.0}")
            } else {
                format!("{v:.2}")
            };
            w.number_box([nx, cy], &text);
            let sx = nx - 16.0 - t::OPTION_W;
            if let Some(nv) = w.slider(
                wid,
                sx,
                row[1] + (row[3] - 19.0) / 2.0,
                t::OPTION_W,
                v,
                min,
                max,
            ) {
                reg.set_value(i, s, Value::Num(nv));
            }
        }
        Kind::Choice { options, .. } => {
            let r = [right - t::OPTION_W, cy, t::OPTION_W, t::CONTROL_H];
            let current = options.get(value.as_choice()).copied().unwrap_or("");
            let open = matches!(m.popup, Some(Popup::Select { module, setting, .. }) if module == i && setting == s);
            let label = option_label(setting, current);
            if w.select(wid, r, &label, open) {
                m.popup = if open {
                    None
                } else {
                    Some(Popup::Select {
                        module: i,
                        setting: s,
                        anchor: r,
                    })
                };
            }
        }
        Kind::Color { .. } => {
            let r = [right - t::OPTION_W, cy, t::OPTION_W, t::CONTROL_H];
            let c = value.as_color();
            let hover = w.f.hover(r);
            w.p.rounded(r, t::R_FIELD, c | 0xFF00_0000);
            let b = if hover {
                with_alpha(t::TEXT, 0.3)
            } else {
                t::BORDER
            };
            w.p.border(r, t::R_FIELD, 1.0, b);
            let ink = if luminance(c) > 0.5 {
                0xFF00_0000
            } else {
                0xFFFF_FFFF
            };
            w.p.icon("paintbrush", [r[0] + 12.0, r[1] + 9.0, 14.0, 14.0], ink);
            let st = crate::font::Style::new(crate::font::Weight::Regular, 13.0);
            w.p.text_mid(
                &format!("#{c:08X}"),
                r[0] + 12.0 + 14.0 + 8.0,
                r[1],
                r[3],
                st,
                ink,
            );
            if w.f.click(r) {
                m.hue = rgb_to_hsv(c).0;
                m.popup = Some(Popup::Color {
                    module: i,
                    setting: s,
                    anchor: r,
                });
            }
        }
        Kind::Key { .. } => {
            let r = [right - t::OPTION_W, cy, t::OPTION_W, t::CONTROL_H];
            let recording = m.focus == Some(Focus::Key(i, s));
            let label = if recording {
                "Press keys...".to_owned()
            } else {
                key_name(value.as_str())
            };
            if w.keybind(wid, r, &label, recording) {
                m.focus = Some(Focus::Key(i, s));
            }
        }
        Kind::Text { max, .. } => {
            let r = [right - t::OPTION_W, cy, t::OPTION_W, t::CONTROL_H];
            let focused = m.focus == Some(Focus::Text(i, s));
            let shown = if focused {
                m.text.clone()
            } else {
                value.as_str().to_owned()
            };
            if w.text_field(r, &shown, "", focused) && !focused {
                m.text = value.as_str().to_owned();
                m.focus = Some(Focus::Text(i, s));
            }
            if focused {
                type_into(&mut m.text, w.f, max);
                if take_key(w.f, "key.keyboard.enter") {
                    m.focus = None;
                }
                reg.set_value(i, s, Value::Text(m.text.clone()));
            }
        }
    }
}

/// A choice option's label: its translation `<setting label>.<option>` would go here; the id, readable.
fn option_label(_setting: &Setting, option: &str) -> String {
    let s = option.replace(['_', '-'], " ");
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// Rows of a select menu: compact items of 30 dp, 2 dp apart, in 4 dp padding (§6.5.4).
const ITEM_H: f32 = 30.0;

fn popup_rect(pop: &Popup, reg: &Registry) -> [f32; 4] {
    match *pop {
        Popup::Select {
            module,
            setting,
            anchor,
        } => {
            let n = match reg.modules()[module].settings()[setting].kind {
                Kind::Choice { options, .. } => options.len(),
                _ => 0,
            };
            let h = 8.0 + n as f32 * (ITEM_H + 2.0) - 2.0;
            [anchor[0], anchor[1] + anchor[3] + 6.0, anchor[2], h]
        }
        Popup::Color { anchor, .. } => {
            let w = 280.0;
            [
                anchor[0] + anchor[2] - w,
                anchor[1] + anchor[3] + 6.0,
                w,
                16.0 + 160.0 + 14.0 + 16.0 + 14.0 + 16.0 + 14.0 + 32.0 + 16.0,
            ]
        }
    }
}

fn draw_popup(m: &mut Menu, w: &mut W<'_, '_>, reg: &Registry, pop: Popup) {
    let r = popup_rect(&pop, reg);
    // The popup's shadow (16 dp elevation), then its surface.
    for i in 1..=4 {
        let g = i as f32 * 3.0;
        w.p.rounded(
            [r[0] - g, r[1] - g + 4.0, r[2] + 2.0 * g, r[3] + 2.0 * g],
            t::R_POPUP + g,
            with_alpha(t::SCRIM, 0.12 / i as f32),
        );
    }
    w.p.rounded(r, t::R_POPUP, t::POPUP);
    match pop {
        Popup::Select {
            module, setting, ..
        } => {
            let Kind::Choice { options, .. } = reg.modules()[module].settings()[setting].kind
            else {
                return;
            };
            let current = reg.value(module, setting).as_choice();
            for (n, o) in options.iter().enumerate() {
                let item = [
                    r[0] + 4.0,
                    r[1] + 4.0 + n as f32 * (ITEM_H + 2.0),
                    r[2] - 8.0,
                    ITEM_H,
                ];
                if w.f.hover(item) {
                    w.p.rounded(item, 12.0, with_alpha(t::TEXT, 0.10));
                }
                let st = crate::font::Style::new(crate::font::Weight::Regular, 13.0);
                w.p.text_mid(
                    &option_label(&reg.modules()[module].settings()[setting], o),
                    item[0] + 8.0,
                    item[1],
                    item[3],
                    st,
                    t::TEXT,
                );
                if n == current {
                    w.p.icon(
                        "tick",
                        [item[0] + item[2] - 8.0 - 14.0, item[1] + 8.0, 14.0, 14.0],
                        t::TEXT,
                    );
                }
                if w.f.click(item) {
                    reg.set_value(module, setting, Value::Choice(n));
                    m.popup = None;
                }
            }
        }
        Popup::Color {
            module, setting, ..
        } => color_picker(m, w, reg, module, setting, r),
    }
}

/// The colour picker (§6.5.5): saturation/brightness pane, hue bar, opacity bar, swatch and hex.
fn color_picker(m: &mut Menu, w: &mut W<'_, '_>, reg: &Registry, i: usize, s: usize, r: [f32; 4]) {
    let c = reg.value(i, s).as_color();
    let (h0, sat, val) = rgb_to_hsv(c);
    let hue = if sat > 0.0 && val > 0.0 { h0 } else { m.hue };
    let a = (c >> 24) as f32 / 255.0;
    let x = r[0] + 16.0;
    let inner = r[2] - 32.0;
    let mut y = r[1] + 16.0;
    w.p.text_mid(
        "Colour picker",
        x,
        y - 4.0,
        18.0,
        crate::font::Style::new(crate::font::Weight::Regular, 14.0),
        t::TEXT,
    );
    y += 14.0 + 4.0;
    // Pane: white → hue across, transparent → black down.
    let pane = [x, y, inner, 160.0];
    w.p.push_clip(pane);
    w.p.gradient_h(pane, 0xFFFF_FFFF, hsv_to_rgb(hue, 1.0, 1.0));
    w.p.gradient_v(pane, 0x0000_0000, 0xFF00_0000);
    w.p.pop_clip();
    let sx = x + sat * inner - 7.0;
    let sy = y + (1.0 - val) * 160.0 - 7.0;
    w.p.circle([sx, sy], 14.0, 0xFFFF_FFFF);
    w.p.circle([sx + 2.0, sy + 2.0], 10.0, c | 0xFF00_0000);
    y += 160.0 + 14.0;
    let hue_bar = [x, y, inner, 16.0];
    let stops = [
        0xFFFF_0000,
        0xFFFF_FF00,
        0xFF00_FF00,
        0xFF00_FFFF,
        0xFF00_00FF,
        0xFFFF_00FF,
        0xFFFF_0000,
    ];
    w.p.push_clip(hue_bar);
    for n in 0..6 {
        let seg = [x + inner * n as f32 / 6.0, y, inner / 6.0 + 0.5, 16.0];
        w.p.gradient_h(seg, stops[n], stops[n + 1]);
    }
    w.p.pop_clip();
    w.p.circle([x + hue / 360.0 * inner - 5.0, y + 3.0], 10.0, 0xFFFF_FFFF);
    y += 16.0 + 14.0;
    let alpha_bar = [x, y, inner, 16.0];
    w.p.rounded(alpha_bar, 8.0, 0xFF6A_6A6A);
    w.p.push_clip(alpha_bar);
    w.p.gradient_h(alpha_bar, c & 0x00FF_FFFF, c | 0xFF00_0000);
    w.p.pop_clip();
    w.p.circle([x + a * inner - 5.0, y + 3.0], 10.0, 0xFFFF_FFFF);
    y += 16.0 + 14.0;
    w.p.rounded([x, y, 32.0, 32.0], 8.0, c | 0xFF00_0000);
    w.p.border([x, y, 32.0, 32.0], 8.0, 1.0, t::BORDER);
    let st = crate::font::Style::new(crate::font::Weight::Regular, 13.0);
    let field = [x + 40.0, y, inner - 40.0, 32.0];
    w.p.rounded(field, t::R_FIELD, t::CARD);
    w.p.border(field, t::R_FIELD, 1.0, t::BORDER);
    w.p.text_mid(&format!("#{c:08X}"), field[0] + 12.0, y, 32.0, st, t::TEXT);
    // Dragging.
    let held = w.f.down;
    if w.f.click(pane) {
        m.picking = Some(0);
    } else if w.f.click(hue_bar) {
        m.picking = Some(1);
    } else if w.f.click(alpha_bar) {
        m.picking = Some(2);
    }
    if !held {
        m.picking = None;
    }
    let mx = ((w.f.mouse[0] - x) / inner).clamp(0.0, 1.0);
    let my = ((w.f.mouse[1] - pane[1]) / 160.0).clamp(0.0, 1.0);
    let rgba = |h: f32, s: f32, v: f32, a: f32| {
        (((a * 255.0).round() as u32) << 24) | (hsv_to_rgb(h, s, v) & 0x00FF_FFFF)
    };
    let new = match m.picking {
        Some(0) => Some(rgba(hue, mx, 1.0 - my, a)),
        Some(1) => {
            m.hue = mx * 360.0;
            Some(rgba(m.hue, sat, val, a))
        }
        Some(2) => Some(rgba(hue, sat, val, mx)),
        _ => None,
    };
    if let Some(n) = new
        && n != c
    {
        reg.set_value(i, s, Value::Color(n));
    }
    let _ = over;
}

fn luminance(c: u32) -> f32 {
    let r = ((c >> 16) & 0xFF) as f32 / 255.0;
    let g = ((c >> 8) & 0xFF) as f32 / 255.0;
    let b = (c & 0xFF) as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Hue (0..360), saturation and value (0..1) of an ARGB colour.
pub fn rgb_to_hsv(c: u32) -> (f32, f32, f32) {
    let r = ((c >> 16) & 0xFF) as f32 / 255.0;
    let g = ((c >> 8) & 0xFF) as f32 / 255.0;
    let b = (c & 0xFF) as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / d).rem_euclid(6.0))
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let s = if max == 0.0 { 0.0 } else { d / max };
    (h, s, max)
}

/// An opaque ARGB colour from hue (0..360), saturation and value.
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
    let c = v * s;
    let hp = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let mm = v - c;
    let ch = |f: f32| ((f + mm) * 255.0).round() as u32;
    0xFF00_0000 | (ch(r) << 16) | (ch(g) << 8) | ch(b)
}

fn profiles_page(m: &mut Menu, w: &mut W<'_, '_>, area: [f32; 4]) {
    let Some(dir) = runtime::game_dir() else {
        return;
    };
    let names = pvp_core::config::list(dir);
    let active = runtime::profile();
    let x0 = area[0] + t::GUTTER;
    let (cw, ch, gap) = (199.0, 120.0, 19.0);
    let per_row = 5;
    let mut n = 0;
    for name in &names {
        let r = [
            x0 + (n % per_row) as f32 * (cw + gap),
            area[1] + (n / per_row) as f32 * (ch + gap),
            cw,
            ch,
        ];
        let is_active = *name == active;
        w.p.rounded(r, t::R_CARD, t::CARD);
        if is_active {
            w.p.rounded(r, t::R_CARD, with_alpha(t::ACCENT, 0.08));
            w.p.border(r, t::R_CARD, 2.0, t::ACCENT);
        } else {
            let b = if w.f.hover(r) { t::TEXT_2 } else { t::BORDER };
            w.p.border(r, t::R_CARD, 1.0, b);
        }
        w.p.icon(
            "combat",
            [r[0] + (cw - 32.0) / 2.0, r[1] + 26.0, 32.0, 32.0],
            t::TEXT,
        );
        let label = w.p.ellipsize(name, t::TITLE, cw - 24.0);
        let lw = w.p.text_width(&label, t::TITLE);
        w.p.text(
            &label,
            [r[0] + (cw - lw) / 2.0, r[1] + 72.0],
            t::TITLE,
            t::TEXT,
        );
        if w.f.click(r)
            && !is_active
            && let Err(e) = runtime::switch_profile(name)
        {
            log::warn!("pvp: switching to profile {name}: {e}");
        }
        n += 1;
    }
    // The "new profile" card: a name field and a create button.
    let r = [
        x0 + (n % per_row) as f32 * (cw + gap),
        area[1] + (n / per_row) as f32 * (ch + gap),
        cw,
        ch,
    ];
    w.p.rounded(r, t::R_CARD, t::CARD);
    w.p.border(r, t::R_CARD, 1.0, t::BORDER);
    let focused = m.focus == Some(Focus::ProfileName);
    let field = [r[0] + 12.0, r[1] + 20.0, cw - 24.0, t::CONTROL_H];
    if w.text_field(field, &m.text, "New profile", focused) {
        if !focused {
            m.text.clear();
        }
        m.focus = Some(Focus::ProfileName);
    }
    if focused {
        type_into(&mut m.text, w.f, 32);
    }
    let create = [
        r[0] + 12.0,
        r[1] + 20.0 + t::CONTROL_H + 16.0,
        cw - 24.0,
        t::CONTROL_H,
    ];
    let submit = w.button(id("create", 0), create, "Create", true)
        || (focused && take_key(w.f, "key.keyboard.enter"));
    if submit && pvp_core::config::valid_name(&m.text) {
        let name = m.text.trim().to_owned();
        let made = pvp_core::config::duplicate(dir, &active, &name);
        match made.and_then(|()| runtime::switch_profile(&name)) {
            Ok(()) => {
                m.text.clear();
                m.focus = None;
            }
            Err(e) => log::warn!("pvp: creating profile {name}: {e}"),
        }
    }
}

fn keybinds_page(
    m: &mut Menu,
    w: &mut W<'_, '_>,
    reg: &Registry,
    host: &dyn ClientHost,
    area: [f32; 4],
) {
    let x0 = area[0] + t::GUTTER;
    let width = area[2] - 2.0 * t::GUTTER;
    let mut y = begin_scroll(m, w, area);
    let start = y;
    let mut any = false;
    for (i, md) in reg.modules().iter().enumerate() {
        for (s, setting) in md.settings().iter().enumerate() {
            if !matches!(setting.kind, Kind::Key { .. }) {
                continue;
            }
            let label = format!("{} — {}", md.name(), tr(host, setting.label));
            if !matches_search(m, &label) {
                continue;
            }
            any = true;
            let row = [x0, y, width, t::CONTROL_H + 24.0];
            option_card(w, row, &label, None);
            control(m, w, reg, i, s, setting, row);
            y += row[3] + 8.0;
        }
    }
    if !any {
        w.p.text("No keybinds yet", [x0, y], t::BODY, t::TEXT_2);
    }
    end_scroll(m, w, area, y - start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_window_fits_ninety_percent_of_the_screen_and_never_grows_past_one() {
        assert_eq!(Menu::scale_for([3840.0, 2160.0]), 1.0);
        let s = Menu::scale_for([1280.0, 720.0]);
        assert!((s - 0.9 * 1280.0 / 1391.0).abs() < 1e-4, "{s}");
        assert_eq!(Menu::scale_for([100.0, 100.0]), 0.25);
    }

    #[test]
    fn hsv_round_trips() {
        for c in [
            0xFFFF_0000u32,
            0xFF2B_4BFF,
            0xFF12_3456,
            0xFFFF_FFFF,
            0xFF00_0000,
        ] {
            let (h, s, v) = rgb_to_hsv(c);
            assert_eq!(hsv_to_rgb(h, s, v), c, "{c:08X}");
        }
    }
}
