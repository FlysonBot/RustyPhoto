---
id: TASK-1
title: 'Offscreen UI snapshot: render the app to PNG (gap G1)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
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
- [ ] #1 `cargo run -p editor-app -- --snapshot out.png --size 1200x800` writes a 1200x800 PNG of the main window and exits with code 0 without opening a window
- [ ] #2 The app's state and UI live in `crates/editor-app/src/app.rs` (library); `main.rs` is under 40 lines; canvas drawing lives in `canvas.rs`
- [ ] #3 At least 2 snapshot tests (startup window; File menu open) compare against stored PNGs with a documented tolerance
- [ ] #4 Snapshot tests skip with a printed notice (not fail) when no GPU adapter exists, so `cargo xtask ci` stays green on CI
- [ ] #5 `docs/recipes/check-ui.md` explains in 10 lines or fewer how an agent takes and looks at a snapshot; AGENTS.md rule 6 points to it
- [ ] #6 `cargo xtask ci` passes
<!-- AC:END -->
