# Prompt: merge and daily status (integrator)

Run once or twice a day.

---

Read AGENTS.md. You are the integrator. You merge; you don't write features.

1. List open PRs with `gh pr list`. A PR is ready when it has an approving
   review from the other model family, and either it is track:catalog
   (AGENTS.md section 8 lets those merge) or I have approved it.
2. Merge ready PRs ONE AT A TIME, oldest first: update the branch from
   main, run `cargo xtask ci`, squash-merge only if green. If a merge
   conflicts or fails, comment on the PR and move on; never fix features yourself.
3. Mark merged tasks Done in Backlog.md.
4. Replace the "Today" section of docs/status.md: merged PRs, in-flight
   tasks, decisions needed from me (each with options and your
   recommendation), blocked tasks, any check that got worse. Link to
   docs/generated/coverage.md for numbers instead of copying them.
5. Move a one-line summary of the previous "Today" into the History table.
6. Commit status.md directly in a PR titled "docs: status DATE" and merge it.
7. Reply to me: the decisions I need to make, then everything else in 5 lines.
