//! Snapshot tests: render the app headless and compare with the PNGs stored
//! in `tests/snapshots/`. How to take, look at and update snapshots:
//! `docs/recipes/check-ui.md`.
//!
//! Tolerance: a pixel "differs" when its color distance is over 0.6
//! (egui_kittest's per-pixel threshold, weighted YIQ distance; absorbs
//! anti-aliasing differences between GPUs and drivers). A test fails when
//! more than MAX_FAILED_PIXELS (10) pixels differ. Measured 2026-10-31 on
//! Intel Iris Plus, Linux: 0 pixels differ on both Vulkan and OpenGL, so
//! 10 is only a small margin; a wrong icon or a moved 1px line is far more.
//!
//! The right end of the status bar ("N of M catalog features live") is
//! masked before comparing, because that count changes with every new
//! command and would otherwise break these tests in unrelated PRs.
//!
//! With no GPU adapter (real or software) the tests print SKIP and pass, so
//! CI stays green (ADR-008).

use editor_app::snapshot;
use egui_kittest::kittest::Queryable;
use egui_kittest::{try_image_snapshot_options, SnapshotOptions};
use std::io::Write;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
/// Pixels allowed past the per-pixel threshold (see the top of this file).
const MAX_FAILED_PIXELS: usize = 10;
/// Status-bar area hidden before comparing: bottom-right corner.
const MASK_W: u32 = 400;
const MASK_H: u32 = 26;

type AppHarness = egui_kittest::Harness<'static, editor_app::EditorApp>;

fn harness_or_skip(test: &str) -> Result<Option<AppHarness>, snapshot::SnapshotError> {
    // Only "no GPU adapter" is a skip; any other error fails the test.
    match snapshot::harness(WIDTH, HEIGHT) {
        Ok(h) => Ok(Some(h)),
        Err(e @ snapshot::SnapshotError::NoGpu(_)) => {
            // Written straight to stderr (not eprintln!) so the test
            // runner doesn't hide it: the skip must show in CI logs.
            let _ = writeln!(
                std::io::stderr(),
                "SKIP {test}: no GPU adapter, snapshot test not run ({e})"
            );
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

/// Renders, masks the live count, and compares with the stored PNG.
fn check(harness: &mut AppHarness, name: &str) -> Result<(), String> {
    let mut image = harness.render()?;
    let (w, h) = image.dimensions();
    for y in h.saturating_sub(MASK_H)..h {
        let background = *image.get_pixel(0, y);
        for x in w.saturating_sub(MASK_W)..w {
            image.put_pixel(x, y, background);
        }
    }
    let options = SnapshotOptions::new()
        .threshold(0.6)
        .max_failed_pixels(MAX_FAILED_PIXELS);
    try_image_snapshot_options(&image, name, &options).map_err(|e| e.to_string())
}

#[test]
fn startup() {
    let Some(mut harness) = harness_or_skip("startup").unwrap() else {
        return;
    };
    assert_eq!(harness.state().status(), "Ready");
    check(&mut harness, "startup").unwrap();
}

#[test]
fn file_menu_open() {
    let Some(mut harness) = harness_or_skip("file_menu_open").unwrap() else {
        return;
    };
    harness.get_by_label("File").click();
    harness.run();
    check(&mut harness, "file_menu_open").unwrap();
}
