---
id: TASK-21
title: 'App: parameter dialogs generated from ParamSpec (gap G3)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
updated_date: '2026-11-01T01:04:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-8
  - TASK-20
references:
  - docs/decisions/ADR-008-ui-snapshots.md
priority: high
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Commands with settings run with defaults only. A dialog generated from each command's ParamSpec makes every such command usable without UI work per command. The owner approves the dialog look once.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-app/src/dialog.rs` (new)
- `crates/editor-app/src/app.rs` (one call)
- `crates/editor-app/src/lib.rs` (one `mod` line)
- `crates/editor-app/Cargo.toml` (rfd)
- `crates/editor-app/tests/dialog.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Clicking a command with parameters opens a modal dialog with one widget per kind (slider+number, checkbox, dropdown, text, color picker, file picker via rfd, layer dropdown)
- [ ] #2 OK runs the command with the entered values; Cancel runs nothing; an engine error is shown in the dialog and the dialog stays open
- [ ] #3 Commands without parameters run at once, as today
- [ ] #4 Snapshot of one dialog attached to the PR and looked at; an egui_kittest test fills a field and presses OK
- [ ] #5 `cargo xtask ci` passes
<!-- AC:END -->
