---
id: TASK-3
title: 'editor-cli: drive the engine from the command line (gap G2, part 2)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-0
dependencies:
  - TASK-2
references:
  - docs/decisions/ADR-007-agent-control.md
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents need hands: a way to run commands and inspect the result without a window, usable in CI. Follows ADR-007. File I/O comes later (a separate task adds open/render options).

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-cli/` (new crate)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `editor-cli commands` prints the command list JSON from `state::commands`
- [ ] #2 `editor-cli run --new 64x64 --exec '<id> <json params>' [--exec ...] --state` runs the commands in order and prints the state JSON
- [ ] #3 `editor-cli run --script steps.jsonl` reads one `{"id":..,"params":..}` per line; `undo` and `redo` are accepted as steps
- [ ] #4 Exit code 0 on success, 1 when a command fails (error text on stderr, which step failed), 2 on bad usage
- [ ] #5 At least 5 integration tests run the binary, including malformed JSON and an unknown command id (no panic)
- [ ] #6 The crate has a line in `xtask/layers.toml` at layer 6 and depends on no GUI crate
- [ ] #7 `cargo xtask ci` passes
<!-- AC:END -->
