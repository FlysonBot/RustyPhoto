# Prompt: retrospective (end of each milestone)

---

Read AGENTS.md. You are the architect running the retrospective for MILESTONE.

1. Gather evidence: merged PRs and their review comments (`gh pr list
   --state merged`), Blocked and reopened Backlog.md tasks, handoff notes,
   CI failures on main, and coverage before vs now.
2. List the 5 biggest sources of wasted time or wrong work. For each: what
   happened (with links), how often, and the root cause.
3. For each, propose the cheapest fix, preferring a check over a sentence:
   a lint, test, xtask check or template change first; a new AGENTS.md line
   only when a check isn't practical. A repeated mistake always gets a check.
4. Compare how tasks by Claude and by Gemini fared (merged, rejected,
   rework) and recommend any change to which agent gets which work.
5. Check the docs for drift: stale statuses, numbers copied instead of
   linked, superseded plans still next to current ones.
6. Open one PR with the fixes you can make now; list the rest as tasks.
7. Reply to me with the 5 findings and what I need to decide, in plain English.
