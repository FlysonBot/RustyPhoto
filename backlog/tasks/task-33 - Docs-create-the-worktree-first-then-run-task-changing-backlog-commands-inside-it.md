---
id: TASK-33
title: >-
  Docs: create the worktree first, then run task-changing backlog commands
  inside it
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T07:27:00Z'
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
- [ ] #1 03-implement.md creates the worktree before any backlog command that changes a task
- [ ] #2 AGENTS.md section 6 says read-only backlog commands work anywhere, but commands that change a task run inside your own worktree, and why
- [ ] #3 Verified in a worktree that a backlog edit there changes only the worktree's files, not main's
- [ ] #4 cargo xtask ci passes
<!-- AC:END -->
