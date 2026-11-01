---
id: TASK-34
title: >-
  Docs: make a claim visible to other agents (check the board, commit and push
  the claim)
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T07:32:00Z'
updated_date: '2026-11-01T07:34:00Z'
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
- [x] #1 AGENTS.md section 4 says to find unclaimed work with git fetch then backlog board, not backlog task list, and why
- [x] #2 Claiming = edit the task in your worktree, then commit and push that change before any other work
- [x] #3 Verified that a claim committed on one branch shows on the board from another worktree
- [x] #4 cargo xtask ci passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. AGENTS.md section 4: look for work with git fetch + backlog board (sees other branches), not task list (own checkout only).
2. Claiming = in your worktree, assign + In Progress, then commit and push before other work; later of two claims gives way. New tasks from gaps: commit and push the same way.
3. 03-implement.md step 2: commit and push the claim right away.
4. Dropped from the original idea: 'only one agent creates tasks'. Measured that task numbers do not collide, so the rule is not needed. Owner asked for this 2026-11-01 (plan approved by that request).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measured 2026-11-01, backlog 1.55.3, worktrees t33/t34/t35 of one repo:
- From t34, backlog task view TASK-33 (committed only on t33): 'Task TASK-33 not found. Task lookups read only the local working copy'. --dep TASK-33 refused the same way.
- backlog board export from t35 listed TASK-33 and TASK-34 In Progress @opus-1; TASK-34 was committed on t34 and never pushed.
- An uncommitted claim (TASK-10 set In Progress @probe in t35) did not show on the board from t34. So the claim must be committed.
- task create numbering: skipped a number used by an uncommitted task in another worktree, by a local branch, and by a pushed branch made in a separate clone (after git fetch). No collisions seen.
- Side finding: board export with an absolute path writes to that path relative to the current folder (e.g. ./tmp/...).
cargo xtask ci: All checks passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
AGENTS.md section 4 now has agents look for work with git fetch + backlog board, which includes other agents' branches, instead of backlog task list, which only reads the agent's own checkout. A claim (or a new task) is committed and pushed right away; uncommitted claims are invisible to other agents (measured). 03-implement.md step 2 says the same. The idea of a single task creator was dropped: measured that task numbers don't collide across worktrees, branches or a fetched remote branch. Gate green.
<!-- SECTION:FINAL_SUMMARY:END -->
