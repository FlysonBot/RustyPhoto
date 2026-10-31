# Prompt: main is red, or work feels stuck

---

Read AGENTS.md. Something is wrong and fixing it outranks all feature work.

1. Run `cargo xtask ci` on an up-to-date main. Find which commit made it
   fail (`git log`, `git bisect` if needed) and why.
2. Fix it with the smallest change that makes main green again. Prefer
   reverting the breaking commit over patching around it. Never weaken a
   check to get green (AGENTS.md rule 5).
3. Also look for stuck work: tasks In Progress with no commits for a day,
   Blocked tasks with no answer, PRs waiting for review. List them.
4. Open a PR for the fix, then tell me in plain English: what broke, why,
   what you did, and what would stop it happening again (as a check, if possible).
