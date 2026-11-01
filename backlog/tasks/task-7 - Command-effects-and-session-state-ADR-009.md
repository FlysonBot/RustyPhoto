---
id: TASK-7
title: Command effects and session state (ADR-009)
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies:
  - TASK-2
  - TASK-4
  - TASK-5
references:
  - docs/decisions/ADR-009-command-effects.md
priority: high
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Zoom, colors, tools, export, open and new are catalog features but don't edit the image; today every command must be an undoable document edit. Without this they can't be live or reachable by agents.

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-engine/src/cmd/mod.rs` (structural change)
- `crates/editor-engine/src/cmd/*.rs` (existing commands: add the effect)
- `crates/editor-engine/src/lib.rs`
- `crates/editor-engine/src/session.rs` (new)
- `crates/editor-engine/src/state.rs`
- `crates/editor-engine/tests/panic_hunt.rs`
- `docs/recipes/add-a-command.md`
- `docs/specs/state-json.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `CommandSpec` has an `effect`: Edit, Session, ReadOnly or Replace, with the meaning in ADR-009
- [ ] #2 Only Edit commands add a history step; a test runs one command of each effect and checks the undo count (1, 0, 0, 0) and that Replace clears history
- [ ] #3 The engine holds a `Session`: view zoom (default 1.0) and pan, foreground color (black) and background color (white), current tool id
- [ ] #4 State JSON includes the session; the spec documents it
- [ ] #5 All existing commands still pass their tests; the fuzz test covers every effect
- [ ] #6 Recipe updated to say how to pick an effect; `cargo xtask ci` passes
<!-- AC:END -->
