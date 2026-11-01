---
id: TASK-1
title: 'Offscreen UI snapshot: render the app to PNG (gap G1)'
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-10-31T23:45:00Z'
updated_date: '2026-11-01T02:53:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-0
dependencies: []
references:
  - docs/decisions/ADR-008-ui-snapshots.md
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents cannot see the app, so every UI change waits for the owner to look (AGENTS.md rule 6). This gives agents eyes. It also splits the app into a library (app.rs, canvas.rs) so later UI tasks each touch their own file. Follows ADR-008 (wait until it is Accepted).

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-app/Cargo.toml`
- `crates/editor-app/src/main.rs`
- `crates/editor-app/src/lib.rs` (new)
- `crates/editor-app/src/app.rs` (new)
- `crates/editor-app/src/canvas.rs` (new)
- `crates/editor-app/src/menu.rs`
- `crates/editor-app/src/snapshot.rs` (new)
- `crates/editor-app/tests/snapshot.rs` (new)
- `crates/editor-app/tests/snapshots/` (new)
- `AGENTS.md` (rule 6 only)
- `docs/recipes/check-ui.md` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `cargo run -p editor-app -- --snapshot out.png --size 1200x800` writes a 1200x800 PNG of the main window and exits with code 0 without opening a window
- [x] #2 The app's state and UI live in `crates/editor-app/src/app.rs` (library); `main.rs` is under 40 lines; canvas drawing lives in `canvas.rs`
- [x] #3 At least 2 snapshot tests (startup window; File menu open) compare against stored PNGs with a documented tolerance
- [x] #4 Snapshot tests skip with a printed notice (not fail) when no GPU adapter exists, so `cargo xtask ci` stays green on CI
- [x] #5 `docs/recipes/check-ui.md` explains in 10 lines or fewer how an agent takes and looks at a snapshot; AGENTS.md rule 6 points to it
- [x] #6 `cargo xtask ci` passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Plan (opus-1). Worktree ../t1-ui-snapshot, branch t1-ui-snapshot, CARGO_TARGET_DIR=target/agent-opus-1.

1. Split the app into a library (no behavior change):
   - src/lib.rs: declares the modules below.
   - src/app.rs: EditorApp (state, run_command, menu bar, status bar) - moved from main.rs.
   - src/canvas.rs: the central panel drawing and the document-to-texture refresh.
   - src/menu.rs: unchanged except visibility.
   - src/main.rs (<40 lines): reads the arguments by hand (no new parser crate); with --snapshot it calls snapshot::write_png, else opens the window as today.
2. Add egui_kittest 0.37 (MIT/Apache, from the egui authors, ADR-008) as a normal dependency of editor-app with features eframe + wgpu + snapshot, plus one [workspace.dependencies] line. It has to be a normal dependency, not test-only, because the shipped binary's --snapshot flag uses it. Cost: the app now also compiles wgpu (longer first build, bigger binary). Logged in docs/provenance-log.md.
3. src/snapshot.rs:
   - gpu_available(): asks wgpu for any adapter (a real GPU or a software one like lavapipe/llvmpipe). Needed because egui_kittest itself crashes when there is no adapter; we check first so our code never crashes (rule 1).
   - harness(width, height): runs EditorApp inside egui_kittest's headless harness at that size.
   - write_png(path, width, height) -> Result: renders a few frames (so layout settles) and saves the PNG. No window is opened.
   - CLI behavior: `editor --snapshot out.png [--size 1200x800]` (default 1200x800). Exit 0 on success. On no GPU adapter or bad arguments: a plain-English message on stderr and exit code 2, never a panic.
   - What the image shows: the window's content (menu bar, canvas, status bar). OS title bar/borders are not part of it.
4. tests/snapshot.rs, stored PNGs in tests/snapshots/:
   - startup: the main window at 1200x800.
   - file_menu_open: clicks "File" through the accessibility tree, then snapshots.
   - Tolerance (documented at the top of the test file and in the recipe): egui_kittest's per-pixel color threshold (default 0.6 on its 0-100 scale) and up to 0.1% of pixels may differ, to absorb small GPU/driver anti-aliasing differences between machines.
   - No adapter: the test prints "SKIP: no GPU adapter, snapshot test not run" and returns (passes), so CI stays green (rule 9; enabling software rendering on CI is a separate owner-approved change).
   - Updating stored PNGs: UPDATE_SNAPSHOTS=1 cargo test -p editor-app. Rule 5 applies: only in a task whose purpose changes the UI, and the PR must show old vs new PNG.
5. docs/recipes/check-ui.md (10 lines or fewer): take a snapshot, open it with the Read tool to look at it, run the snapshot tests, update stored PNGs. AGENTS.md rule 6 becomes: "Look at UI changes: take a snapshot (docs/recipes/check-ui.md), look at it, attach it to the PR."
6. docs/gaps.md G1 entry: mark closed with evidence and date; note CI still skips (needs GPU/software renderer on CI, owner's call).
7. cargo xtask ci until green; take a snapshot and look at it myself; PR with template.

Decisions for the owner to confirm:
- (a) egui_kittest + wgpu ship inside the app binary (needed for --snapshot). Alternative: put --snapshot behind a cargo feature so normal builds skip wgpu - smaller binary, but agents must remember the feature flag. I recommend shipping it always (simpler).
- (b) Exit code 2 when no GPU adapter exists for --snapshot.
- (c) The tolerance in step 4.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Plan approved by owner 2026-10-31 (choices a, b, c as proposed).

Implementation notes (opus-1):
- egui_kittest crashes when no adapter exists, so snapshot.rs builds the wgpu RenderState itself (egui-wgpu + pollster, both direct deps) and turns 'no adapter' into SnapshotError::NoGpu.
- The status-bar live count ("N of M catalog features live") is masked in snapshot tests, else every new command would break them. Masked area: bottom-right 400x26 px.
- Tolerance: per-pixel threshold 0.6, up to 0.1% of pixels (960 at 1200x800) may differ, as approved. egui_kittest's own docs advise a far smaller pixel budget (about 10); 0.1% could hide a small change like one icon. Worth tightening once we see cross-machine noise.
- The File-menu snapshot includes the mouse pointer (drawn by the harness) and will legitimately change when File commands go live (TASK-19).
- Verified: tests pass on this machine (Vulkan); replacing startup.png with the wrong image fails the test; WGPU_BACKEND=metal (no adapter on Linux) makes tests print SKIP and pass, and the CLI exit with code 2.
- No catalog changes (no command added), so coverage/floor unchanged.

Cross-family review (Gemini via agy, 2026-10-31): fixed (1) flag without a value (--snapshot / --size) now exits 2 instead of opening the window or silently using defaults; (2) harness()/write_png() reject zero or oversized sizes themselves; (3) snapshot tests skip only on NoGpu, other errors fail. Dropped as not real: 'baseline PNGs missing' (they are committed, just excluded from the reviewed diff); negative canvas scale (sizes 1x1, 50x20, 1x800 render fine in a debug build); mask colour sampling (deterministic, applied identically to stored and new image); live_count per-frame cost (pre-existing, ~600 lookups).

Owner review 2026-10-31: tolerance tightened from 0.1% of pixels (960) to 10 pixels (threshold 0.6 unchanged). Measured 0 differing pixels between Vulkan and OpenGL on Intel Iris Plus. Not measured on Windows/macOS: their main-branch CI runners may have an adapter, so the tests may run there for the first time after merge.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added offscreen UI snapshots (gap G1, ADR-008). The app is now a library (app.rs state+UI, canvas.rs drawing, menu.rs, snapshot.rs) and main.rs is 30 lines. `cargo run -p editor-app -- --snapshot out.png --size 1200x800` writes the window's content to a PNG without opening a window (exit 0; exit 2 with a message on bad size or no GPU adapter). Two snapshot tests (startup, File menu open) compare against stored PNGs in crates/editor-app/tests/snapshots/ with the documented tolerance and print SKIP when no adapter exists. New recipe docs/recipes/check-ui.md; AGENTS.md rule 6 now points to it; gaps.md G1 updated; provenance logged. cargo xtask ci passes. Risks: tolerance may be loose (see notes); snapshot tests do not run on CI until a software renderer is enabled there (owner's call).
<!-- SECTION:FINAL_SUMMARY:END -->
