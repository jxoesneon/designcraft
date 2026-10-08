//! Keystroke state machine providing InDesign keyboard ergonomics.

use designcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Standard InDesign single-key shortcuts
            "v" | "V" => Some(Tool::Selection),
            "a" | "A" => Some(Tool::DirectSelection),
            "t" | "T" => Some(Tool::Type),
            "\\" => Some(Tool::Line),
            "p" | "P" => Some(Tool::Pen),
            "n" | "N" => Some(Tool::Pencil),
            "f" | "F" => Some(Tool::RectangleFrame),
            "m" | "M" => Some(Tool::Rectangle),
            "c" | "C" => Some(Tool::Scissors),
            "e" | "E" => Some(Tool::FreeTransform),
            "h" | "H" => Some(Tool::Hand),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::Pen), Some(Tool::Selection));
        assert_eq!(k.on_key_down("A", Tool::Selection), Some(Tool::DirectSelection));
        assert_eq!(k.on_key_down("t", Tool::Selection), Some(Tool::Type));
        assert_eq!(k.on_key_down("\\", Tool::Type), Some(Tool::Line));
        assert_eq!(k.on_key_down("p", Tool::Line), Some(Tool::Pen));
        assert_eq!(k.on_key_down("n", Tool::Pen), Some(Tool::Pencil));
        assert_eq!(k.on_key_down("f", Tool::Pencil), Some(Tool::RectangleFrame));
        assert_eq!(k.on_key_down("m", Tool::RectangleFrame), Some(Tool::Rectangle));
        assert_eq!(k.on_key_down("c", Tool::Rectangle), Some(Tool::Scissors));
        assert_eq!(k.on_key_down("e", Tool::Scissors), Some(Tool::FreeTransform));
        assert_eq!(k.on_key_down("h", Tool::FreeTransform), Some(Tool::Hand));
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Type;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Selection;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
