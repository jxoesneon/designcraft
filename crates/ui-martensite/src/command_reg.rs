//! Decoupled command catalog and taxonomy for DesignCraft.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Layout,
    Type,
    Object,
    View,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "file.new", label: "Document…", category: CommandCategory::File, default_shortcut: Some("Cmd+N"), secondary_shortcut: None },
    CommandSpec { id: "file.open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Cmd+O"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Cmd+S"), secondary_shortcut: None },
    CommandSpec {
        id: "file.saveAs",
        label: "Save As…",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+Shift+S"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "file.place", label: "Place…", category: CommandCategory::File, default_shortcut: Some("Cmd+D"), secondary_shortcut: None },
    CommandSpec {
        id: "file.exportPdf",
        label: "Export: Adobe PDF…",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+E"),
        secondary_shortcut: None,
    },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Shift+Z"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.selectAll",
        label: "Select All",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+A"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.deselectAll",
        label: "Deselect All",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+Shift+A"),
        secondary_shortcut: None,
    },
    // Layout
    CommandSpec {
        id: "layout.marginsAndColumns",
        label: "Margins and Columns…",
        category: CommandCategory::Layout,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "layout.documentSetup",
        label: "Document Setup…",
        category: CommandCategory::Layout,
        default_shortcut: Some("Cmd+Alt+P"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "layout.createGuides",
        label: "Create Guides…",
        category: CommandCategory::Layout,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // Type
    CommandSpec {
        id: "type.fillWithPlaceholder",
        label: "Fill with Placeholder Text",
        category: CommandCategory::Type,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "type.bold", label: "Bold", category: CommandCategory::Type, default_shortcut: Some("Cmd+Shift+B"), secondary_shortcut: None },
    CommandSpec {
        id: "type.italic",
        label: "Italic",
        category: CommandCategory::Type,
        default_shortcut: Some("Cmd+Shift+I"),
        secondary_shortcut: None,
    },
    // Object
    CommandSpec { id: "object.group", label: "Group", category: CommandCategory::Object, default_shortcut: Some("Cmd+G"), secondary_shortcut: None },
    CommandSpec {
        id: "object.ungroup",
        label: "Ungroup",
        category: CommandCategory::Object,
        default_shortcut: Some("Cmd+Shift+G"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "object.lock", label: "Lock", category: CommandCategory::Object, default_shortcut: Some("Cmd+L"), secondary_shortcut: None },
    // View
    CommandSpec {
        id: "view.fitSpread",
        label: "Fit Spread in Window",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+Alt+0"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.actualSize",
        label: "Actual Size",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+1"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.rulers", label: "Rulers", category: CommandCategory::View, default_shortcut: Some("Cmd+R"), secondary_shortcut: None },
    CommandSpec {
        id: "view.baselineGrid",
        label: "Baseline Grid",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+Alt+'"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.guides", label: "Guides", category: CommandCategory::View, default_shortcut: Some("Cmd+;"), secondary_shortcut: None },
    CommandSpec {
        id: "view.togglePreview",
        label: "Preview",
        category: CommandCategory::View,
        default_shortcut: Some("W"),
        secondary_shortcut: None,
    },
    // Window
    CommandSpec {
        id: "window.hidePanels",
        label: "Show/Hide Panels",
        category: CommandCategory::Window,
        default_shortcut: Some("Tab"),
        secondary_shortcut: None,
    },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.map(|c| c.label), Some(cmd.label));
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Layout).is_empty());
        assert!(!commands_by_category(CommandCategory::Type).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}
