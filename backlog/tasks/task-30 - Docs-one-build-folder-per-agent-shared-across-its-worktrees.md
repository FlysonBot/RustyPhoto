---
id: TASK-30
title: 'Docs: one build folder per agent, shared across its worktrees'
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T03:16:00Z'
updated_date: '2026-11-01T03:24:00Z'
labels:
  - track:design
  - agent:opus
dependencies: []
priority: medium
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
AGENTS.md section 6 says CARGO_TARGET_DIR=target/agent-<name>. That path is relative, so it lands inside each new worktree and every branch recompiles all outside libraries (wgpu etc.) from scratch. Point it at one folder per agent in the main checkout instead, so an agent's branches reuse its compiled libraries. Folders stay per agent, so parallel agents still never share a lock or overwrite each other. Owner approved 2026-10-31.

**Files allowed:** `AGENTS.md` (section 6 build-folder line), `docs/prompts/03-implement.md` (step 2)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 AGENTS.md section 6 and 03-implement.md give a per-agent build folder outside the worktree that works from any worktree
- [x] #2 A second worktree of the same agent reuses compiled libraries (measured)
- [x] #3 cargo xtask ci passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Path: "$(git rev-parse --path-format=absolute --git-common-dir)/../target/agent-<name>" = <main checkout>/target/agent-<name>, already gitignored. 2. Edit both docs. 3. Measure: build in a fresh worktree with the shared folder, compare with a cold build. 4. PR.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measured 2026-10-31 with the shared folder: cold full gate (cargo xtask ci) 246 s; same gate from another worktree (t29) reusing the folder 2 s, with zero crates recompiled when the code is identical. Gemini review: added PowerShell form and kept NAME explicit in 03-implement.md. Rejected: fingerprint invalidation across worktrees (measured: none); lock contention (one agent runs one task at a time, different agents use different folders); bare-repo / git<2.31 (not our setup).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
AGENTS.md section 6 and 03-implement.md now give each agent one build folder, <main checkout>/target/agent-<name>, shared by all its worktrees, instead of a folder inside each worktree. New branches no longer recompile the outside libraries: full gate 246 s cold vs 2 s from a second worktree. Folders stay per agent, so parallel agents never share a lock.
<!-- SECTION:FINAL_SUMMARY:END -->
