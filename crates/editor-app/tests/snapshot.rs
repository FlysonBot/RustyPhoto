//! Snapshot tests: render the app headless and compare with the PNGs stored
//! in `tests/snapshots/`. How to take, look at and update snapshots:
//! `docs/recipes/check-ui.md`.
//!
//! Tolerance: a pixel "differs" when its color distance is over 0.6
//! (egui_kittest's per-pixel threshold, weighted YIQ distance; absorbs
//! anti-aliasing differences between GPUs and drivers). A test fails when
//! more than 0.1% of the image's pixels differ.
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
/// Status-bar area hidden before comparing: bottom-right corner.
const MASK_W: u32 = 400;
const MASK_H: u32 = 26;

fn harness_or_skip(test: &str) -> Option<egui_kittest::Harness<'static, editor_app::EditorApp>> {
    match snapshot::harness(WIDTH, HEIGHT) {
        Ok(h) => Some(h),
        Err(e) => {
            // Written straight to stderr (not eprintln!) so the test
            // runner doesn't hide it: the skip must show in CI logs.
            let _ = writeln!(
                std::io::stderr(),
                "SKIP {test}: no GPU adapter, snapshot test not run ({e})"
            );
            None
        }
    }
}

/// Renders, masks the live count, and compares with the stored PNG.
fn check(
    harness: &mut egui_kittest::Harness<'static, editor_app::EditorApp>,
    name: &str,
) -> Result<(), String> {
    let mut image = harness.render()?;
    let (w, h) = image.dimensions();
    for y in h.saturating_sub(MASK_H)..h {
        let background = *image.get_pixel(0, y);
        for x in w.saturating_sub(MASK_W)..w {
            image.put_pixel(x, y, background);
        }
    }
    let max_failed = (w as usize * h as usize) / 1000;
    let options = SnapshotOptions::new()
        .threshold(0.6)
        .max_failed_pixels(max_failed);
    try_image_snapshot_options(&image, name, &options).map_err(|e| e.to_string())
}

#[test]
fn startup() {
    let Some(mut harness) = harness_or_skip("startup") else {
        return;
    };
    assert_eq!(harness.state().status(), "Ready");
    check(&mut harness, "startup").unwrap();
}

#[test]
fn file_menu_open() {
    let Some(mut harness) = harness_or_skip("file_menu_open") else {
        return;
    };
    harness.get_by_label("File").click();
    harness.run();
    check(&mut harness, "file_menu_open").unwrap();
}
