//! Martensite design-system tokens calibrated for console and timeline workflows.

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
    pub surface_timeline_bg: RgbaColor,
    pub surface_input: RgbaColor,
    pub surface_active_tab: RgbaColor,
    pub surface_inactive_tab: RgbaColor,
    pub border_divider: RgbaColor,
    pub text_primary: RgbaColor,
    pub text_dimmed: RgbaColor,
    pub text_scrubby: RgbaColor,
    pub accent_active: RgbaColor,
    pub accent_focus: RgbaColor,
    pub meter_green: RgbaColor,
    pub meter_yellow: RgbaColor,
    pub meter_red: RgbaColor,
    pub fader_cap: RgbaColor,
    pub record_arm: RgbaColor,
    pub corner_radius: f32,
    pub widget_spacing: f32,
}

impl CraftTheme {
    pub fn console_dark() -> Self {
        Self {
            surface_app_bg: RgbaColor::rgb(30, 32, 36),
            surface_panel: RgbaColor::rgb(38, 41, 47),
            surface_timeline_bg: RgbaColor::rgb(24, 26, 30),
            surface_input: RgbaColor::rgb(20, 22, 26),
            surface_active_tab: RgbaColor::rgb(38, 41, 47),
            surface_inactive_tab: RgbaColor::rgb(52, 56, 64),
            border_divider: RgbaColor::rgb(22, 24, 27),
            text_primary: RgbaColor::rgb(228, 232, 238),
            text_dimmed: RgbaColor::rgb(150, 156, 166),
            text_scrubby: RgbaColor::rgb(180, 186, 196),
            accent_active: RgbaColor::rgb(64, 170, 255),
            accent_focus: RgbaColor::rgb(96, 150, 220),
            meter_green: RgbaColor::rgb(40, 220, 100),
            meter_yellow: RgbaColor::rgb(240, 200, 60),
            meter_red: RgbaColor::rgb(235, 70, 60),
            fader_cap: RgbaColor::rgb(180, 190, 205),
            record_arm: RgbaColor::rgb(230, 60, 50),
            corner_radius: 4.0,
            widget_spacing: 6.0,
        }
    }

    pub fn studio_light() -> Self {
        Self {
            surface_app_bg: RgbaColor::rgb(214, 216, 220),
            surface_panel: RgbaColor::rgb(226, 228, 232),
            surface_timeline_bg: RgbaColor::rgb(238, 240, 243),
            surface_input: RgbaColor::rgb(246, 247, 249),
            surface_active_tab: RgbaColor::rgb(226, 228, 232),
            surface_inactive_tab: RgbaColor::rgb(202, 205, 210),
            border_divider: RgbaColor::rgb(188, 191, 196),
            text_primary: RgbaColor::rgb(28, 30, 34),
            text_dimmed: RgbaColor::rgb(96, 100, 108),
            text_scrubby: RgbaColor::rgb(70, 74, 82),
            accent_active: RgbaColor::rgb(0, 122, 204),
            accent_focus: RgbaColor::rgb(30, 110, 190),
            meter_green: RgbaColor::rgb(30, 160, 70),
            meter_yellow: RgbaColor::rgb(200, 160, 30),
            meter_red: RgbaColor::rgb(210, 50, 40),
            fader_cap: RgbaColor::rgb(90, 96, 106),
            record_arm: RgbaColor::rgb(200, 40, 32),
            corner_radius: 4.0,
            widget_spacing: 6.0,
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
        for theme in [CraftTheme::console_dark(), CraftTheme::studio_light()] {
            let text = theme.text_primary.luminance();
            let panel = theme.surface_panel.luminance();
            assert!((text - panel).abs() > 0.3);
        }
    }
}
