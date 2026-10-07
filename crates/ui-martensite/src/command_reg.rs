//! Publishing command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "layout.margins_columns", label: "Margins and Columns…", shortcut: None },
    Command { id: "view.grids", label: "Show Baseline Grid", shortcut: Some("Alt+Cmd+'") },
    Command { id: "type.insert_placeholder", label: "Fill with Placeholder Text", shortcut: None },
];
