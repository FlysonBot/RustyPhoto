---
id: TASK-35
title: 'Workflow: add an In Review status and mark Done on the PR branch before merge'
status: In Review
assignee:
  - '@opus-1'
created_date: '2026-11-01T07:32:00Z'
updated_date: '2026-11-01T07:36:00Z'
labels:
  - track:design
  - agent:opus
dependencies: []
priority: high
ordinal: 32000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Statuses are To Do, In Progress, Blocked and Done. A task whose PR is open and waiting for the owner looks the same as one still being worked on, so other agents and the integrator can't tell it is finished. Marking a task Done also has no documented home: docs/prompts/05-merge.md says to mark merged tasks Done, which means a direct commit to main. In the TASK-1/29-32 session it was done by an extra commit on the PR branch before squash-merging, so main got the status with the feature; that habit is not written down. Owner asked for this 2026-11-01.

**Files allowed:** `backlog/config.yml` (statuses, via `backlog config set`), `docs/prompts/03-implement.md` (step 6), `docs/prompts/05-merge.md` (steps 1-3), `AGENTS.md` (section 5, last row)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Statuses are To Do, In Progress, Blocked, In Review, Done (Done stays last)
- [x] #2 03-implement.md: after opening the PR, set the task In Review, commit and push
- [x] #3 05-merge.md: set the task Done on the PR branch, commit and push, then squash-merge; no direct commit to main for status
- [x] #4 Verified a task can be set In Review and the board shows the column
- [x] #5 cargo xtask ci passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. backlog/config.yml: add In Review before Done (the CLI refuses 'config set statuses' and says to edit the file).
2. 03-implement.md step 6: set the task In Review before pushing and opening the PR; back to In Progress if review sends it back.
3. 05-merge.md: ready PRs have In Review tasks; set Done on the PR branch (commit, push) right before the squash-merge; back to In Review if the merge fails; old step 3 removed and later steps renumbered.
4. AGENTS.md section 5 last row: set it In Review. Owner asked for this 2026-11-01 (plan approved by that request).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Verified 2026-11-01 (backlog 1.55.3): 'backlog config set statuses' is refused ('statuses cannot be set directly... Edit the list in the project config file'), so config.yml was edited. After that: backlog config get statuses = To Do, In Progress, Blocked, In Review, Done; backlog task edit TASK-35 -s 'In Review' worked; board export header = | To Do | In Progress | Blocked | In Review | Done | with TASK-35 in it; task list -s Done still lists the Done tasks. 05-merge.md steps 4-7 were renumbered 3-6 (outside 'steps 1-3'; no doc refers to those numbers). cargo xtask ci: All checks passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added an In Review status (before Done). The implement prompt sets a task In Review when its PR opens, and back to In Progress if review sends it back. The merge prompt sets the task Done on the PR branch right before the squash-merge, so the status reaches main with the feature and nobody commits status changes to main directly. AGENTS.md section 5 mentions In Review. Verified the CLI and board accept the new status; gate green.
<!-- SECTION:FINAL_SUMMARY:END -->
