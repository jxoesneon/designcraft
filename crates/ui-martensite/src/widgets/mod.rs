//! Martensite widget suite for DesignCraft.

pub mod control_bar;
pub mod dock_panel;
pub mod pages_panel;
pub mod scrubby_input;
pub mod spread_view;
pub mod text_wrap;
pub mod tool_strip;

pub use control_bar::ControlBarWidget;
pub use dock_panel::DockPanelGroup;
pub use pages_panel::{PageItemDef, PagesPanelWidget};
pub use scrubby_input::ScrubbyInputWidget;
pub use spread_view::SpreadViewWidget;
pub use text_wrap::TextWrapWidget;
pub use tool_strip::ToolStripWidget;
