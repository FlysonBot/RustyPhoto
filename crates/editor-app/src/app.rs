//! The app's state and UI: menus built from the feature catalog, the
//! canvas, and a status bar.
//!
//! Menu items whose command doesn't exist yet are shown greyed out, so the
//! whole target product is visible from day one.

use crate::canvas::Canvas;
use crate::menu;
use editor_catalog::Entry;
use editor_engine::{Document, Engine};
use eframe::egui;
use serde_json::json;

pub struct EditorApp {
    engine: Option<Engine>,
    catalog: Vec<Entry>,
    menus: Vec<menu::Node>,
    status: String,
    canvas: Canvas,
}

impl EditorApp {
    pub fn new() -> Self {
        let (catalog, status) = match editor_catalog::load() {
            Ok(c) => (c, "Ready".to_string()),
            Err(e) => (Vec::new(), format!("Catalog error: {e}")),
        };
        let engine = Document::new(800, 600, [255, 255, 255, 255])
            .ok()
            .map(Engine::new);
        let menus = menu::build(&catalog);
        EditorApp {
            engine,
            catalog,
            menus,
            status,
            canvas: Canvas::new(),
        }
    }

    /// The text shown on the left of the status bar.
    pub fn status(&self) -> &str {
        &self.status
    }

    fn run_command(&mut self, id: &str) {
        let Some(engine) = self.engine.as_mut() else {
            self.status = "No document".into();
            return;
        };
        // Commands run with default parameters for now. Parameter dialogs
        // generated from CommandSpec::params are a planned task.
        match engine.execute(id, &json!({})) {
            Ok(_) => {
                self.status = format!("Ran {id}");
                self.canvas.mark_dirty();
            }
            Err(e) => self.status = format!("{id}: {e}"),
        }
    }

    fn live_count(&self) -> usize {
        let Some(engine) = self.engine.as_ref() else {
            return 0;
        };
        self.catalog
            .iter()
            .filter(|e| engine.spec(&e.id).is_some())
            .count()
    }
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for EditorApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        self.canvas.refresh(&ctx, self.engine.as_ref());
        let mut clicked: Option<String> = None;
        let mut shell: Option<menu::Shell> = None;

        egui::Panel::top("menu_bar").show(root, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let state = menu::State {
                    engine: self.engine.as_ref(),
                };
                for node in &self.menus {
                    menu::show(ui, node, &self.catalog, &state, &mut clicked, &mut shell);
                }
            });
        });

        let total = self.catalog.len();
        let live = self.live_count();
        egui::Panel::bottom("status_bar").show(root, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{live} of {total} catalog features live"));
                });
            });
        });

        self.canvas.show(root);

        if let Some(action) = shell {
            if let Some(engine) = self.engine.as_mut() {
                let changed = match action {
                    menu::Shell::Undo => engine.undo(),
                    menu::Shell::Redo => engine.redo(),
                };
                if changed {
                    self.canvas.mark_dirty();
                }
            }
        }
        if let Some(id) = clicked {
            self.run_command(&id);
        }
    }
}
