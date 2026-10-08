//! Spread view widget with subpixel pan/zoom, rulers, and page guides.

pub struct SpreadViewWidget {
    pub zoom: f32,
    pub pan_offset: [f32; 2],
    /// Spread size in points (facing pages side by side plus bleed).
    pub spread_size: [u32; 2],
    pub rulers_visible: bool,
    pub show_baseline_grid: bool,
    pub guides_phase: f32,
}

impl SpreadViewWidget {
    pub fn new(width: u32, height: u32) -> Self {
        Self { zoom: 1.0, pan_offset: [0.0, 0.0], spread_size: [width, height], rulers_visible: true, show_baseline_grid: false, guides_phase: 0.0 }
    }

    pub fn zoom_at(&mut self, factor: f32, cursor: [f32; 2]) {
        let old_zoom = self.zoom;
        let new_zoom = (self.zoom * factor).clamp(0.01, 64.0);
        let ratio = new_zoom / old_zoom;

        self.pan_offset[0] = cursor[0] - (cursor[0] - self.pan_offset[0]) * ratio;
        self.pan_offset[1] = cursor[1] - (cursor[1] - self.pan_offset[1]) * ratio;
        self.zoom = new_zoom;
    }

    pub fn advance_guides_animation(&mut self, dt: f32) {
        self.guides_phase = (self.guides_phase + dt * 2.0) % 1.0;
    }

    pub fn screen_to_spread(&self, screen: [f32; 2]) -> [f32; 2] {
        [(screen[0] - self.pan_offset[0]) / self.zoom, (screen[1] - self.pan_offset[1]) / self.zoom]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread_coordinates_and_zoom() {
        let mut view = SpreadViewWidget::new(1190, 842);
        assert_eq!(view.screen_to_spread([100.0, 100.0]), [100.0, 100.0]);

        view.zoom_at(2.0, [0.0, 0.0]);
        assert_eq!(view.zoom, 2.0);
        assert_eq!(view.screen_to_spread([100.0, 100.0]), [50.0, 50.0]);

        view.advance_guides_animation(0.25);
        assert!((view.guides_phase - 0.5).abs() < 1e-4);
    }
}
