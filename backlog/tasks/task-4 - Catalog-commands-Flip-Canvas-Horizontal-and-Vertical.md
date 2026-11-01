---
id: TASK-4
title: 'Catalog commands: Flip Canvas Horizontal and Vertical'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:catalog
  - agent:sonnet
milestone: m-0
dependencies: []
references:
  - docs/recipes/add-a-command.md
priority: high
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The M0 gate needs one catalog command built, reviewed and merged by agents, proving the loop works end to end. Flipping is simple and fully specified: mirror every layer of the canvas.

**Track:** catalog · **Agent:** sonnet

**Files allowed:**
- `crates/editor-engine/src/cmd/image_flip.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `image.flip.horizontal` mirrors every layer left-right; `image.flip.vertical` mirrors top-bottom
- [ ] #2 Tests on a 3x2 document with 2 layers check exact pixel positions for both commands
- [ ] #3 Flipping twice restores the exact document; undo restores the exact document
- [ ] #4 Coverage regenerated; `catalog/floor.txt` raised to the new live count (4)
- [ ] #5 Follows `docs/recipes/add-a-command.md` exactly; `cargo xtask ci` passes
<!-- AC:END -->
