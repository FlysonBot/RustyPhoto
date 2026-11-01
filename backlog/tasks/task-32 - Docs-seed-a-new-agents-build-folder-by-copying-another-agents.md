---
id: TASK-32
title: 'Docs: seed a new agent''s build folder by copying another agent''s'
status: Done
assignee:
  - '@opus-1'
created_date: '2026-11-01T04:14:00Z'
updated_date: '2026-11-01T04:20:00Z'
labels:
  - track:design
  - agent:opus
dependencies: []
priority: medium
ordinal: 31000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
With one build folder per agent (TASK-30), each new agent still compiles all libraries once (~200-250 s). Copying an existing agent's folder works if file timestamps are kept: measured 2026-10-31, plain cp -r = full rebuild (201 s), cp -a = 20 s (only the 8 workspace crates rebuilt); on btrfs with --reflink=auto the 4.3 GB copy took 1 s and uses no extra disk. Add this as a first-use step. Owner approved 2026-10-31.

**Files allowed:** `AGENTS.md` (section 6 build-folder bullet), `docs/prompts/03-implement.md` (step 2)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 AGENTS.md section 6 tells a new agent to copy an existing agent's folder, keeping timestamps, before its first build
- [x] #2 The step warns that copying without timestamps rebuilds everything and to copy only while the source agent is not building
- [x] #3 Measured speedup recorded in the task
- [x] #4 cargo xtask ci passes
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measured 2026-10-31 on btrfs: cp -r copy then gate = 201 s (everything rebuilt: timestamps reset); cp -a --reflink=auto copy = 1 s for 4.3 GB, no extra disk; gate in a brand-new worktree using the seeded folder = 19-20 s (only the workspace crates rebuilt). Cold gate for comparison: 246 s.

Owner approved merge 2026-10-31.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
AGENTS.md section 6 and 03-implement.md now tell a new agent to seed its build folder by copying another agent's with timestamps kept (cp -a --reflink=auto; robocopy on Windows) before its first build. Measured: 19-20 s instead of ~200-250 s; a plain cp -r rebuilds everything, which the doc warns about.
<!-- SECTION:FINAL_SUMMARY:END -->
