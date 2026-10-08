//! Tool strip widget: single/double column layout, tool flyouts, and fill/stroke chips.

use designcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Selection, alternatives: &[Tool::DirectSelection] },
    ToolSlot { primary: Tool::Type, alternatives: &[] },
    ToolSlot { primary: Tool::Line, alternatives: &[] },
    ToolSlot { primary: Tool::Pen, alternatives: &[Tool::Scissors] },
    ToolSlot { primary: Tool::Pencil, alternatives: &[] },
    ToolSlot { primary: Tool::RectangleFrame, alternatives: &[] },
    ToolSlot { primary: Tool::Rectangle, alternatives: &[] },
    ToolSlot { primary: Tool::FreeTransform, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    /// Paper, CMYK-ish RGBA chip: [r, g, b, a]. `None` = the "None" swatch.
    pub fill_color: Option<[u8; 4]>,
    pub stroke_color: Option<[u8; 4]>,
    /// Which chip has keyboard/swatches focus: true = fill, false = stroke.
    pub fill_in_front: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Selection,
            double_column: false,
            fill_color: None,                   // [None] like a fresh document
            stroke_color: Some([0, 0, 0, 255]), // Default black stroke
            fill_in_front: true,
        }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    /// Swap fill and stroke (Shift+X).
    pub fn swap_fill_stroke(&mut self) {
        std::mem::swap(&mut self.fill_color, &mut self.stroke_color);
        self.fill_in_front = !self.fill_in_front;
    }

    /// Apply the None swatch to the active chip (/).
    pub fn apply_none(&mut self) {
        if self.fill_in_front {
            self.fill_color = None;
        } else {
            self.stroke_color = None;
        }
    }

    /// Default fill/stroke (D): fill [None], stroke black — like a new frame.
    pub fn default_colors(&mut self) {
        self.fill_color = None;
        self.stroke_color = Some([0, 0, 0, 255]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Selection);
        assert!(!strip.double_column);
        assert_eq!(strip.fill_color, None);
        assert_eq!(strip.stroke_color, Some([0, 0, 0, 255]));

        strip.fill_color = Some([255, 0, 0, 255]);
        strip.swap_fill_stroke();
        assert_eq!(strip.stroke_color, Some([255, 0, 0, 255]));
        assert_eq!(strip.fill_color, Some([0, 0, 0, 255]));
        assert!(!strip.fill_in_front);

        strip.apply_none();
        assert_eq!(strip.stroke_color, None);

        strip.default_colors();
        assert_eq!(strip.fill_color, None);
        assert_eq!(strip.stroke_color, Some([0, 0, 0, 255]));

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());
    }
}
