//! The canvas: shows the flattened document in the middle of the window.

use editor_engine::{preview, Engine};
use eframe::egui;

/// Holds the document's picture as a GPU texture.
pub struct Canvas {
    texture: Option<egui::TextureHandle>,
    /// Set whenever the document changes, so the texture is rebuilt.
    dirty: bool,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            texture: None,
            dirty: true,
        }
    }

    /// Ask for the texture to be rebuilt on the next frame.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Rebuild the texture from the document if it changed.
    pub fn refresh(&mut self, ctx: &egui::Context, engine: Option<&Engine>) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        let Some(engine) = engine else {
            self.texture = None;
            return;
        };
        let doc = engine.document();
        let pixels: Vec<u8> = preview::flatten(doc).into_iter().flatten().collect();
        let size = [doc.width() as usize, doc.height() as usize];
        let image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        self.texture = Some(ctx.load_texture("canvas", image, egui::TextureOptions::LINEAR));
    }

    /// Draw the central panel: the picture scaled down to fit, never up.
    pub fn show(&self, root: &mut egui::Ui) {
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
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
