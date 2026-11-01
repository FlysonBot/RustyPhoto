---
id: TASK-31
title: 'Docs: review prompt puts bug hunting first, with evidence and severity'
status: In Progress
assignee:
  - '@opus-1'
created_date: '2026-11-01T04:14:00Z'
updated_date: '2026-11-01T04:16:00Z'
labels:
  - track:design
  - agent:opus
dependencies: []
priority: medium
ordinal: 30000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
docs/prompts/04-review.md was a compliance checklist with no bug hunt. In the Gemini reviews of PRs #4-#6 every real finding came from a skeptical bug hunt, and the false positives came from unverified claims. Rewrite: bug hunt first (with file:line, a concrete failure, and how it was confirmed; blocking vs minor), then the mechanical checks with n/a allowed, and post as a review comment because all agents share one GitHub account that cannot approve its own PR. Owner approved the draft 2026-10-31.

**Files allowed:** `docs/prompts/04-review.md`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Bug hunting is step 3, before the mechanical checks
- [x] #2 Each finding requires file:line, a concrete failure, how it was confirmed, and blocking/minor
- [x] #3 Mechanical checks allow n/a for items that only apply to commands
- [x] #4 Review is posted with gh pr review --comment and starts with a verdict
- [x] #5 cargo xtask ci passes
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rewrote docs/prompts/04-review.md: a skeptical bug hunt comes first (each finding needs file:line, a concrete failure, how it was confirmed, and blocking/minor), then the existing mechanical checks with n/a allowed; the review is posted as a comment with a verdict because the shared GitHub account can't approve its own PRs. Draft approved by owner before the PR.
<!-- SECTION:FINAL_SUMMARY:END -->
