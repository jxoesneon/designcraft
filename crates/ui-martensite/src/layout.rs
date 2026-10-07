//! Facing page spread and multi-column layout primitives.

#[derive(Clone, Debug, PartialEq)]
pub struct PageDimensions {
    pub width_pt: f32,
    pub height_pt: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_inside: f32,
    pub margin_outside: f32,
    pub columns: u32,
    pub column_gutter: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FacingSpread {
    pub left_page: Option<PageDimensions>,
    pub right_page: PageDimensions,
    pub bleed_pt: f32,
}

impl FacingSpread {
    pub fn new_a4() -> Self {
        let page = PageDimensions {
            width_pt: 595.28,
            height_pt: 841.89,
            margin_top: 36.0,
            margin_bottom: 36.0,
            margin_inside: 48.0,
            margin_outside: 36.0,
            columns: 2,
            column_gutter: 12.0,
        };
        Self {
            left_page: Some(page.clone()),
            right_page: page,
            bleed_pt: 9.0, // 3mm
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread_geometry() {
        let spread = FacingSpread::new_a4();
        assert_eq!(spread.right_page.columns, 2);
        assert_eq!(spread.bleed_pt, 9.0);
    }
}
