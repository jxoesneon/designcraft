//! Comprehensive integration test suite for DesignCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, text wrap, and spread coordinates.

use designcraft_engine::{Engine, Tool};
use designcraft_ui_martensite::{
    DesigncraftApp,
    command_reg::{COMMAND_REGISTRY, find_command},
    menus::generate_main_menu,
    theme::CraftTheme,
    widgets::{ControlBarWidget, DockPanelGroup, PageItemDef, PagesPanelWidget, SpreadViewWidget, TextWrapWidget, ToolStripWidget},
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = DesigncraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Selection);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.rulers_visible);
    assert!(!app.show_baseline_grid);
    assert_eq!(app.spread.right_page.columns, 2);

    // 2. Keystroke Workflow: switch to Type, zoom in, hold space to pan
    let new_tool = app.keyboard.on_key_down("t", app.active_tool);
    assert_eq!(new_tool, Some(Tool::Type));
    app.set_tool(Tool::Type);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(Tool::Hand));
    app.set_tool(Tool::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores Type
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::Type));
    app.set_tool(Tool::Type);

    // 3. Control Bar Interaction for the active text frame
    let mut bar = ControlBarWidget::new();
    bar.set_tool(Tool::Type);
    bar.width.on_pointer_down(0.0);
    bar.width.on_pointer_move(20.0, false, false);
    bar.width.on_pointer_up();
    assert_eq!(bar.width.value, 120.0); // 100 + 20
    bar.set_reference_point(8);
    assert_eq!(bar.reference_point, 8);

    // 4. Pages Panel & Hierarchy Updates
    let mut pages = PagesPanelWidget::new();
    pages.pages.push(PageItemDef {
        id: 1,
        label: "A-Master".to_string(),
        applied_master: String::new(),
        is_master_spread: true,
        has_overrides: false,
        section_prefix: String::new(),
        expanded: false,
        children: vec![],
    });
    pages.pages.push(PageItemDef {
        id: 2,
        label: "1".to_string(),
        applied_master: "A-Master".to_string(),
        is_master_spread: false,
        has_overrides: true,
        section_prefix: String::new(),
        expanded: false,
        children: vec![],
    });
    pages.select_page(2);
    assert_eq!(pages.selected_page_id, Some(2));
    pages.apply_master(2, "B-Master");
    assert_eq!(pages.pages[1].applied_master, "B-Master");

    // 5. Text Wrap linked offsets
    let mut wrap = TextWrapWidget::new();
    wrap.set_offset_top(12.0);
    assert_eq!(wrap.offset_top, 12.0);
    assert_eq!(wrap.offset_bottom, 12.0);
    assert!(wrap.toggle_invert());

    // 6. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Pages", "Links", "Properties"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 7. Spread view coordinates
    let mut view = SpreadViewWidget::new(1190, 842);
    view.zoom_at(2.0, [0.0, 0.0]);
    assert_eq!(view.zoom, 2.0);
    assert_eq!(view.screen_to_spread([100.0, 100.0]), [50.0, 50.0]);

    // 8. Tool strip fill/stroke chips
    let mut strip = ToolStripWidget::new();
    assert_eq!(strip.fill_color, None);
    strip.swap_fill_stroke();
    assert_eq!(strip.fill_color, Some([0, 0, 0, 255]));

    // 9. Menu Generation Consistency
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {cmd_id}");
            }
        }
    }
    assert!(!COMMAND_REGISTRY.is_empty());

    // 10. Theme Color Space Consistency
    let theme = CraftTheme::dark_neutral();
    let studio = CraftTheme::publishing_studio();
    assert_ne!(theme.surface_app_bg, studio.surface_app_bg);
}
