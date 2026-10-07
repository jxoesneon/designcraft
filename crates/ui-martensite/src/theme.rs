//! Publishing theme design tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub pasteboard: Color,
    pub page_paper: Color,
    pub margin_guide: Color,
    pub bleed_guide: Color,
}

impl Theme {
    pub fn publishing_studio() -> Self {
        Self {
            pasteboard: Color(34, 37, 44),
            page_paper: Color(255, 255, 255),
            margin_guide: Color(220, 100, 180),
            bleed_guide: Color(220, 50, 50),
        }
    }
}
