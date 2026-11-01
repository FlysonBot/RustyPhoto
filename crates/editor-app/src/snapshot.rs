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
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Default snapshot size, the same as the window's starting size.
pub const DEFAULT_SIZE: (u32, u32) = (1200, 800);

/// Largest width or height we accept, to keep memory bounded.
const MAX_SIDE: u32 = 8192;

#[derive(Debug)]
pub enum SnapshotError {
    /// No GPU and no software renderer: snapshots can't be drawn here.
    NoGpu(String),
    /// A flag is missing its value, e.g. `--snapshot` with no path.
    BadArgs(String),
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
            SnapshotError::BadArgs(s) => write!(f, "{s}"),
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
    check_size(w, h).map_err(|_| bad())?;
    Ok((w, h))
}

/// Rejects sizes outside 1..=MAX_SIDE (zero-sized textures crash wgpu).
fn check_size(width: u32, height: u32) -> Result<(), SnapshotError> {
    if (1..=MAX_SIDE).contains(&width) && (1..=MAX_SIDE).contains(&height) {
        Ok(())
    } else {
        Err(SnapshotError::BadSize(format!("{width}x{height}")))
    }
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
    check_size(width, height)?;
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

/// A snapshot request from the command line.
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
}

/// Reads `--snapshot PATH [--size WxH]`. `Ok(None)` means `--snapshot`
/// is absent; a flag without its value is an error, never ignored.
pub fn parse_args(args: &[String]) -> Result<Option<Request>, SnapshotError> {
    let value = |flag: &str| -> Result<Option<&String>, SnapshotError> {
        let Some(i) = args.iter().position(|a| a == flag) else {
            return Ok(None);
        };
        match args.get(i + 1) {
            Some(v) if !v.starts_with("--") => Ok(Some(v)),
            _ => Err(SnapshotError::BadArgs(format!("{flag} needs a value"))),
        }
    };
    let Some(path) = value("--snapshot")? else {
        return Ok(None);
    };
    let (width, height) = match value("--size")? {
        Some(text) => parse_size(text)?,
        None => DEFAULT_SIZE,
    };
    Ok(Some(Request {
        path: PathBuf::from(path),
        width,
        height,
    }))
}

/// Handles the command line. `None` when `--snapshot` is absent (open the
/// window as usual), else the exit code: 0 on success, 2 on any error
/// (message on stderr).
pub fn run_from_args(args: &[String]) -> Option<ExitCode> {
    let result = match parse_args(args) {
        Ok(None) => return None,
        Ok(Some(r)) => write_png(&r.path, r.width, r.height),
        Err(e) => Err(e),
    };
    Some(match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("snapshot: {e}");
            ExitCode::from(2)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_arguments() {
        assert!(parse_args(&args(&[])).unwrap().is_none());
        let r = parse_args(&args(&["--snapshot", "a.png"]))
            .unwrap()
            .unwrap();
        assert_eq!(
            (r.path, r.width, r.height),
            (PathBuf::from("a.png"), 1200, 800)
        );
        let r = parse_args(&args(&["--size", "3x4", "--snapshot", "a.png"]))
            .unwrap()
            .unwrap();
        assert_eq!((r.width, r.height), (3, 4));
        for bad in [
            &["--snapshot"][..],
            &["--snapshot", "--size", "3x4", "a.png"],
            &["--snapshot", "a.png", "--size"],
            &["--snapshot", "a.png", "--size", "0x4"],
        ] {
            assert!(parse_args(&args(bad)).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn harness_rejects_zero_size() {
        assert!(matches!(harness(0, 10), Err(SnapshotError::BadSize(_))));
    }

    #[test]
    fn parses_sizes() {
        assert_eq!(parse_size("1200x800").unwrap(), (1200, 800));
        assert_eq!(parse_size("3X4").unwrap(), (3, 4));
        for bad in ["", "1200", "0x5", "5x0", "axb", "99999x2", "-1x2"] {
            assert!(parse_size(bad).is_err(), "{bad}");
        }
    }
}
