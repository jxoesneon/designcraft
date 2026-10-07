//! Publishing menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "Layout", items: &["layout.margins_columns"] },
    MenuCategory { title: "Type", items: &["type.insert_placeholder"] },
    MenuCategory { title: "View", items: &["view.grids"] },
];
