//! DAW console theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub console_bg: Color,
    pub fader_cap: Color,
    pub meter_green: Color,
}

impl Theme {
    pub fn console_dark() -> Self {
        Self {
            console_bg: Color(24, 26, 30),
            fader_cap: Color(180, 190, 205),
            meter_green: Color(40, 220, 100),
        }
    }
}
