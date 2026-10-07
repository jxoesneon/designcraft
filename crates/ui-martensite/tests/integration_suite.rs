//! End-to-end integration test for publishing layout.

use designcraft_ui_martensite::{DesigncraftApp, layout::FacingSpread};

#[test]
fn test_publishing_workflow() {
    let mut app = DesigncraftApp::new();
    assert_eq!(app.spread.right_page.columns, 2);

    app.toggle_baseline_grid();
    assert!(app.show_baseline_grid);
}
