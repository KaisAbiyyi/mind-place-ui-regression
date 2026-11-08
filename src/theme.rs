use gpui::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DensityMode {
    Comfortable,
    Compact,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
    // Backgrounds
    pub bg_app: Rgba,
    pub bg_surface: Rgba,
    pub bg_elevated: Rgba,
    pub bg_subtle: Rgba,
    pub bg_muted: Rgba,
    pub bg_hover: Rgba,
    pub bg_active: Rgba,
    pub bg_inverse: Rgba,
    pub bg_inverse_subtle: Rgba,
    pub bg_glass: Rgba,
    pub bg_backdrop: Rgba,

    // Text
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,
    pub text_faint: Rgba,
    pub text_inverse: Rgba,

    // Borders
    pub border_subtle: Rgba,
    pub border_strong: Rgba,
    pub border_focus: Rgba,

    // Canvas / Hive Graph
    pub canvas_bg: Rgba,
    pub canvas_grid: Rgba,
    pub canvas_ring: Rgba,
    pub canvas_edge: Rgba,
    pub canvas_edge_active: Rgba,
    pub canvas_node_fill: Rgba,
    pub canvas_node_secondary: Rgba,
    pub canvas_node_tertiary: Rgba,
    pub canvas_node_stroke: Rgba,
    pub canvas_label: Rgba,
    pub canvas_label_muted: Rgba,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            bg_app: rgb(0xF4F4F5),
            bg_surface: rgb(0xFFFFFF),
            bg_elevated: rgb(0xFFFFFF),
            bg_subtle: rgb(0xFAFAFA),
            bg_muted: rgb(0xE4E4E7),
            bg_hover: rgb(0xF0F0F2),
            bg_active: rgb(0xE4E4E7),
            bg_inverse: rgb(0x09090B),
            bg_inverse_subtle: rgb(0x18181B),
            bg_glass: rgba(0xFFFFFFE0),
            bg_backdrop: rgba(0x09090B7A),

            text_primary: rgb(0x09090B),
            text_secondary: rgb(0x3F3F46),
            text_muted: rgb(0x71717A),
            text_faint: rgb(0xA1A1AA),
            text_inverse: rgb(0xFAFAFA),

            border_subtle: rgb(0xE4E4E7),
            border_strong: rgb(0xD4D4D8),
            border_focus: rgb(0x09090B),

            canvas_bg: rgb(0xF4F4F5),
            canvas_grid: rgba(0x09090B0E),
            canvas_ring: rgba(0x09090B14),
            canvas_edge: rgba(0x09090B24),
            canvas_edge_active: rgba(0x09090BB8),
            canvas_node_fill: rgb(0x18181B),
            canvas_node_secondary: rgb(0x52525B),
            canvas_node_tertiary: rgb(0xA1A1AA),
            canvas_node_stroke: rgb(0xFFFFFF),
            canvas_label: rgb(0x09090B),
            canvas_label_muted: rgb(0x52525B),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            bg_app: rgb(0x09090B),
            bg_surface: rgb(0x121215),
            bg_elevated: rgb(0x18181B),
            bg_subtle: rgb(0x18181B),
            bg_muted: rgb(0x27272A),
            bg_hover: rgb(0x1F1F23),
            bg_active: rgb(0x27272A),
            bg_inverse: rgb(0xFAFAFA),
            bg_inverse_subtle: rgb(0xE4E4E7),
            bg_glass: rgba(0x121215E0),
            bg_backdrop: rgba(0x000000BD),

            text_primary: rgb(0xFAFAFA),
            text_secondary: rgb(0xA1A1AA),
            text_muted: rgb(0x71717A),
            text_faint: rgb(0x52525B),
            text_inverse: rgb(0x09090B),

            border_subtle: rgb(0x27272A),
            border_strong: rgb(0x3F3F46),
            border_focus: rgb(0xFAFAFA),

            canvas_bg: rgb(0x0C0C0E),
            canvas_grid: rgba(0xFAFAFA0C),
            canvas_ring: rgba(0xFAFAFA12),
            canvas_edge: rgba(0xFAFAFA24),
            canvas_edge_active: rgba(0xFAFAFAC7),
            canvas_node_fill: rgb(0xFAFAFA),
            canvas_node_secondary: rgb(0xA1A1AA),
            canvas_node_tertiary: rgb(0x52525B),
            canvas_node_stroke: rgb(0x09090B),
            canvas_label: rgb(0xFAFAFA),
            canvas_label_muted: rgb(0xA1A1AA),
        }
    }
}

pub const RADIUS_XS: Pixels = px(5.0);
pub const RADIUS_SM: Pixels = px(7.0);
pub const RADIUS_MD: Pixels = px(10.0);
pub const RADIUS_LG: Pixels = px(14.0);

pub const FONT_SANS: &str = "Segoe UI";
pub const FONT_MONO: &str = "Consolas";
