//! The painter the menu, the HUD editor and the HUD elements draw with: `DrawContext` plus what OneConfig's
//! look needs and the game's GUI lacks (Poppins text, rounded rectangles, borders, SVG icons, a scale), all
//! through one dynamic GUI texture, the UI atlas.
//!
//! A painter works in its own unit: GUI pixels for the HUD, OneConfig's dp for the menu. `k` turns a unit
//! into GUI pixels (what `DrawContext` takes) and `phys` into physical pixels (what text, corners and icons
//! are rasterized at, so they stay sharp at any size).

use crate::atlas::Atlas;
use crate::font::{Fonts, Style};
use crate::icons::Icons;
use crate::shape::{self, Shapes};
use crate::theme;
use mod_api::{ClientHost, DrawContext, Identifier};
use std::time::Instant;

/// The UI atlas's GUI sprite id, in the UI mod's namespace.
pub const ATLAS_ID: &str = "pvp_ui:ui_atlas";
/// 1024 x 512 RGBA = 2 MiB of the mod's 4 MiB dynamic texture budget.
const ATLAS_W: u32 = 1024;
const ATLAS_H: u32 = 512;

/// The UI's drawing resources, shared by every hook that draws.
pub struct Ui {
    atlas: Atlas,
    pub fonts: Fonts,
    shapes: Shapes,
    icons: Icons,
    /// The game's GUI scale (physical pixels per GUI pixel), as last read.
    pub gui_scale: f32,
    /// The window's size in physical pixels, as last read.
    pub window: [f32; 2],
    created: bool,
    /// Sprites the game refused to draw (logged once a second while any are).
    refused: u32,
    start: Instant,
    /// Seconds since the UI started, read once per frame.
    pub now: f32,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            atlas: Atlas::new(ATLAS_W, ATLAS_H),
            fonts: Fonts::default(),
            shapes: Shapes::default(),
            icons: Icons::default(),
            gui_scale: 2.0,
            window: [1920.0, 1080.0],
            created: false,
            refused: 0,
            start: Instant::now(),
            now: 0.0,
        }
    }
}

impl Ui {
    /// Once a frame before drawing (`render_frame_pre`): reads the GUI scale and sends the atlas's new pixels.
    /// A scale change, or an atlas that filled up last frame, clears it: what is drawn next is rasterized again.
    pub fn sync(&mut self, host: &mut dyn ClientHost) {
        let now = self.start.elapsed().as_secs_f32();
        if self.refused > 0 && now.floor() != self.now.floor() {
            log::warn!(
                "pvp: {} UI sprite draws refused (created: {})",
                self.refused,
                self.created
            );
            self.refused = 0;
        }
        self.now = now;
        let scale = host.gui_scale().unwrap_or(2).max(1) as f32;
        if debug() && (scale != self.gui_scale || self.atlas.full) {
            log::info!(
                "pvp: clearing the UI atlas (scale {} -> {scale}, full {}, refused {:?}, shelves used {})",
                self.gui_scale,
                self.atlas.full,
                self.atlas.refused,
                self.atlas.used_height()
            );
        }
        if scale != self.gui_scale || self.atlas.full {
            self.gui_scale = scale;
            self.atlas.clear();
        }
        if !self.atlas.dirty {
            return;
        }
        let Ok(id) = Identifier::parse(ATLAS_ID) else {
            return;
        };
        let (w, h, px) = (self.atlas.width, self.atlas.height, &self.atlas.pixels);
        let result = if self.created {
            host.update_dynamic_texture(&id, w, h, px)
        } else {
            host.create_dynamic_texture(&id, w, h, px)
        };
        match result {
            Ok(()) => {
                if !self.created {
                    log::info!("pvp: UI atlas {ATLAS_ID} created ({w}x{h})");
                } else if debug() {
                    let ink = px.chunks(4).filter(|p| p[3] > 0).count();
                    log::info!("pvp: UI atlas updated, {ink} inked texels");
                }
                self.created = true;
                self.atlas.dirty = false;
            }
            Err(e) => log::warn!("pvp: the UI atlas could not be uploaded: {e:?}"),
        }
    }

    /// A painter in GUI pixels over `draw`, for one hook call.
    pub fn painter<'a>(&'a mut self, draw: &'a mut dyn DrawContext) -> Painter<'a> {
        let phys = self.gui_scale;
        Painter {
            ui: self,
            draw,
            k: 1.0,
            phys,
            origin: [0.0, 0.0],
            opacity: 1.0,
        }
    }

    /// A painter whose unit is `phys` physical pixels, with its origin at `origin` (GUI pixels).
    pub fn painter_scaled<'a>(
        &'a mut self,
        draw: &'a mut dyn DrawContext,
        phys: f32,
        origin: [f32; 2],
    ) -> Painter<'a> {
        let k = phys / self.gui_scale;
        Painter {
            ui: self,
            draw,
            k,
            phys,
            origin,
            opacity: 1.0,
        }
    }
}

pub struct Painter<'a> {
    pub ui: &'a mut Ui,
    pub draw: &'a mut dyn DrawContext,
    /// GUI pixels per unit.
    pub k: f32,
    /// Physical pixels per unit.
    pub phys: f32,
    /// Where unit (0, 0) is, in GUI pixels.
    pub origin: [f32; 2],
    /// Multiplies every colour's alpha (the window's open and close fades).
    pub opacity: f32,
}

impl Painter<'_> {
    /// The screen's size in units.
    pub fn size(&self) -> [f32; 2] {
        let [w, h] = self.draw.size();
        [w / self.k, h / self.k]
    }

    /// A GUI-pixel point in units.
    pub fn to_units(&self, p: [f32; 2]) -> [f32; 2] {
        [
            (p[0] - self.origin[0]) / self.k,
            (p[1] - self.origin[1]) / self.k,
        ]
    }

    fn gui(&self, r: [f32; 4]) -> [f32; 4] {
        [
            self.origin[0] + r[0] * self.k,
            self.origin[1] + r[1] * self.k,
            r[2] * self.k,
            r[3] * self.k,
        ]
    }

    fn col(&self, argb: u32) -> Option<u32> {
        let c = if self.opacity < 1.0 {
            theme::alpha(argb, self.opacity)
        } else {
            argb
        };
        (c >> 24 != 0).then_some(c)
    }

    pub fn fill(&mut self, rect: [f32; 4], argb: u32) {
        if let Some(c) = self.col(argb)
            && rect[2] > 0.0
            && rect[3] > 0.0
        {
            let r = self.gui(rect);
            self.draw.fill(r, c);
        }
    }

    /// A rectangle blending from `top` to `bottom`.
    pub fn gradient_v(&mut self, rect: [f32; 4], top: u32, bottom: u32) {
        let (t, b) = (
            theme::alpha(top, self.opacity),
            theme::alpha(bottom, self.opacity),
        );
        if (t | b) >> 24 != 0 {
            let r = self.gui(rect);
            self.draw.fill_gradient(r, t, b);
        }
    }

    /// A rectangle blending from `left` to `right`, in physical-pixel-wide strips.
    pub fn gradient_h(&mut self, rect: [f32; 4], left: u32, right: u32) {
        let strips = ((rect[2] * self.phys).ceil() as usize).clamp(1, 512);
        let w = rect[2] / strips as f32;
        for i in 0..strips {
            let t = (i as f32 + 0.5) / strips as f32;
            self.fill(
                [rect[0] + i as f32 * w, rect[1], w, rect[3]],
                theme::lerp(left, right, t),
            );
        }
    }

    /// Radius in physical pixels for `radius` units, clamped to fit `rect`.
    fn radius_px(&self, rect: [f32; 4], radius: f32) -> u32 {
        let max = rect[2].min(rect[3]) / 2.0;
        (radius.min(max).max(0.0) * self.phys).round() as u32
    }

    /// A filled rectangle with rounded corners.
    pub fn rounded(&mut self, rect: [f32; 4], radius: f32, argb: u32) {
        let Some(c) = self.col(argb) else { return };
        if rect[2] <= 0.0 || rect[3] <= 0.0 {
            return;
        }
        let r = self.radius_px(rect, radius);
        if r == 0 {
            let g = self.gui(rect);
            return self.draw.fill(g, c);
        }
        if let Some(slot) = self.ui.shapes.disc(&mut self.ui.atlas, r, 0) {
            let p = shape::pieces(rect, slot, r, self.phys, None);
            self.emit(p, c);
        }
    }

    /// The border (`width` units, inside `rect`) of a rounded rectangle.
    pub fn border(&mut self, rect: [f32; 4], radius: f32, width: f32, argb: u32) {
        let Some(c) = self.col(argb) else { return };
        if width <= 0.0 || rect[2] <= 0.0 || rect[3] <= 0.0 {
            return;
        }
        let r = self.radius_px(rect, radius);
        let ring = (width * self.phys).round().max(1.0) as u32;
        let width = ring as f32 / self.phys;
        if r == 0 {
            let [x, y, w, h] = rect;
            for f in [
                [x, y, w, width],
                [x, y + h - width, w, width],
                [x, y + width, width, h - 2.0 * width],
                [x + w - width, y + width, width, h - 2.0 * width],
            ] {
                let g = self.gui(f);
                self.draw.fill(g, c);
            }
            return;
        }
        if let Some(slot) = self.ui.shapes.disc(&mut self.ui.atlas, r, ring) {
            let p = shape::pieces(rect, slot, r, self.phys, Some(width));
            self.emit(p, c);
        }
    }

    /// A circle of diameter `d` with its top left at `at`.
    pub fn circle(&mut self, at: [f32; 2], d: f32, argb: u32) {
        self.rounded([at[0], at[1], d, d], d / 2.0, argb);
    }

    fn emit(&mut self, p: shape::Pieces, c: u32) {
        for (src, dst) in p.corners {
            let g = self.gui(dst);
            if !self.draw.sprite(ATLAS_ID, src, g, Some(c)) {
                self.ui.refused += 1;
            }
        }
        for f in p.fills {
            let g = self.gui(f);
            self.draw.fill(g, c);
        }
    }

    /// `text` with its line's top left at `at`.
    pub fn text(&mut self, text: &str, at: [f32; 2], style: Style, argb: u32) {
        let Some(c) = self.col(argb) else { return };
        if text.is_empty() {
            return;
        }
        let quads = self
            .ui
            .fonts
            .layout(&mut self.ui.atlas, text, style, at, self.phys);
        for q in quads {
            let g = self.gui(q.dst);
            if !self.draw.sprite(ATLAS_ID, q.src, g, Some(c)) {
                self.ui.refused += 1;
            }
        }
    }

    /// `text` vertically centred in a box of height `h` starting at `y`, left at `x`.
    pub fn text_mid(&mut self, text: &str, x: f32, y: f32, h: f32, style: Style, argb: u32) {
        let lh = self.line_height(style);
        self.text(text, [x, y + (h - lh) / 2.0], style, argb);
    }

    /// `text` shortened with "…" to fit `max` units.
    pub fn ellipsize(&self, text: &str, style: Style, max: f32) -> String {
        if self.text_width(text, style) <= max {
            return text.to_owned();
        }
        let mut s: String = text.to_owned();
        while !s.is_empty() && self.text_width(&format!("{s}…"), style) > max {
            s.pop();
        }
        format!("{s}…")
    }

    pub fn text_width(&self, text: &str, style: Style) -> f32 {
        self.ui.fonts.width(text, style)
    }

    pub fn line_height(&self, style: Style) -> f32 {
        self.ui.fonts.line_height(style)
    }

    /// Icon `name` drawn into the square `rect`, tinted `argb`.
    pub fn icon(&mut self, name: &str, rect: [f32; 4], argb: u32) {
        let Some(c) = self.col(argb) else { return };
        let px = (rect[2] * self.phys).round() as u32;
        if let Some(slot) = self.ui.icons.get(&mut self.ui.atlas, name, px) {
            let g = self.gui(rect);
            if !self.draw.sprite(ATLAS_ID, slot.src(), g, Some(c)) {
                self.ui.refused += 1;
            }
        }
    }

    pub fn push_clip(&mut self, rect: [f32; 4]) {
        let g = self.gui(rect);
        let _ = self.draw.push_clip(g);
    }

    pub fn pop_clip(&mut self) {
        let _ = self.draw.pop_clip();
    }
}

/// `PVP_UI_DEBUG=1`: log atlas uploads and draw the atlas itself over the menu.
pub fn debug() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("PVP_UI_DEBUG").is_some_and(|v| v == "1"))
}
