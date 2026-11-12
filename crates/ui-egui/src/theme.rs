//! Design tokens and fonts.
//!
//! Colours are our own, tuned to read like a modern desktop spreadsheet. Fonts come from egui's
//! bundled faces and the operating system at runtime: the UI uses the platform UI font, cells use
//! the best available match for the workbook font (Calibri/Carlito/Aptos/Arial…), and egui's
//! bundled Ubuntu/Hack/egui-icons and monochrome emoji faces are the last resort. A build with
//! `CRAFT_FONTS_DIR` also embeds the permitted OFL craft-fonts faces (barred Adobe/Source/Noto CJK
//! families never enter it, and a web build keeps only its explicit allowlist). Native builds opt
//! into craft-fonts' OFL LXGW Chinese face, so a host without a Han system face still draws Chinese
//! interface text; no font file is stored in this repo.
//!
//! Chrome colours belong in [`Tokens`], each with a light and a dark value; colours
//! drawn on the sheet keep their light value in both themes, because the sheet stays white. The
//! foreground/background pairs are listed in `contrast_tests`, which holds them to WCAG 2 (4.5:1
//! for text, 3:1 for glyphs) in both themes: a new token goes there too.

use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use egui::epaint::text::Blob;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, Visuals};

mod font_fallback;
mod font_setup;

pub use font_fallback::symbol_face_installed;
#[cfg(test)]
use font_fallback::{cjk_han_japanese, cjk_han_simplified, cjk_hangul, fallback_faces, hebrew, lao, symbols_more, thai};
use font_setup::pick;
pub use font_setup::{
    CELL, CELL_BOLD, CELL_BOLD_ITALIC, CELL_ITALIC, CraftFont, HanOrder, MONO, SERIF, UI, UI_BOLD, available_text, cell_family, cell_text_width,
    chinese_font_loaded, craft_font_scripts, craft_fonts_cover, craft_fonts_loaded, craft_fonts_required, font_definitions,
    font_definitions_for_language, font_definitions_with, pinned_font_definitions, symbol_fallback, ui_bold, ui_font,
};
#[cfg(test)]
use font_setup::{candidates, craft_font_faces, try_load};
#[cfg(test)]
pub(crate) use font_setup::{drop_unparseable_fonts, font_parses};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    pub window: Color32,
    pub ribbon: Color32,
    pub ribbon_border: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub accent: Color32,
    pub accent_dark: Color32,
    pub accent_soft: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub separator: Color32,
    pub grid_bg: Color32,
    /// Behind the pages in Page Layout view.
    pub desk: Color32,
    pub gridline: Color32,
    pub header_bg: Color32,
    pub header_text: Color32,
    pub header_line: Color32,
    pub header_sel_bg: Color32,
    pub header_sel_text: Color32,
    pub header_all_bg: Color32,
    pub sel_fill: Color32,
    pub sel_border: Color32,
    pub input_bg: Color32,
    pub input_border: Color32,
    pub tab_bar: Color32,
    pub tab_active: Color32,
    pub status_bar: Color32,
    pub cell_text: Color32,
    pub menu_bg: Color32,
    pub shadow: Color32,
    pub danger: Color32,
    /// Background error checking: the corner triangle, and the warning button's diamond and mark.
    pub error_check_green: Color32,
    pub warning_fill: Color32,
    pub warning_ink: Color32,
    /// Info-bar (notice strip) fills, by severity.
    pub notice_warn_bg: Color32,
    pub notice_warn_border: Color32,
    pub notice_info_bg: Color32,
    pub notice_info_border: Color32,
    /// Text and glyphs drawn on an `accent` fill (primary buttons, switched-on toggles).
    pub on_accent: Color32,
    /// The category palette, indexed by [`Cat`]: the coloured parts of icons, gallery highlights.
    pub cat: [Color32; 8],
    /// The page colour inside icons (documents, cells, clipboards).
    pub paper: Color32,
    /// Reference colours for formulas edited in the window-coloured formula bar.
    pub ref_colors: [Color32; 8],
    /// Reference colours on the sheet (outlines and the in-cell editor). The sheet stays white.
    pub sheet_ref_colors: [Color32; 8],
    /// Code colours in the code editor (and the window-coloured fields it sits in).
    pub syntax: Syntax,
    /// The message box warning disc and its glyph.
    pub warning: Color32,
    pub on_warning: Color32,
    /// Controls drawn on the sheet (filter and validation buttons, object handles). They follow
    /// the sheet, so they are light in both themes.
    pub sheet_control_bg: Color32,
    pub sheet_control_border: Color32,
    pub sheet_control_text: Color32,
    /// A placeholder fill on the sheet (a picture that has not loaded).
    pub sheet_track: Color32,
    /// The macro recording dot (Record Macro, the status bar).
    pub recording: Color32,
    /// A breakpoint dot in the macro step editor's gutter.
    pub breakpoint: Color32,
    /// Macro overlays on the sheet (Check and debugging): the inputs, the cells the macro writes,
    /// the cells it would write over, and the current step's target. Sheet colours.
    pub sheet_macro_input: Color32,
    pub sheet_macro_footprint: Color32,
    pub sheet_macro_conflict: Color32,
    pub sheet_macro_target: Color32,
}

/// The eight category colours, in the order of [`Tokens::cat`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cat {
    Blue,
    Green,
    Orange,
    Red,
    Purple,
    Teal,
    Amber,
    Slate,
}

impl Cat {
    pub const ALL: [Cat; 8] = [Cat::Blue, Cat::Green, Cat::Orange, Cat::Red, Cat::Purple, Cat::Teal, Cat::Amber, Cat::Slate];
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// Reference colours on the white sheet, the same in both themes (the sheet does not follow the
/// theme). The light theme's formula bar is white too, so it uses them as well.
const SHEET_REF_COLORS: [Color32; 8] =
    [rgb(0x2E6FD8), rgb(0xD03C3C), rgb(0x7A4FC9), rgb(0x23803D), rgb(0xB5531A), rgb(0x16798A), rgb(0xB83B8E), rgb(0x77681A)];
/// Macro overlays on the sheet, the same in both themes (the sheet stays white).
const SHEET_MACRO_INPUT: Color32 = rgb(0x2E6FD8);
const SHEET_MACRO_FOOTPRINT: Color32 = rgb(0x23803D);
const SHEET_MACRO_CONFLICT: Color32 = rgb(0xC42B1C);
const SHEET_MACRO_TARGET: Color32 = rgb(0x6B3FC0);

/// Code colours for one background; plain text uses the surface's own text colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Syntax {
    pub keyword: Color32,
    pub string: Color32,
    pub number: Color32,
    pub comment: Color32,
    pub builtin: Color32,
}

const LIGHT_SYNTAX: Syntax =
    Syntax { keyword: rgb(0x0B57B3), string: rgb(0xA3361B), number: rgb(0x157A5A), comment: rgb(0x6B7280), builtin: rgb(0x7B3FB5) };
const DARK_SYNTAX: Syntax =
    Syntax { keyword: rgb(0x6FAEFF), string: rgb(0xF09A7A), number: rgb(0x7FD1A8), comment: rgb(0x9AA3AE), builtin: rgb(0xC69BF2) };
const SHEET_CONTROL_BG: Color32 = rgb(0xF3F3F3);
const SHEET_CONTROL_BORDER: Color32 = rgb(0x8A8A8A);
const SHEET_CONTROL_TEXT: Color32 = rgb(0x1F1F1F);
const SHEET_TRACK: Color32 = rgb(0xD6D6D6);

impl Tokens {
    pub fn light() -> Tokens {
        Tokens::palette(false)
    }

    pub fn dark() -> Tokens {
        Tokens::palette(true)
    }

    /// One palette for both modes: each colour is written once, with its light and dark values.
    fn palette(dark: bool) -> Tokens {
        Tokens {
            dark,
            window: pick(dark, Color32::from_rgb(0xEF, 0xED, 0xE8), Color32::from_rgb(0x1F, 0x1C, 0x28)),
            ribbon: pick(dark, Color32::from_rgb(0xF7, 0xF6, 0xF2), Color32::from_rgb(0x1D, 0x1A, 0x26)),
            ribbon_border: pick(dark, Color32::from_rgb(0xE0, 0xDE, 0xE7), Color32::from_rgb(0x35, 0x32, 0x44)),
            text: pick(dark, Color32::from_rgb(0x22, 0x1F, 0x2E), Color32::from_rgb(0xED, 0xEA, 0xF6)),
            text_dim: pick(dark, Color32::from_rgb(0x5D, 0x5A, 0x6E), Color32::from_rgb(0xB5, 0xB1, 0xCC)),
            text_disabled: pick(dark, Color32::from_rgb(0x9A, 0x96, 0xAC), Color32::from_rgb(0x66, 0x61, 0x7E)),
            accent: pick(dark, Color32::from_rgb(0x7C, 0x3A, 0xED), Color32::from_rgb(0xA7, 0x8B, 0xFA)),
            accent_dark: pick(dark, Color32::from_rgb(0x5B, 0x21, 0xB6), Color32::from_rgb(0xC4, 0xB5, 0xFD)),
            accent_soft: pick(dark, Color32::from_rgb(0xEF, 0xE9, 0xFB), Color32::from_rgb(0x2E, 0x28, 0x48)),
            hover: pick(dark, Color32::from_rgb(0xE9, 0xE7, 0xEF), Color32::from_rgb(0x2B, 0x28, 0x38)),
            pressed: pick(dark, Color32::from_rgb(0xD8, 0xD5, 0xE2), Color32::from_rgb(0x37, 0x33, 0x49)),
            separator: pick(dark, Color32::from_rgb(0xE0, 0xDE, 0xE7), Color32::from_rgb(0x35, 0x32, 0x44)),
            grid_bg: Color32::WHITE,
            desk: pick(dark, Color32::from_rgb(0xDE, 0xDC, 0xD2), Color32::from_rgb(0x1E, 0x1B, 0x26)),
            gridline: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            header_bg: pick(dark, Color32::from_rgb(0xF2, 0xF0, 0xEB), Color32::from_rgb(0x22, 0x1F, 0x2E)),
            header_text: pick(dark, Color32::from_rgb(0x3D, 0x3A, 0x4A), Color32::from_rgb(0xD8, 0xD4, 0xE8)),
            header_line: pick(dark, Color32::from_rgb(0xD2, 0xCF, 0xDA), Color32::from_rgb(0x35, 0x32, 0x44)),
            header_sel_bg: pick(dark, Color32::from_rgb(0xE2, 0xDF, 0xEA), Color32::from_rgb(0x33, 0x2F, 0x48)),
            header_sel_text: pick(dark, Color32::from_rgb(0x5B, 0x21, 0xB6), Color32::from_rgb(0xC4, 0xB5, 0xFD)),
            header_all_bg: pick(dark, Color32::from_rgb(0xDC, 0xD3, 0xF7), Color32::from_rgb(0x2E, 0x28, 0x48)),
            sel_fill: pick(dark, Color32::from_rgba_unmultiplied(0x7C, 0x3A, 0xED, 0x1C), Color32::from_rgba_unmultiplied(0xA7, 0x8B, 0xFA, 0x24)),
            sel_border: Color32::from_rgb(0x7C, 0x3A, 0xED),
            input_bg: pick(dark, Color32::WHITE, Color32::from_rgb(0x17, 0x15, 0x22)),
            input_border: pick(dark, Color32::from_rgb(0xD1, 0xD1, 0xD1), Color32::from_rgb(0x50, 0x50, 0x50)),
            tab_bar: pick(dark, Color32::from_rgb(0xEF, 0xED, 0xE8), Color32::from_rgb(0x14, 0x12, 0x1D)),
            tab_active: pick(dark, Color32::WHITE, Color32::from_rgb(0x1F, 0x1C, 0x28)),
            status_bar: pick(dark, Color32::from_rgb(0xEF, 0xED, 0xE8), Color32::from_rgb(0x10, 0x0E, 0x18)),
            cell_text: Color32::BLACK,
            menu_bg: pick(dark, Color32::WHITE, Color32::from_rgb(0x22, 0x1F, 0x2E)),
            shadow: pick(dark, Color32::from_black_alpha(40), Color32::from_black_alpha(90)),
            danger: pick(dark, Color32::from_rgb(0xC4, 0x2B, 0x1C), Color32::from_rgb(0xFF, 0x95, 0x85)),
            error_check_green: pick(dark, Color32::from_rgb(0x1E, 0x8E, 0x3E), Color32::from_rgb(0x4C, 0xC2, 0x6B)),
            warning_fill: Color32::from_rgb(0xF2, 0xB7, 0x05),
            warning_ink: Color32::BLACK,
            notice_warn_bg: pick(dark, Color32::from_rgb(0xFF, 0xF4, 0xCE), Color32::from_rgb(0x4A, 0x3F, 0x1E)),
            notice_warn_border: pick(dark, Color32::from_rgb(0xF0, 0xC3, 0x6E), Color32::from_rgb(0x7A, 0x67, 0x30)),
            notice_info_bg: pick(dark, Color32::from_rgb(0xEF, 0xEC, 0xFA), Color32::from_rgb(0x24, 0x20, 0x38)),
            notice_info_border: pick(dark, Color32::from_rgb(0xCE, 0xC6, 0xEA), Color32::from_rgb(0x3F, 0x39, 0x5A)),
            on_accent: pick(dark, Color32::WHITE, Color32::from_rgb(0x24, 0x14, 0x45)),
            cat: pick(
                dark,
                [rgb(0x2F6FCF), rgb(0x1E8A4C), rgb(0xB44F12), rgb(0xC8372D), rgb(0x8350C8), rgb(0x13808A), rgb(0x9C6C00), rgb(0x5A6B7D)],
                [rgb(0x6EA8F7), rgb(0x5CC587), rgb(0xF0965A), rgb(0xF07B6E), rgb(0xB394EE), rgb(0x4CC7CF), rgb(0xF2C14E), rgb(0xA9B6C4)],
            ),
            paper: pick(dark, Color32::WHITE, rgb(0x454545)),
            ref_colors: pick(
                dark,
                SHEET_REF_COLORS,
                [rgb(0x6FA3F5), rgb(0xF27C7C), rgb(0xB394EE), rgb(0x6CCB85), rgb(0xF0A061), rgb(0x5CC9DA), rgb(0xE77FC4), rgb(0xD4C25A)],
            ),
            sheet_ref_colors: SHEET_REF_COLORS,
            syntax: pick(dark, LIGHT_SYNTAX, DARK_SYNTAX),
            warning: rgb(0xF2C32E),
            on_warning: rgb(0x1F1F1F),
            sheet_control_bg: SHEET_CONTROL_BG,
            sheet_control_border: SHEET_CONTROL_BORDER,
            sheet_control_text: SHEET_CONTROL_TEXT,
            sheet_track: SHEET_TRACK,
            recording: pick(dark, rgb(0xC42B1C), rgb(0xF1705F)),
            breakpoint: pick(dark, rgb(0xD13438), rgb(0xF1707A)),
            sheet_macro_input: SHEET_MACRO_INPUT,
            sheet_macro_footprint: SHEET_MACRO_FOOTPRINT,
            sheet_macro_conflict: SHEET_MACRO_CONFLICT,
            sheet_macro_target: SHEET_MACRO_TARGET,
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        if ctx.global_style().visuals.dark_mode { Tokens::dark() } else { Tokens::light() }
    }

    /// One colour of the category palette.
    pub fn cat(&self, c: Cat) -> Color32 {
        self.cat.get(c as usize).copied().unwrap_or(self.text)
    }

    /// The reference colour for a formula's `index`-th reference, on the bar or on the sheet.
    pub fn ref_color(&self, index: usize, on_sheet: bool) -> Color32 {
        let set = if on_sheet { &self.sheet_ref_colors } else { &self.ref_colors };
        set.get(index % set.len()).copied().unwrap_or(self.text)
    }
}

/// The NOVA thread: the three-pixel violet gradient along the top edge of every NOVA Workspace
/// window. It is the one piece of chrome the four apps and NOVA Desk share, so the suite reads as
/// one product rather than four lookalikes. Drawn over the title bar, never over a full-screen view.
pub const THREAD_H: f32 = 3.0;

/// The thread, left to right: deep violet ink, NOVA violet, bright violet, orchid.
const THREAD_STOPS: [(f32, Color32); 4] = [
    (0.0, Color32::from_rgb(0x4C, 0x1D, 0x95)),
    (0.30, Color32::from_rgb(0x7C, 0x3A, 0xED)),
    (0.65, Color32::from_rgb(0xA8, 0x55, 0xF7)),
    (1.0, Color32::from_rgb(0xD9, 0x46, 0xEF)),
];

/// Blend two opaque colours; `t` is clamped to 0..=1.
fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 {
        let v = f32::from(x) + (f32::from(y) - f32::from(x)) * t;
        v.round().clamp(0.0, 255.0) as u8
    };
    Color32::from_rgb(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()))
}

/// The thread's colour at `t` (0 at the left edge, 1 at the right).
fn thread_color(t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let mut lo = THREAD_STOPS[0];
    for hi in &THREAD_STOPS[1..] {
        if t <= hi.0 {
            let span = (hi.0 - lo.0).max(f32::EPSILON);
            return mix(lo.1, hi.1, (t - lo.0) / span);
        }
        lo = *hi;
    }
    THREAD_STOPS[3].1
}

/// Paint the NOVA thread along the top edge of `screen`.
pub fn thread(painter: &egui::Painter, screen: egui::Rect) {
    let r = egui::Rect::from_min_max(screen.left_top(), egui::pos2(screen.right(), screen.top() + THREAD_H));
    let mut mesh = egui::Mesh::default();
    // Enough columns that the ramp is smooth on a wide window and cheap on a narrow one.
    const COLUMNS: u32 = 48;
    for i in 0..=COLUMNS {
        let t = i as f32 / COLUMNS as f32;
        let x = r.left() + r.width() * t;
        let c = thread_color(t);
        mesh.colored_vertex(egui::pos2(x, r.top()), c);
        mesh.colored_vertex(egui::pos2(x, r.bottom()), c);
        if i > 0 {
            let a = (i - 1) * 2;
            mesh.add_triangle(a, a + 1, a + 2);
            mesh.add_triangle(a + 1, a + 3, a + 2);
        }
    }
    painter.with_clip_rect(r).add(egui::Shape::mesh(mesh));
}

/// Light the title bar from above: its own colour at the bottom edge, 9% NOVA violet at the top.
/// Call first inside the bar, before its widgets, so they draw over the wash.
pub fn lit_bar(ui: &egui::Ui, fill: Color32) {
    let content = ui.max_rect();
    let screen = ui.ctx().content_rect();
    let bar = egui::Rect::from_min_max(egui::pos2(screen.left(), content.top()), egui::pos2(screen.right(), content.bottom()));
    if bar.height() <= 0.0 {
        return;
    }
    let lit = mix(fill, Color32::from_rgb(0x7C, 0x3A, 0xED), 0.09);
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(bar.left_top(), lit);
    mesh.colored_vertex(bar.right_top(), lit);
    mesh.colored_vertex(bar.right_bottom(), fill);
    mesh.colored_vertex(bar.left_bottom(), fill);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    ui.painter().with_clip_rect(bar).add(egui::Shape::mesh(mesh));
}

/// WCAG 2 relative luminance of an opaque colour.
pub fn luminance(c: Color32) -> f32 {
    let lin = |v: u8| {
        let v = f32::from(v) / 255.0;
        if v <= 0.040_45 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
}

/// WCAG 2 contrast ratio between two opaque colours (1 to 21).
pub fn contrast(a: Color32, b: Color32) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// Installs light and dark token visuals in both theme slots, keeps the 2-3 px widget and 8-10 px window/menu corner radii from 0.36, and sets spacing, text styles and the active theme.
pub fn apply(ctx: &egui::Context, dark: bool) {
    // Install both palettes before System can switch between the style slots.
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let dark = theme == egui::Theme::Dark;
        let t = if dark { Tokens::dark() } else { Tokens::light() };
        let mut v = if dark { Visuals::dark() } else { Visuals::light() };
        v.panel_fill = t.window;
        v.window_fill = t.menu_bg;
        v.extreme_bg_color = t.input_bg;
        v.selection.bg_fill = t.accent_soft;
        v.selection.stroke = Stroke::new(1.0, t.accent);
        v.hyperlink_color = t.accent;
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text);
        v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text);
        v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        v.widgets.hovered.weak_bg_fill = t.hover;
        v.widgets.active.weak_bg_fill = t.pressed;
        v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.input_border);
        // 0.37 raises the default widget corner radii; NOVA Grid keeps its 0.36 shape (#8617).
        v.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);
        v.widgets.inactive.corner_radius = egui::CornerRadius::same(2);
        v.widgets.hovered.corner_radius = egui::CornerRadius::same(3);
        v.widgets.active.corner_radius = egui::CornerRadius::same(2);
        v.widgets.open.corner_radius = egui::CornerRadius::same(2);
        v.window_corner_radius = egui::CornerRadius::same(10);
        v.menu_corner_radius = egui::CornerRadius::same(8);
        v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: t.shadow };
        ctx.set_visuals_of(theme, v);
    }
    ctx.options_mut(|o| o.fallback_theme = egui::Theme::Light);
    ctx.set_theme(if dark { egui::Theme::Dark } else { egui::Theme::Light });
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, ui_font(13.0));
        s.text_styles.insert(egui::TextStyle::Button, ui_font(13.0));
        s.text_styles.insert(egui::TextStyle::Small, ui_font(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, ui_bold(18.0));
        s.text_styles.insert(egui::TextStyle::Monospace, FontId::new(12.0, FontFamily::Name(MONO.into())));
    });
}

/// Model colour → egui colour.
pub fn color32(rgb: [u8; 3]) -> Color32 {
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

#[cfg(test)]
mod contrast_tests {
    use super::*;

    /// Body text needs 4.5:1 (WCAG AA); glyphs, icons and other graphics need 3:1.
    const TEXT: f32 = 4.5;
    const GLYPH: f32 = 3.0;

    /// Every foreground/background pair the chrome draws: ribbon, popups, dialogs, panes, formula
    /// bar, status bar, plus the colours that sit on the white sheet.
    fn pairs(t: &Tokens) -> Vec<(String, Color32, Color32, f32)> {
        let mut v: Vec<(String, Color32, Color32, f32)> = Vec::new();
        let mut add = |name: &str, fg: Color32, bg: Color32, min: f32| v.push((name.to_string(), fg, bg, min));
        let surfaces = [
            ("window", t.window),
            ("ribbon", t.ribbon),
            ("menu", t.menu_bg),
            ("input", t.input_bg),
            ("status bar", t.status_bar),
            ("tab bar", t.tab_bar),
            ("active tab", t.tab_active),
            ("hover", t.hover),
        ];
        for (name, bg) in surfaces {
            add(&format!("text on {name}"), t.text, bg, TEXT);
            add(&format!("dim text on {name}"), t.text_dim, bg, TEXT);
            add(&format!("accent glyph on {name}"), t.accent, bg, GLYPH);
            add(&format!("danger on {name}"), t.danger, bg, TEXT);
        }
        add("text on pressed", t.text, t.pressed, TEXT);
        add("accent text (File, contextual tabs) on window", t.accent, t.window, TEXT);
        for (name, bg) in [("ribbon", t.ribbon), ("menu", t.menu_bg), ("hover", t.hover), ("pressed", t.pressed)] {
            for c in Cat::ALL {
                add(&format!("{c:?} icon colour on {name}"), t.cat(c), bg, GLYPH);
            }
        }
        add("icon stroke on paper", t.text, t.paper, GLYPH);
        add("on_accent on accent", t.on_accent, t.accent, TEXT);
        add("warning glyph", t.on_warning, t.warning, TEXT);
        add("notice text (warning)", t.text, t.notice_warn_bg, TEXT);
        add("notice text (info)", t.text, t.notice_info_bg, TEXT);
        add("header text", t.header_text, t.header_bg, TEXT);
        add("selected header text", t.header_sel_text, t.header_sel_bg, TEXT);
        for (i, c) in t.ref_colors.iter().enumerate() {
            add(&format!("formula bar reference {i}"), *c, t.input_bg, TEXT);
        }
        let s = t.syntax;
        for (name, c) in [("keyword", s.keyword), ("string", s.string), ("number", s.number), ("comment", s.comment), ("builtin", s.builtin)] {
            add(&format!("code {name} on input"), c, t.input_bg, TEXT);
        }
        // On the sheet.
        add("cell text", t.cell_text, t.grid_bg, TEXT);
        for (i, c) in t.sheet_ref_colors.iter().enumerate() {
            add(&format!("sheet reference {i}"), *c, t.grid_bg, TEXT);
        }
        add("selection border on sheet", t.sel_border, t.grid_bg, GLYPH);
        add("sheet control text", t.sheet_control_text, t.sheet_control_bg, TEXT);
        add("sheet control border on sheet", t.sheet_control_border, t.grid_bg, GLYPH);
        add("recording dot on ribbon", t.recording, t.ribbon, GLYPH);
        add("recording dot on status bar", t.recording, t.status_bar, GLYPH);
        for (name, c) in [
            ("macro input", t.sheet_macro_input),
            ("macro footprint", t.sheet_macro_footprint),
            ("macro conflict", t.sheet_macro_conflict),
            ("macro target", t.sheet_macro_target),
        ] {
            add(&format!("{name} outline on sheet"), c, t.grid_bg, GLYPH);
        }
        // The macro step editor: breakpoint dots in the gutter, error markers, the current step.
        add("breakpoint on window", t.breakpoint, t.window, GLYPH);
        add("breakpoint on current step", t.breakpoint, t.accent_soft, GLYPH);
        add("text on current step", t.text, t.accent_soft, TEXT);
        add("current step arrow", t.accent, t.accent_soft, GLYPH);
        add("error marker on window", t.danger, t.window, GLYPH);
        v
    }

    #[test]
    fn token_pairs_meet_wcag_contrast_in_both_themes() {
        let mut failures = Vec::new();
        for t in [Tokens::light(), Tokens::dark()] {
            for (name, fg, bg, min) in pairs(&t) {
                let ratio = contrast(fg, bg);
                if ratio < min {
                    failures.push(format!("{} theme: {name} is {ratio:.2}:1, needs {min}:1", if t.dark { "dark" } else { "light" }));
                }
            }
        }
        assert!(failures.is_empty(), "contrast failures:\n{}", failures.join("\n"));
    }

    #[test]
    fn contrast_matches_wcag_reference_values() {
        assert!((contrast(Color32::BLACK, Color32::WHITE) - 21.0).abs() < 0.01);
        assert!((contrast(Color32::WHITE, Color32::WHITE) - 1.0).abs() < 0.001);
        // #767676 on white is the classic 4.54:1 grey.
        assert!((contrast(rgb(0x767676), Color32::WHITE) - 4.54).abs() < 0.01);
    }

    #[test]
    fn sheet_tokens_follow_the_sheet_not_the_theme() {
        let (l, d) = (Tokens::light(), Tokens::dark());
        assert_eq!(l.grid_bg, d.grid_bg);
        assert_eq!(l.sheet_ref_colors, d.sheet_ref_colors);
        assert_eq!(
            (l.sheet_control_bg, l.sheet_control_border, l.sheet_control_text, l.sheet_track),
            (d.sheet_control_bg, d.sheet_control_border, d.sheet_control_text, d.sheet_track)
        );
        assert_eq!(
            (l.sheet_macro_input, l.sheet_macro_footprint, l.sheet_macro_conflict, l.sheet_macro_target),
            (d.sheet_macro_input, d.sheet_macro_footprint, d.sheet_macro_conflict, d.sheet_macro_target)
        );
        for c in Cat::ALL {
            assert_ne!(l.cat(c), d.cat(c), "{c:?} has a dark-mode value of its own");
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
