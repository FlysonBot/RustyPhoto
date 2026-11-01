---
id: TASK-33
title: >-
  Docs: create the worktree first, then run task-changing backlog commands
  inside it
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T07:27:00Z'
updated_date: '2026-11-01T07:33:00Z'
labels:
  - track:design
  - agent:opus
dependencies: []
priority: high
ordinal: 32000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The backlog CLI writes task files into the checkout it runs in. docs/prompts/03-implement.md has the agent claim its task (step 1) before creating its worktree (step 2), so the claim, and any task the agent creates, land as uncommitted edits in the main checkout instead of on the agent's branch. Observed four times in the TASK-1/29-32 session (files had to be moved by hand). With parallel agents, main fills with edits from several agents that collide. Owner asked for this 2026-11-01.

**Files allowed:** `docs/prompts/03-implement.md` (steps 1-2), `AGENTS.md` (section 6, worktree bullet)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 03-implement.md creates the worktree before any backlog command that changes a task
- [x] #2 AGENTS.md section 6 says read-only backlog commands work anywhere, but commands that change a task run inside your own worktree, and why
- [x] #3 Verified in a worktree that a backlog edit there changes only the worktree's files, not main's
- [x] #4 cargo xtask ci passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. 03-implement.md: swap steps 1 and 2 so the worktree comes first; say every later step runs inside it.
2. AGENTS.md section 6 worktree bullet: create it before changing any task; task-changing backlog commands write into the checkout they run in; reading works anywhere.
3. Verify by creating a task inside a worktree and checking main stays clean. Owner asked for this 2026-11-01 (plan approved by that request).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Verified 2026-11-01 (backlog 1.55.3): creating TASK-33 inside this worktree wrote only the worktree's backlog/tasks; git status in the main checkout stayed clean. cargo xtask ci: All checks passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
03-implement.md now creates the worktree first (step 1) and claims the task second, with every later step run inside the worktree. AGENTS.md section 6 explains why: backlog commands that change a task write into the checkout they run in, so they must run in the agent's own worktree, never the main checkout; reading works anywhere. Verified main stays clean when a task is created in a worktree. Gate green.
<!-- SECTION:FINAL_SUMMARY:END -->
