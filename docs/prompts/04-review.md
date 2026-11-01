# Prompt: review one PR

Use the other model family than the PR's author (Claude reviews Gemini's
work, Gemini reviews Claude's). Replace PR-NUMBER.

---

Read AGENTS.md. You are the reviewer for PR PR-NUMBER. You did not write it.
Your job is to find what is wrong, not to confirm it is right.

1. Read the PR description, its Backlog.md task and the full diff
   (`gh pr view PR-NUMBER`, `gh pr diff PR-NUMBER`).
2. Check it out in a fresh worktree and run `cargo xtask ci`.
3. Hunt for bugs first. Be skeptical; assume there is at least one. Look for:
   - inputs that crash, hang or give a wrong result (empty, zero, huge,
     missing, out of order, a flag with no value)
   - errors that are swallowed or treated as success (a test that passes
     when it should fail, a skip that hides a real failure)
   - docs, comments or the PR description saying something untrue
   - anything slow or wasteful at the size the app will really run at
   For every finding give: `file:line`, a concrete failure (this input or
   state → this wrong result), and how you confirmed it (ran it, wrote a
   test, or traced the code). If you could not confirm it, say so, or drop it.
   Mark each finding **blocking** (must fix before merge) or **minor**.
4. Then the mechanical checks. Report each as pass, fail or n/a, with a
   one-line reason:
   - every acceptance criterion is actually met (verify, don't trust ticks)
   - only files in the task's "Files allowed" changed
   - no golden rule broken (AGENTS.md section 3), no check weakened
   - tests test real behavior with exact values (for commands: including
     undo and bad params)
   - no scope added or dropped versus the task
   - docs updated where the change affects them; coverage regenerated if a
     command was added
5. Post your review as a comment (`gh pr review PR-NUMBER --comment`; every
   agent shares one GitHub account, which can't approve its own PR). Start
   with a verdict: APPROVE or CHANGES REQUESTED, then a numbered list of
   blocking fixes, then minor ones.
6. Reply to me in plain English, 5 lines max: verdict, the main reason,
   and anything only I can judge (e.g. how something looks).
Do not fix the code yourself.
