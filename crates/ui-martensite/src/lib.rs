//! Sovereign retained-mode desktop publishing interface for DesignCraft built on Martensite.

pub mod command_reg;
pub mod layout;
pub mod menus;
pub mod theme;

pub struct DesigncraftApp {
    pub spread: layout::FacingSpread,
    pub zoom: f32,
    pub show_baseline_grid: bool,
}

impl DesigncraftApp {
    pub fn new() -> Self {
        Self {
            spread: layout::FacingSpread::new_a4(),
            zoom: 1.0,
            show_baseline_grid: false,
        }
    }

    pub fn toggle_baseline_grid(&mut self) -> bool {
        self.show_baseline_grid = !self.show_baseline_grid;
        self.show_baseline_grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_designcraft_app() {
        let mut app = DesigncraftApp::new();
        assert!(!app.show_baseline_grid);
        assert!(app.toggle_baseline_grid());
        assert!(app.show_baseline_grid);
    }
}
