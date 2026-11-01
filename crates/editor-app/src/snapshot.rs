//! Renders the app to a PNG without opening a window, so agents (and the
//! snapshot tests) can see the UI. Uses `egui_kittest`, the egui authors'
//! test harness, which draws frames with `wgpu` on a GPU or a software
//! renderer (ADR-008).
//!
//! `egui_kittest` itself crashes when no graphics adapter exists, so we
//! create the renderer ourselves first and turn "no adapter" into an error.

use crate::EditorApp;
use eframe::egui;
use egui_kittest::wgpu::{default_wgpu_setup, WgpuTestRenderer};
use egui_kittest::Harness;
use std::fmt;
use std::path::Path;
use std::process::ExitCode;

/// Default snapshot size, the same as the window's starting size.
pub const DEFAULT_SIZE: (u32, u32) = (1200, 800);

/// Largest width or height we accept, to keep memory bounded.
const MAX_SIDE: u32 = 8192;

#[derive(Debug)]
pub enum SnapshotError {
    /// No GPU and no software renderer: snapshots can't be drawn here.
    NoGpu(String),
    /// `--size` wasn't like `1200x800`.
    BadSize(String),
    /// Drawing the frame failed.
    Render(String),
    /// Writing the PNG file failed.
    Save(String),
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnapshotError::NoGpu(e) => write!(f, "no GPU adapter (real or software) found: {e}"),
            SnapshotError::BadSize(s) => {
                write!(
                    f,
                    "bad size {s:?}: expected WIDTHxHEIGHT, e.g. 1200x800, each 1..={MAX_SIDE}"
                )
            }
            SnapshotError::Render(e) => write!(f, "rendering failed: {e}"),
            SnapshotError::Save(e) => write!(f, "saving the PNG failed: {e}"),
        }
    }
}

impl std::error::Error for SnapshotError {}

/// Parses `1200x800` into (1200, 800).
pub fn parse_size(text: &str) -> Result<(u32, u32), SnapshotError> {
    let bad = || SnapshotError::BadSize(text.to_string());
    let (w, h) = text.split_once(['x', 'X']).ok_or_else(bad)?;
    let w: u32 = w.trim().parse().map_err(|_| bad())?;
    let h: u32 = h.trim().parse().map_err(|_| bad())?;
    if w == 0 || h == 0 || w > MAX_SIDE || h > MAX_SIDE {
        return Err(bad());
    }
    Ok((w, h))
}

/// Creates a fresh wgpu renderer, or says why there is none.
fn renderer() -> Result<WgpuTestRenderer, SnapshotError> {
    let setup = default_wgpu_setup();
    let instance = pollster::block_on(setup.new_instance());
    let config = egui_wgpu::WgpuConfiguration {
        wgpu_setup: setup,
        ..Default::default()
    };
    let state = pollster::block_on(egui_wgpu::RenderState::create(
        &config,
        &instance,
        None,
        egui_wgpu::RendererOptions::PREDICTABLE,
    ))
    .map_err(|e| SnapshotError::NoGpu(e.to_string()))?;
    Ok(WgpuTestRenderer::from_render_state(state))
}

/// True if snapshots can be drawn on this machine.
pub fn gpu_available() -> bool {
    renderer().is_ok()
}

/// Runs the app headless at `width` x `height` logical pixels, settled
/// (layout done, no pending repaints). Tests click and query through it.
pub fn harness(width: u32, height: u32) -> Result<Harness<'static, EditorApp>, SnapshotError> {
    let renderer = renderer()?;
    let mut harness = Harness::builder()
        .with_size(egui::vec2(width as f32, height as f32))
        .renderer(renderer)
        .build_eframe(|_cc| EditorApp::new());
    // Never steps past its limit and never panics; a frame that keeps
    // asking for repaints is still drawn as it is.
    let _ = harness.run_ok();
    Ok(harness)
}

/// Draws one settled frame of the main window and saves it as a PNG.
pub fn write_png(path: &Path, width: u32, height: u32) -> Result<(), SnapshotError> {
    let mut harness = harness(width, height)?;
    let image = harness.render().map_err(SnapshotError::Render)?;
    image
        .save(path)
        .map_err(|e| SnapshotError::Save(e.to_string()))
}

/// Handles `--snapshot PATH [--size WxH]` from the command line.
/// Returns `None` when `--snapshot` is absent (open the window as usual),
/// else the exit code: 0 on success, 2 on any error (message on stderr).
pub fn run_from_args(args: &[String]) -> Option<ExitCode> {
    let value = |flag: &str| {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
    };
    let path = value("--snapshot")?;
    let size = value("--size").map_or(Ok(DEFAULT_SIZE), |s| parse_size(s));
    Some(
        match size.and_then(|(w, h)| write_png(Path::new(path), w, h)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("snapshot: {e}");
                ExitCode::from(2)
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sizes() {
        assert_eq!(parse_size("1200x800").unwrap(), (1200, 800));
        assert_eq!(parse_size("3X4").unwrap(), (3, 4));
        for bad in ["", "1200", "0x5", "5x0", "axb", "99999x2", "-1x2"] {
            assert!(parse_size(bad).is_err(), "{bad}");
        }
    }
}
