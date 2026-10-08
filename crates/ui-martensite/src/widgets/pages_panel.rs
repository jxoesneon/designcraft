//! Pages panel widget: document pages and parent (master) spreads.

#[derive(Clone, Debug, PartialEq)]
pub struct PageItemDef {
    pub id: u64,
    pub label: String,
    pub applied_master: String,
    pub is_master_spread: bool,
    pub has_overrides: bool,
    pub section_prefix: String,
    pub expanded: bool,
    pub children: Vec<PageItemDef>,
}

pub struct PagesPanelWidget {
    pub pages: Vec<PageItemDef>,
    pub selected_page_id: Option<u64>,
    pub active_master: String,
    pub spread_shuffle: bool,
}

impl PagesPanelWidget {
    pub fn new() -> Self {
        Self { pages: Vec::new(), selected_page_id: None, active_master: "A-Master".to_string(), spread_shuffle: true }
    }

    pub fn select_page(&mut self, id: u64) {
        self.selected_page_id = Some(id);
    }

    pub fn apply_master(&mut self, id: u64, master: &str) {
        if let Some(item) = find_page_mut(&mut self.pages, id) {
            item.applied_master = master.to_string();
            item.has_overrides = false;
        }
    }

    pub fn toggle_shuffle(&mut self) -> bool {
        self.spread_shuffle = !self.spread_shuffle;
        self.spread_shuffle
    }
}

fn find_page_mut(items: &mut [PageItemDef], id: u64) -> Option<&mut PageItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_page_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pages_panel_mutation() {
        let mut panel = PagesPanelWidget::new();
        panel.pages.push(PageItemDef {
            id: 1,
            label: "A-Master".to_string(),
            applied_master: String::new(),
            is_master_spread: true,
            has_overrides: false,
            section_prefix: String::new(),
            expanded: false,
            children: vec![],
        });
        panel.pages.push(PageItemDef {
            id: 2,
            label: "1".to_string(),
            applied_master: "A-Master".to_string(),
            is_master_spread: false,
            has_overrides: true,
            section_prefix: String::new(),
            expanded: false,
            children: vec![],
        });

        panel.select_page(2);
        assert_eq!(panel.selected_page_id, Some(2));

        panel.apply_master(2, "B-Master");
        assert_eq!(panel.pages[1].applied_master, "B-Master");
        assert!(!panel.pages[1].has_overrides);

        assert!(!panel.toggle_shuffle());
        assert!(panel.toggle_shuffle());
    }
}
