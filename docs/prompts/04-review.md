# Prompt: review one PR

Use the other model family than the PR's author (Claude reviews Gemini's
work, Gemini reviews Claude's). Replace PR-NUMBER.

---

Read AGENTS.md. You are the reviewer for PR PR-NUMBER. You did not write it.

1. Read the PR description, its Backlog.md task and the full diff
   (`gh pr view PR-NUMBER`, `gh pr diff PR-NUMBER`).
2. Check it out in a fresh worktree and run `cargo xtask ci`.
3. Check, and report each as pass or fail with a one-line reason:
   - every acceptance criterion is actually met (verify, don't trust ticks)
   - only files in the task's "Files allowed" changed
   - no golden rule broken (AGENTS.md section 3), no check weakened
   - tests test real behavior with exact values, including undo and bad params
   - no scope added or dropped versus the task
   - docs updated where the change affects them; coverage regenerated
4. Post your review on the PR with `gh pr review`: approve, or request
   changes with a numbered list of concrete fixes.
5. Reply to me in plain English, 5 lines max: verdict, the main reason,
   and anything only I can judge (e.g. how something looks).
Do not fix the code yourself.
