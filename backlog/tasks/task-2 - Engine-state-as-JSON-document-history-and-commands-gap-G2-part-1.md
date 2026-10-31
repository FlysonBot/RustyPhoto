---
id: TASK-2
title: 'Engine state as JSON: document, history and commands (gap G2, part 1)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-0
dependencies: []
references:
  - docs/decisions/ADR-007-agent-control.md
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents and tests need to read what the editor holds as structured data instead of guessing. This JSON format becomes the contract the CLI (and later MCP) returns. Follows ADR-007.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-engine/src/state.rs` (new)
- `crates/editor-engine/src/lib.rs` (one `pub mod` line and re-export)
- `docs/specs/state-json.md` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `editor_engine::state::describe(&Engine)` returns JSON with: width, height, layers (index, name, visible, opacity), active layer, can_undo, can_redo
- [ ] #2 `editor_engine::state::commands(&Engine)` returns every command with id, label, params text and enabled flag
- [ ] #3 `docs/specs/state-json.md` documents every field with its type, and has one full example
- [ ] #4 A test checks the exact JSON for a 2-layer 4x3 document, so any format change is visible in review
- [ ] #5 Key order is stable (two calls give byte-identical output)
- [ ] #6 `cargo xtask ci` passes
<!-- AC:END -->
