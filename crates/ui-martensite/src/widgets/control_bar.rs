//! Control bar widget adapting dynamically to the active tool and selection.

use crate::widgets::scrubby_input::ScrubbyInputWidget;
use designcraft_engine::Tool;

pub struct ControlBarWidget {
    pub active_tool: Tool,
    pub x_pos: ScrubbyInputWidget,
    pub y_pos: ScrubbyInputWidget,
    pub width: ScrubbyInputWidget,
    pub height: ScrubbyInputWidget,
    pub font_size: ScrubbyInputWidget,
    pub leading: ScrubbyInputWidget,
    /// Reference point on the 3x3 proxy (0..=8, row-major from top-left).
    pub reference_point: u8,
    pub constrain_proportions: bool,
}

impl ControlBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Selection,
            x_pos: ScrubbyInputWidget::new("X", 0.0, -100000.0, 100000.0, "pt"),
            y_pos: ScrubbyInputWidget::new("Y", 0.0, -100000.0, 100000.0, "pt"),
            width: ScrubbyInputWidget::new("W", 100.0, 0.0, 100000.0, "pt"),
            height: ScrubbyInputWidget::new("H", 100.0, 0.0, 100000.0, "pt"),
            font_size: ScrubbyInputWidget::new("Size", 12.0, 0.1, 1296.0, "pt"),
            leading: ScrubbyInputWidget::new("Leading", 14.4, 0.0, 500.0, "pt"),
            reference_point: 0,
            constrain_proportions: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn set_reference_point(&mut self, point: u8) {
        self.reference_point = point.min(8);
    }

    pub fn toggle_constrain_proportions(&mut self) -> bool {
        self.constrain_proportions = !self.constrain_proportions;
        self.constrain_proportions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_bar_defaults() {
        let mut bar = ControlBarWidget::new();
        assert_eq!(bar.active_tool, Tool::Selection);
        assert_eq!(bar.font_size.value, 12.0);
        assert_eq!(bar.width.value, 100.0);
        assert!(!bar.constrain_proportions);

        bar.set_tool(Tool::Type);
        assert_eq!(bar.active_tool, Tool::Type);

        bar.set_reference_point(4);
        assert_eq!(bar.reference_point, 4);
        bar.set_reference_point(99);
        assert_eq!(bar.reference_point, 8);

        assert!(bar.toggle_constrain_proportions());
        assert!(bar.constrain_proportions);
    }
}
