//! Text wrap panel widget: wrap shape plus per-edge offsets in points.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrapShape {
    BoundingBox,
    DetectEdges,
    AlphaChannel,
    Contour,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextWrapWidget {
    pub shape: WrapShape,
    pub offset_top: f32,
    pub offset_bottom: f32,
    pub offset_inside: f32,
    pub offset_outside: f32,
    pub invert: bool,
    /// Single linked offset value when `offsets_linked` is on.
    pub offsets_linked: bool,
}

impl TextWrapWidget {
    pub fn new() -> Self {
        Self {
            shape: WrapShape::BoundingBox,
            offset_top: 0.0,
            offset_bottom: 0.0,
            offset_inside: 0.0,
            offset_outside: 0.0,
            invert: false,
            offsets_linked: true,
        }
    }

    pub fn set_shape(&mut self, shape: WrapShape) {
        self.shape = shape;
    }

    /// Set the top offset; when offsets are linked every edge follows.
    pub fn set_offset_top(&mut self, pts: f32) {
        let v = pts.max(0.0);
        self.offset_top = v;
        if self.offsets_linked {
            self.offset_bottom = v;
            self.offset_inside = v;
            self.offset_outside = v;
        }
    }

    pub fn toggle_link(&mut self) -> bool {
        self.offsets_linked = !self.offsets_linked;
        self.offsets_linked
    }

    pub fn toggle_invert(&mut self) -> bool {
        self.invert = !self.invert;
        self.invert
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_wrap_linked_offsets() {
        let mut wrap = TextWrapWidget::new();
        assert_eq!(wrap.shape, WrapShape::BoundingBox);
        assert!(wrap.offsets_linked);

        wrap.set_offset_top(9.0);
        assert_eq!(wrap.offset_top, 9.0);
        assert_eq!(wrap.offset_bottom, 9.0);
        assert_eq!(wrap.offset_inside, 9.0);
        assert_eq!(wrap.offset_outside, 9.0);

        assert!(!wrap.toggle_link());
        wrap.set_offset_top(24.0);
        assert_eq!(wrap.offset_top, 24.0);
        assert_eq!(wrap.offset_bottom, 9.0);

        wrap.set_shape(WrapShape::DetectEdges);
        assert_eq!(wrap.shape, WrapShape::DetectEdges);

        assert!(wrap.toggle_invert());
        assert!(wrap.invert);
    }
}
