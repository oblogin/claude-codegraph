---
name: self-review
description: Mandatory self-review checklist for anyone who wrote code, tests or fixtures before reporting the work as done. Use at the end of every implementation or test task, and whenever asked to "проведи саморевью" / review your own diff.
---

# Self-review before reporting

You are about to say "done". First review your own work as if a stranger wrote it.

## Steps

1. **See everything you changed:** `git status`, `git diff` (and read new untracked files in full). Don't review from memory.
2. **Run the project's checks** (build, formatter, linter with warnings as errors, full test suite — the exact commands are in CLAUDE.md or the task's plan). All green, or explain precisely what fails and why.
3. **Go through the checklist** below; fix what you find, then re-run the checks.
4. **Write the result into your report** in a «Саморевью» section: what you checked, what you found and fixed, what you consciously left and why.

## Checklist

**Matches the task**
- Every acceptance criterion is met and covered by a test (name the test).
- Follows the plan and the ADRs it references; any deviation is justified in the report.
- Nothing outside the task's scope was changed (no drive-by refactors, no formatting of unrelated files).

**Correctness**
- Edge cases: empty input, missing files, invalid data, very large input, Windows paths and CRLF, non-ASCII names.
- Errors are propagated or reported, never swallowed silently; no `unwrap()`/`expect()`/bare `except` on data from outside.
- No hidden global state, races on shared files or databases, or order-dependent behaviour.

**Tests**
- Tests fail without your change (at least think it through for each).
- No tests weakened, skipped or deleted to get green.
- Fixtures and expected results are exact, not "whatever the code produced".

**Hygiene**
- No debug output, commented-out code, TODOs without a task, dead code.
- Names and comments match what the code does now.
- Borrowed code is recorded where the project requires it (e.g. `THIRD_PARTY.md`).
- No secrets, absolute personal paths or machine-specific values committed.

If something can't be fixed within the task — say so plainly in the report instead of hiding it.
