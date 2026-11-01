---
id: TASK-34
title: >-
  Docs: make a claim visible to other agents (check the board, commit and push
  the claim)
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T07:32:00Z'
labels:
  - track:design
  - agent:opus
dependencies: []
priority: high
ordinal: 32000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
AGENTS.md section 4 says to claim a task (assign yourself, set In Progress) before starting, but the claim lives only in the claiming agent's worktree, and the agent picking work looks with `backlog task list`, which reads only its own checkout. Two agents starting together can both take the same To Do task.

Measured 2026-11-01 with backlog 1.55.3: `backlog task list` and `task view` read only the local checkout ("Task lookups read only the local working copy"); so does `--dep`, which refuses a task that exists only on another branch. `backlog board` (and `board export`) show tasks from other branches, including a claim committed on another agent's branch. New task numbers do not collide: `task create` skipped numbers used uncommitted in another worktree on the same machine, on local branches, and on a pushed branch from another clone after `git fetch`. So a separate rule for who creates tasks is not needed.

Builds on TASK-33 (same files, merge after it). Owner asked for this 2026-11-01.

**Files allowed:** `AGENTS.md` (section 4), `docs/prompts/03-implement.md` (step 1)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 AGENTS.md section 4 says to find unclaimed work with git fetch then backlog board, not backlog task list, and why
- [ ] #2 Claiming = edit the task in your worktree, then commit and push that change before any other work
- [ ] #3 Verified that a claim committed on one branch shows on the board from another worktree
- [ ] #4 cargo xtask ci passes
<!-- AC:END -->
