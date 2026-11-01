---
id: TASK-17
title: History with memory budget, labels and timestamps (ADR-004)
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
updated_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies:
  - TASK-10
  - TASK-11
references:
  - docs/decisions/ADR-004-history-storage.md
priority: high
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The 50-step limit throws history away and full copies waste memory. The vision asks for unlimited steps within a memory budget (default 2 GB) with a notice when old steps drop, and a timestamp per step for the History panel.

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-engine/src/history.rs` (new)
- `crates/editor-engine/src/lib.rs`
- `crates/editor-engine/src/state.rs`
- `docs/specs/state-json.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each history entry has the command label and a timestamp; state JSON lists them
- [ ] #2 Default budget 2 GiB, settable (minimum 64 MiB); `UNDO_LIMIT` is removed
- [ ] #3 With a 64 MiB budget and 1000 one-pixel edits on a 6000x4000 image, all 1000 steps are kept
- [ ] #4 When over budget, the oldest steps are dropped until it fits, and `execute` reports how many (test checks the number)
- [ ] #5 Undo and redo of a 6000x4000, 20-layer document take under 10 ms each in a test (a pointer swap)
- [ ] #6 Replace-effect commands clear history; `cargo xtask ci` passes
<!-- AC:END -->
