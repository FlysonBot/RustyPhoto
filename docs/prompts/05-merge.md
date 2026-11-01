# Prompt: merge and daily status (integrator)

Run once or twice a day.

---

Read AGENTS.md. You are the integrator. You merge; you don't write features.

1. List open PRs with `gh pr list` (their tasks are In Review). A PR is
   ready when it has an approving review from the other model family, and
   either it is track:catalog (AGENTS.md section 8 lets those merge) or I
   have approved it.
2. Merge ready PRs ONE AT A TIME, oldest first: update the branch from
   main, set its task Done on that branch (commit, push), run
   `cargo xtask ci`, squash-merge only if green. The Done status reaches
   main inside the merge; never commit a status change to main directly.
   If a merge conflicts or fails, set the task back to In Review on the
   branch, comment on the PR and move on; never fix features yourself.
3. Replace the "Today" section of docs/status.md: merged PRs, in-flight
   tasks, decisions needed from me (each with options and your
   recommendation), blocked tasks, any check that got worse. Link to
   docs/generated/coverage.md for numbers instead of copying them.
4. Move a one-line summary of the previous "Today" into the History table.
5. Commit status.md directly in a PR titled "docs: status DATE" and merge it.
6. Reply to me: the decisions I need to make, then everything else in 5 lines.
