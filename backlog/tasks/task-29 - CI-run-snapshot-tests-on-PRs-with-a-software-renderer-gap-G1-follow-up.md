---
id: TASK-29
title: 'CI: run snapshot tests on PRs with a software renderer (gap G1 follow-up)'
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T03:06:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-0
dependencies: []
references:
  - docs/decisions/ADR-008-ui-snapshots.md
priority: high
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Snapshot tests (TASK-1) skip on PRs because the ubuntu-latest runner has no GPU adapter. Windows and macOS runners (push to main only) do have one and the tests pass there. Install Mesa's software Vulkan driver (lavapipe) on the Linux runner so snapshot tests actually run on every PR. Owner requested this CI change on 2026-10-31 (AGENTS.md rule 9: CI changes go in their own PR with owner approval).

**Track:** design · **Agent:** opus

**Files allowed:**
- `.github/workflows/ci.yml` (owner-approved CI change)
- `crates/editor-app/tests/snapshot.rs` (only if a per-OS tolerance is needed, with measured numbers)
- `docs/recipes/check-ui.md`
- `docs/gaps.md` (G1 entry)
- `AGENTS.md` (rule 6 wording only)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 On PRs the Linux CI job runs both snapshot tests (no SKIP line in the log) and they pass
- [ ] #2 Windows and macOS jobs on main are unchanged
- [ ] #3 Stored PNGs are not regenerated to make CI pass; any tolerance change is measured and documented
- [ ] #4 check-ui.md, gaps.md G1 and AGENTS.md rule 6 no longer say CI skips the tests
- [ ] #5 cargo xtask ci passes locally and on the PR
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a Linux-only step before the gate: sudo apt-get update && sudo apt-get install -y mesa-vulkan-drivers (lavapipe). egui_kittest prefers CPU adapters, so it will pick lavapipe.
2. Push and read the PR log: confirm no SKIP and the tests pass. If lavapipe differs from the stored PNGs by more than 10 pixels, report the measured count to the owner before changing any tolerance.
3. Update check-ui.md, gaps.md G1, AGENTS.md rule 6.
4. PR with template.
<!-- SECTION:PLAN:END -->
