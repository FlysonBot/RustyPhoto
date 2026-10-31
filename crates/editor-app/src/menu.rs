//! Builds the menu bar from the feature catalog.

use editor_catalog::Entry;
use editor_engine::Engine;
use eframe::egui;

/// One menu or submenu, in catalog order.
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    /// Indexes into the catalog.
    pub items: Vec<usize>,
}

/// Actions built into the app shell rather than the command registry.
pub enum Shell {
    Undo,
    Redo,
}

pub struct State<'a> {
    pub engine: Option<&'a Engine>,
}

/// Groups catalog entries by their `menu` path ("Image/Adjustments").
pub fn build(catalog: &[Entry]) -> Vec<Node> {
    let mut roots: Vec<Node> = Vec::new();
    for (index, entry) in catalog.iter().enumerate() {
        if entry.menu.is_empty() {
            continue; // tools and formats have no menu
        }
        let mut level = &mut roots;
        let parts: Vec<&str> = entry.menu.split('/').collect();
        let last = parts.len().saturating_sub(1);
        for (depth, part) in parts.into_iter().enumerate() {
            let pos = match level.iter().position(|n| n.name == part) {
                Some(p) => p,
                None => {
                    level.push(Node {
                        name: part.to_string(),
                        children: Vec::new(),
                        items: Vec::new(),
                    });
                    level.len().saturating_sub(1)
                }
            };
            let Some(node) = level.get_mut(pos) else {
                break;
            };
            if depth == last {
                node.items.push(index);
                break;
            }
            level = &mut node.children;
        }
    }
    roots
}

pub fn show(
    ui: &mut egui::Ui,
    node: &Node,
    catalog: &[Entry],
    state: &State,
    clicked: &mut Option<String>,
    shell: &mut Option<Shell>,
) {
    ui.menu_button(&node.name, |ui| {
        if node.name == "Edit" {
            let can_undo = state.engine.is_some_and(Engine::can_undo);
            let can_redo = state.engine.is_some_and(Engine::can_redo);
            if ui
                .add_enabled(can_undo, egui::Button::new("Undo"))
                .clicked()
            {
                *shell = Some(Shell::Undo);
            }
            if ui
                .add_enabled(can_redo, egui::Button::new("Redo"))
                .clicked()
            {
                *shell = Some(Shell::Redo);
            }
            ui.separator();
        }
        for child in &node.children {
            show(ui, child, catalog, state, clicked, shell);
        }
        for &index in &node.items {
            let Some(entry) = catalog.get(index) else {
                continue;
            };
            let exists = state.engine.is_some_and(|e| e.spec(&entry.id).is_some());
            let enabled = state.engine.is_some_and(|e| e.is_enabled(&entry.id));
            let mut button = egui::Button::new(&entry.label);
            if let Some(shortcut) = &entry.shortcut {
                button = button.shortcut_text(shortcut);
            }
            let response = ui.add_enabled(enabled, button);
            let response = if exists {
                response
            } else {
                response.on_disabled_hover_text(format!("Not built yet ({})", entry.id))
            };
            if response.clicked() {
                *clicked = Some(entry.id.clone());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, menu: &str) -> Entry {
        Entry {
            id: id.into(),
            menu: menu.into(),
            label: id.into(),
            shortcut: None,
            kind: "command".into(),
        }
    }

    #[test]
    fn nests_by_menu_path_in_catalog_order() {
        let catalog = vec![
            entry("a", "Image/Adjustments"),
            entry("b", "Image"),
            entry("c", "File"),
            entry("t", ""),
        ];
        let menus = build(&catalog);
        assert_eq!(menus.len(), 2);
        assert_eq!(menus[0].name, "Image");
        assert_eq!(menus[0].items, vec![1]);
        assert_eq!(menus[0].children[0].name, "Adjustments");
        assert_eq!(menus[0].children[0].items, vec![0]);
        assert_eq!(menus[1].name, "File");
    }
}
