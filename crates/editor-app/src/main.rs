//! The desktop app: a window with menus built from the feature catalog,
//! a canvas showing the document, and a status bar.
//!
//! Menu items whose command doesn't exist yet are shown greyed out, so the
//! whole target product is visible from day one.

mod menu;

use editor_catalog::Entry;
use editor_engine::{preview, Document, Engine};
use eframe::egui;
use serde_json::json;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Image Editor (starter)")
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "image-editor",
        options,
        Box::new(|_cc| Ok(Box::new(EditorApp::new()))),
    )
}

struct EditorApp {
    engine: Option<Engine>,
    catalog: Vec<Entry>,
    menus: Vec<menu::Node>,
    status: String,
    texture: Option<egui::TextureHandle>,
    /// Set whenever the document changes, so the canvas texture is rebuilt.
    dirty: bool,
}

impl EditorApp {
    fn new() -> Self {
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
            texture: None,
            dirty: true,
        }
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
                self.dirty = true;
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

    fn refresh_texture(&mut self, ctx: &egui::Context) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        let Some(engine) = self.engine.as_ref() else {
            self.texture = None;
            return;
        };
        let doc = engine.document();
        let pixels: Vec<u8> = preview::flatten(doc).into_iter().flatten().collect();
        let size = [doc.width() as usize, doc.height() as usize];
        let image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        self.texture = Some(ctx.load_texture("canvas", image, egui::TextureOptions::LINEAR));
    }
}

impl eframe::App for EditorApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        self.refresh_texture(&ctx);
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

        egui::CentralPanel::default().show(root, |ui| match &self.texture {
            Some(texture) => {
                let available = ui.available_size();
                let size = texture.size_vec2();
                let scale = (available.x / size.x).min(available.y / size.y).min(1.0);
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(size * scale));
                });
            }
            None => {
                ui.label("No document");
            }
        });

        if let Some(action) = shell {
            if let Some(engine) = self.engine.as_mut() {
                let changed = match action {
                    menu::Shell::Undo => engine.undo(),
                    menu::Shell::Redo => engine.redo(),
                };
                if changed {
                    self.dirty = true;
                }
            }
        }
        if let Some(id) = clicked {
            self.run_command(&id);
        }
    }
}
