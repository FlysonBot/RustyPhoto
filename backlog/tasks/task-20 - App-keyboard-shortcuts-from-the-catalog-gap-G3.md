---
id: TASK-20
title: 'App: keyboard shortcuts from the catalog (gap G3)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-1
references:
  - docs/decisions/ADR-008-ui-snapshots.md
priority: medium
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Catalog entries carry shortcuts but pressing them does nothing. People who know professional editors expect them.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-app/src/shortcuts.rs` (new)
- `crates/editor-app/src/app.rs` (one call)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every catalog shortcut of a live, enabled entry runs its command; `Ctrl` means Cmd on macOS
- [ ] #2 Ctrl+Z undoes, Ctrl+Shift+Z redoes (app shell)
- [ ] #3 Shortcuts are ignored while a text field has focus
- [ ] #4 A parser test covers every shortcut string in the catalog (all parse); an egui_kittest test presses Ctrl+I and checks the invert ran
- [ ] #5 `cargo xtask ci` passes
<!-- AC:END -->
