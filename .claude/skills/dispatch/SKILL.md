---
name: dispatch
description: Lead's hand-off step of the Opus-plans / Sonnet-executes pipeline. Use when a planned task is ready to be executed by an agent — choosing which pipeline agent runs it, whether to start a new agent or continue the one that did the previous related task, writing the delegation prompt, then verifying, reviewing and closing the task.
---

# Dispatch a planned task to an executor agent

Precondition: the task has a `### План реализации` (see `plan-task`). If it doesn't, plan first.

## 1. Choose the agent

| Work | Agent | Model |
|---|---|---|
| Implement code by plan — the default | `developer` (middle) | sonnet |
| Implement the hardest code: tricky algorithms, concurrency, performance, cross-cutting core changes, or a task the middle developer failed twice | `senior-developer` | opus |
| Write tests, fixtures, run suites, reproduce bugs | `tester` | sonnet |
| Research by a detailed brief: code, repos, docs, web | `researcher` | sonnet |
| Independent read-only review of a finished diff | `reviewer` | sonnet; pass `model: opus` for core or very complex changes |
| Architecture, ADR drafts, decomposing an epic into planned tasks — when the lead delegates it | `architect` | opus |

**Opus is for planning and the genuinely hard parts only.** Everything that can be done from a precise brief — research, tests, ordinary implementation, routine review — goes to Sonnet. The quality lever is the brief: a Sonnet executor with a plan-mode-quality plan beats an Opus executor with a vague one. Mark senior-level tasks explicitly in the plan (`**Исполнитель:** senior-developer`) and say why.

## 2. New agent or continue an existing one

**Continue** the agent that did an earlier task (send it the new task with `SendMessage` to its id — it keeps its whole context) when **all** hold:
- the new task directly continues that agent's task (same module or files, or `depends_on` it and builds on its code);
- that agent finished successfully and its context is not bloated (it has not done more than ~3 tasks in a row);
- it is the same kind of work (a developer continues development, not review).

**Start a new agent** otherwise — and always for independent review (the reviewer must not have written the code).

Record the chain in the task journal: `исполнитель: developer, агент <id>, цепочка от [[F04-02 …]]`. Agent ids live only within the session; after a restart every chain starts fresh, so the plan must never depend on the agent's memory — only on the task file.

## 3. Parallelism

Run tasks in parallel only if they touch disjoint files. Otherwise run them one after another, or give each `isolation: worktree` and merge afterwards. Never let two executors edit the same file at the same time.

## 4. Delegation prompt (template)

```
Ты — <роль> проекта <имя> (<путь>). Прочитай CLAUDE.md проекта — правила обязательны.
Задача — <ID> «<название>»: <полный путь к файлу задачи>. Выполни раздел «План реализации»
и критерии готовности. <Для продолжения: это продолжение твоей задачи <ID>; всё, что ты знаешь о коде, в силе.>
Отклонение от плана — только если план не работает; тогда сделай минимально необходимое и опиши в отчёте.
Файлы плана (Docs/Tasks) не редактируй и не коммить — это делает лид.
Перед отчётом — саморевью по скиллу self-review.
Отчёт — в формате из твоих инструкций.
```

Run executors in the background unless your next step depends on them. Log «взята в работу; исполнитель …» in the task journal right away.

## 5. When the report arrives

1. **Verify yourself.** Run the project's check commands (build, lint, tests) — don't trust the report alone. Look at the diff (`git diff --stat`, then the risky parts).
2. **Independent review** — mandatory for size `L`, for core modules the project names in CLAUDE.md, and whenever the self-review list is thin: start a fresh `reviewer` on the diff with the task file.
3. **Fixes** go back to the **same developer agent** via `SendMessage` (it has the context), with the reviewer's findings verbatim. Re-verify after.
4. **Close.** Tick criteria, journal entry with the result, what the self-review and review found and fixed; `done` (or `review` if a human must check by hand). Commit with the project's attribution rules.
5. If the report reveals that future tasks must change — replan them (and note why in their journals).

## 6. Keep the lead's context small

Don't read whole transcripts of agents. Their final report is the interface; ask them follow-up questions via `SendMessage` instead of re-doing their work.
