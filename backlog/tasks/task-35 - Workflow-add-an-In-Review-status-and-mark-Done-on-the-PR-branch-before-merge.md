---
id: TASK-35
title: 'Workflow: add an In Review status and mark Done on the PR branch before merge'
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
Statuses are To Do, In Progress, Blocked and Done. A task whose PR is open and waiting for the owner looks the same as one still being worked on, so other agents and the integrator can't tell it is finished. Marking a task Done also has no documented home: docs/prompts/05-merge.md says to mark merged tasks Done, which means a direct commit to main. In the TASK-1/29-32 session it was done by an extra commit on the PR branch before squash-merging, so main got the status with the feature; that habit is not written down. Owner asked for this 2026-11-01.

**Files allowed:** `backlog/config.yml` (statuses, via `backlog config set`), `docs/prompts/03-implement.md` (step 6), `docs/prompts/05-merge.md` (steps 1-3), `AGENTS.md` (section 5, last row)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Statuses are To Do, In Progress, Blocked, In Review, Done (Done stays last)
- [ ] #2 03-implement.md: after opening the PR, set the task In Review, commit and push
- [ ] #3 05-merge.md: set the task Done on the PR branch, commit and push, then squash-merge; no direct commit to main for status
- [ ] #4 Verified a task can be set In Review and the board shows the column
- [ ] #5 cargo xtask ci passes
<!-- AC:END -->
