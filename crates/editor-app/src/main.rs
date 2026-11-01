//! Starts the editor. `editor --snapshot out.png [--size 1200x800]` instead
//! renders the main window to a PNG without opening a window, then exits.

use editor_app::{snapshot, EditorApp};
use eframe::egui;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(code) = snapshot::run_from_args(&args) {
        return code;
    }
    let (w, h) = snapshot::DEFAULT_SIZE;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Image Editor (starter)")
            .with_inner_size([w as f32, h as f32]),
        ..Default::default()
    };
    let app = Box::new(|_cc: &eframe::CreationContext<'_>| {
        Ok(Box::new(EditorApp::new()) as Box<dyn eframe::App>)
    });
    match eframe::run_native("image-editor", options, app) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("editor: {e}");
            ExitCode::FAILURE
        }
    }
}
