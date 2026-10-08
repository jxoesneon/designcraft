//! Martensite design system tokens calibrated for print and page-layout workflows.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_f32_array(self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }

    pub fn luminance(&self) -> f32 {
        0.2126 * (self.r as f32 / 255.0) + 0.7152 * (self.g as f32 / 255.0) + 0.0722 * (self.b as f32 / 255.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CraftTheme {
    pub surface_app_bg: RgbaColor,
    pub surface_panel: RgbaColor,
    pub surface_pasteboard: RgbaColor,
    pub surface_page: RgbaColor,
    pub surface_input: RgbaColor,
    pub surface_active_tab: RgbaColor,
    pub surface_inactive_tab: RgbaColor,
    pub border_divider: RgbaColor,
    pub text_primary: RgbaColor,
    pub text_dimmed: RgbaColor,
    pub text_scrubby: RgbaColor,
    pub accent_active: RgbaColor,
    pub accent_focus: RgbaColor,
    pub guide_margin: RgbaColor,
    pub guide_bleed: RgbaColor,
    pub guide_baseline: RgbaColor,
    pub corner_radius: f32,
    pub widget_spacing: f32,
}

impl CraftTheme {
    /// Neutral dark chrome with a white page and DesignCraft green accents.
    pub fn dark_neutral() -> Self {
        Self {
            surface_app_bg: RgbaColor::rgb(50, 50, 50),
            surface_panel: RgbaColor::rgb(38, 38, 38),
            surface_pasteboard: RgbaColor::rgb(30, 30, 30),
            surface_page: RgbaColor::rgb(255, 255, 255),
            surface_input: RgbaColor::rgb(24, 24, 24),
            surface_active_tab: RgbaColor::rgb(38, 38, 38),
            surface_inactive_tab: RgbaColor::rgb(56, 56, 56),
            border_divider: RgbaColor::rgb(26, 26, 26),
            text_primary: RgbaColor::rgb(223, 223, 223),
            text_dimmed: RgbaColor::rgb(158, 158, 158),
            text_scrubby: RgbaColor::rgb(181, 181, 181),
            accent_active: RgbaColor::rgb(123, 181, 28),
            accent_focus: RgbaColor::rgb(156, 204, 68),
            guide_margin: RgbaColor::rgb(220, 100, 180),
            guide_bleed: RgbaColor::rgb(220, 50, 50),
            guide_baseline: RgbaColor::rgb(110, 160, 220),
            corner_radius: 4.0,
            widget_spacing: 6.0,
        }
    }

    /// Deep studio chrome for long production sessions.
    pub fn publishing_studio() -> Self {
        Self {
            surface_app_bg: RgbaColor::rgb(24, 27, 32),
            surface_panel: RgbaColor::rgb(28, 32, 39),
            surface_pasteboard: RgbaColor::rgb(34, 37, 44),
            surface_page: RgbaColor::rgb(255, 255, 255),
            surface_input: RgbaColor::rgb(15, 17, 21),
            surface_active_tab: RgbaColor::rgb(28, 32, 39),
            surface_inactive_tab: RgbaColor::rgb(40, 46, 57),
            border_divider: RgbaColor::rgb(48, 55, 67),
            text_primary: RgbaColor::rgb(240, 246, 252),
            text_dimmed: RgbaColor::rgb(139, 148, 158),
            text_scrubby: RgbaColor::rgb(170, 180, 195),
            accent_active: RgbaColor::rgb(123, 181, 28),
            accent_focus: RgbaColor::rgb(156, 204, 68),
            guide_margin: RgbaColor::rgb(220, 100, 180),
            guide_bleed: RgbaColor::rgb(220, 50, 50),
            guide_baseline: RgbaColor::rgb(110, 160, 220),
            corner_radius: 6.0,
            widget_spacing: 8.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversions() {
        let c = RgbaColor::rgba(255, 128, 0, 255);
        let f = c.to_f32_array();
        assert_eq!(f[0], 1.0);
        assert!((f[1] - 0.50196).abs() < 1e-4);
        assert_eq!(f[2], 0.0);
        assert_eq!(f[3], 1.0);
    }

    #[test]
    fn test_luminance_calculation() {
        let black = RgbaColor::rgb(0, 0, 0);
        let white = RgbaColor::rgb(255, 255, 255);
        assert_eq!(black.luminance(), 0.0);
        assert!((white.luminance() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_theme_contrast_sanity() {
        let theme = CraftTheme::publishing_studio();
        assert!(theme.text_primary.luminance() > theme.surface_panel.luminance() + 0.4);
        assert!(theme.surface_pasteboard.luminance() > theme.surface_input.luminance());
        assert!(theme.surface_page.luminance() > theme.surface_pasteboard.luminance());
    }
}
