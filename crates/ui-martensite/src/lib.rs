//! Sovereign retained-mode interface for DesignCraft built on the Martensite GUI engine.

pub mod command_reg;
pub mod layout;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use designcraft_engine::Engine;
use std::sync::{Arc, Mutex};

/// Application state container managing the Martensite GUI pipeline.
pub struct DesigncraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: designcraft_engine::Tool,
    pub spread: layout::FacingSpread,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    pub rulers_visible: bool,
    pub show_baseline_grid: bool,
    pub is_dirty: bool,
}

impl DesigncraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::publishing_studio(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: designcraft_engine::Tool::Selection,
            spread: layout::FacingSpread::new_a4(),
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            rulers_visible: true,
            show_baseline_grid: false,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: designcraft_engine::Tool) {
        self.active_tool = tool;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 64.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
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
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = DesigncraftApp::new(engine);
        assert_eq!(app.active_tool, designcraft_engine::Tool::Selection);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.rulers_visible);
        assert!(!app.show_baseline_grid);
        assert!(!app.is_dirty);
        assert_eq!(app.spread.right_page.columns, 2);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = DesigncraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 64.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = DesigncraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_toggles() {
        let engine = Engine::new();
        let mut app = DesigncraftApp::new(engine);

        assert!(app.rulers_visible);
        assert!(!app.toggle_rulers());
        assert!(!app.rulers_visible);
        assert!(app.toggle_rulers());

        assert!(!app.show_baseline_grid);
        assert!(app.toggle_baseline_grid());
        assert!(app.show_baseline_grid);
        assert!(!app.toggle_baseline_grid());
    }
}
