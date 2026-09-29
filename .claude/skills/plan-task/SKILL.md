---
name: plan-task
description: Lead's planning step of the Opus-plans / Sonnet-executes pipeline. Use before handing any implementation, test or research task to an executor agent, when a task in Docs/Tasks (claude-tasks format) has no "План реализации" yet or its plan is stale, or when the user asks to plan or decompose work "как в plan mode".
---

# Plan a task like plan mode, then write the plan into the task

You are the lead (Opus). The executor is usually a **Sonnet** agent. It starts with **no context** except the task file, the project's CLAUDE.md and your delegation prompt, and it is noticeably weaker than you at:
- inferring intent from a vague step;
- keeping a multi-file design consistent without being told how the pieces fit;
- noticing the trap that makes the obvious solution wrong;
- stopping at the edge of the task instead of improvising.

So the plan must carry **your** thinking, not just your conclusions. Write it at the depth of a plan-mode plan you would be willing to execute yourself without re-reading the code: exact files, exact signatures, the key logic spelled out, the tests with their data, and every trap named. A long plan is fine (100–400 lines for an `L` task is normal); a short plan for a hard task is a bug.

If the project keeps its plan in `Docs/Tasks/`, also load `task:format` and follow it: the plan goes into the task file, sections and IDs stay as that format requires.

## 1. Investigate (read, don't guess)

- Read the task: «Коротко», criteria, details, journal, `depends_on`, the ADRs and research reports it links — the relevant sections, not the titles.
- Read the code the task touches: entry points, types, neighbouring modules, existing tests and fixtures, build files. Note `path:line` of everything the executor will need.
- Find what to reuse (existing helpers, patterns, borrowed code the project allows) and pick the **one existing file the new code should look like**.
- If a library or API is involved, check its actual version and signatures in the lockfile or docs — don't plan against an API from memory.
- List every open question and resolve it yourself from code, ADRs or a sensible default. Ask the user only for what only they can provide.

## 2. Decide

- Pick one approach. Why — in a sentence; each rejected alternative — in a line.
- If the task hides a design choice nobody has made, split out a `research` task with an ADR instead of burying the choice in the plan.
- If the task is bigger than size `L` (more than one or two executor sessions), split it before planning.
- Decide who executes: `developer` by default; `senior-developer` only when the core of the task is genuinely hard (non-trivial algorithm, concurrency, performance, cross-cutting core change) — say why.

## 3. Write the plan into the task

Put it at the top of `## Детали` as `### План реализации` (other details stay below). Write in the project's language (Russian for the user's projects). Use this structure and fill **every** part:

```markdown
### План реализации

**Исполнитель:** developer (sonnet) · **Цепочка:** новая | продолжение [[<ID>]] · **Почему этот исполнитель:** …

**Цель одним абзацем.** Что должно работать в конце и как это будет видно.

**Контекст.**
- Что уже есть: `путь:строка` — тип/функция — что делает.
- На что опираемся: ADR-00N §X (перескажи нужное правило в одной-двух строках, не только ссылкой).
- Образец стиля: `путь/к/похожему/файлу` — писать так же.
- Версии библиотек и API, которые используем (из lock-файла), с сигнатурами вызовов.

**Подход.** Выбранное решение и почему; отвергнутое — по строке. Как части связаны между собой (кто кого вызывает, где данные создаются и где читаются).

**Шаги.** Каждый шаг — законченный кусок, после которого сборка зелёная, с проверкой в конце шага.
1. **`путь/к/файлу`** — создать | изменить.
   - Что добавить: типы и функции с полными сигнатурами (код-блоком).
   - Ключевая логика: псевдокод или код для неочевидных мест, порядок операций, условия и ветки, обработка ошибок (какой тип ошибки, что возвращаем).
   - Чего НЕ делать в этом шаге.
   - Проверка шага: команда и ожидаемый результат.
2. …

**Тесты.** Таблица: имя теста · что проверяет · входные данные (фикстура или литерал) · ожидаемый результат · какой критерий задачи закрывает. Отдельно — какие существующие тесты должны остаться зелёными.

**Ловушки.** Конкретные места, где очевидное решение неверно, и как правильно. (Примеры: одноимённые символы; CRLF на Windows; 64-битные ID теряют точность в JS; порядок записи в БД; путь с кириллицей.)

**Крайние случаи.** Пустой ввод, отсутствующие файлы, битые данные, большие объёмы — что делать в каждом.

**Вне рамок.** Что исполнитель НЕ делает, даже если хочется (соседние задачи — со ссылками).

**Готово, когда.** Список проверяемых условий, совпадающий с критериями задачи, и точные команды проверки (сборка, форматтер, линтер с warnings-as-errors, тесты, ручной прогон, если нужен).
```

### Depth rules (Sonnet-proofing)

- **Paths, names, signatures — exact.** Never "add a module for X"; always `crates/store/src/schema.rs`, `pub fn open(path: &Path) -> Result<Store, StoreError>`.
- **Spell out non-obvious logic.** Algorithms, state machines, SQL, parsing rules: give pseudocode or the actual code for the tricky 20%. Leave only the routine 80% to the executor.
- **Retell the ADR rule you rely on.** A link alone will be skimmed.
- **Name the traps.** If you noticed a way to get it subtly wrong while investigating, it goes into «Ловушки».
- **Tests with data.** Each test has concrete input and expected output; every acceptance criterion is covered by at least one test in the table.
- **Order for green builds.** Scaffold types first, then logic, then wiring, then tests that exercise the whole path.
- **Close the scope.** «Вне рамок» lists what's tempting but belongs to another task.
- **No open architectural questions.** Anything that affects other modules is decided in the plan; the executor chooses only local details.

For **research tasks** the plan is a technical brief (ТЗ) for `researcher`: the exact questions (numbered), sources with URLs and what to clone where, what not to do (don't run third-party code), the method for each question, the report's path and section structure, how to mark verified vs claimed facts, and what «answered» means for each question.

For **test tasks** — the behaviour to specify case by case (input → expected), where fixtures go, the expected-result format with an example, and the traps each case must contain.

## 4. Self-check the plan, then mark ready

Read the plan as the Sonnet executor would, and answer honestly:
1. Could an engineer who has never seen this project do it from the task file alone, without asking anything?
2. Is there any step where two reasonable engineers would build different things? — make it precise.
3. Does every criterion map to a step and a test?
4. Are the traps you know about written down?

Only then set the task `ready` with a journal line «план записан», and hand it off with the `dispatch` skill.
