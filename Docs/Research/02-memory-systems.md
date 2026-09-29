# 02. Системы памяти агентов: agentmemory, claude-mem, OpenViking

> Задача: F01-02 «Разбор систем памяти агентов».
> Дата: 2026-09-29. Исследователь: Claude (субагент).
> Главный вопрос: можно ли взять agentmemory или claude-mem как готовый движок для **журнала работы (3a)** и что из трёх систем пригодно для **знаний проекта (3b)**, которые лежат в git как Markdown и должны нормально сливаться.

## Что разбирали

| | agentmemory | claude-mem | OpenViking |
|---|---|---|---|
| Репозиторий | `rohitg00/agentmemory` (npm `@agentmemory/agentmemory` 0.9.29) | `thedotmack/claude-mem` (npm `claude-mem` 13.28.0) | `volcengine/OpenViking` |
| Что это | Сервер памяти + плагин Claude Code (хуки, MCP) | Плагин Claude Code: хуки, фоновый «наблюдатель», MCP | «База контекста»: виртуальная файловая система `viking://` для знаний, памяти и навыков, плюс плагины для агентов |
| Язык | TypeScript (≈48 тыс. строк в `src/`) | TypeScript под **Bun** (≈85 тыс. строк в `src/`) | Python (≈72 тыс. строк) + Rust и C++ (≈121 тыс. строк) |
| Лицензия | Apache-2.0. **Но** движок iii-engine, без которого сервер не работает, — Elastic License 2.0 (исходники открыты, но это не open source) | Apache-2.0 (open-core: командная синхронизация и облако — закрытая платная часть) | **AGPL-3.0** (основной проект); Apache-2.0 только у `crates/ov_cli`, `crates/ragfs` и `examples/` |
| Что нужно для запуска | Node ≥ 20 + отдельный бинарник iii-engine (Rust) или Docker | Node ≥ 20, **Bun** ≥ 1.1, **uv + Python** (для Chroma, можно выключить), вход в Claude (наблюдатель тратит вашу подписку) | Python ≥ 3.10, свой сервер, модель эмбеддингов и LLM (облачные или локальные через Ollama) |
| Windows | Движок на Windows **не ставится автоматически**, только вручную `iii.exe`, через Docker или WSL2 | Заявлена. Хуки запускаются через `bash` (на Windows нужен Git Bash), Chroma поднимается через `uvx` | Заявлена через колёса (wheels); при сбое нужна сборка через CMake/MinGW |
| Разобранный коммит | `ab3e4ef` (2026-09-28) | `ade13f3` (2026-09-28) | `af11ccb` (2026-09-29) |

Репозитории склонированы с `--depth 1`. Код читался, но не запускался.

**Как читать ссылки.** Все ссылки ведут на конкретный коммит.
- **[код]** — проверил по исходникам сам;
- **[заявлено]** — так написано в README или документации, в коде до конца не проверял;
- **[не нашёл]** — искал в коде и не нашёл.

Корни ссылок:
- `AM:` = `https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/`
- `CM:` = `https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/`
- `OV:` = `https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/`

---

## Главное коротко

1. **Ни agentmemory, ни claude-mem не стоит брать как готовый движок журнала работы.** Обе системы — это тяжёлые фоновые серверы со своими портами, своими процессами и своей базой. agentmemory требует движок iii-engine под лицензией ELv2, который на Windows приходится ставить вручную. claude-mem требует Bun, по умолчанию тянет Python/Chroma, включает телеметрию и запускает второй процесс Claude, который тратит подписку пользователя. Вытащить из них «ядро» нельзя: весь код завязан на их рантайм. Журнал работы проще написать самим, а удачные куски кода и идеи взять оттуда.
2. **agentmemory без ключа LLM почти ничего «умного» не делает.** По умолчанию наблюдение — это обрезанный лог вызова инструмента: важность всегда 5, понятий нет. Сводки сессий, объединение, извлечение понятий без ключа API отключены. Выдача контекста в новую сессию тоже выключена по умолчанию (`AGENTMEMORY_INJECT_CONTEXT`). Почти всё из README («4 уровня памяти», «обнаружение противоречий», «гибридный поиск на старте сессии») либо требует ключа, либо в коде проще, чем описано.
3. **Самый полезный для нас механизм agentmemory — цепочка версий фактов.** При сохранении новой записи ищется похожая. Если сходство слов больше 0.7, старая запись помечается «не последняя» и уходит из поиска, но остаётся в истории (`supersedes`, `parentId`, `version`). Если сходство 0.4–0.7, сохранение проходит, а в ответ возвращается подсказка `similarTo`: «похоже на запись X». Идея хорошая, но сходство считается по словам (Jaccard), а не по смыслу. Две разные записи с похожими словами могут ошибочно заменить одна другую.
4. **Самые полезные идеи claude-mem — «сначала оглавление, потом подробности» и типы наблюдений.** В начале сессии агент получает не тексты, а список «ID · время · тип · заголовок». Подробности он добирает сам: `search` → `timeline` → `get_observations([ID])` → при необходимости сырой вывод инструмента. Наблюдения пишет отдельная модель-«наблюдатель» по чёткой схеме: тип (`bugfix`, `feature`, `decision`, `discovery` и др.), понятия (`gotcha`, `trade-off`, `problem-solution`), факты, связанные файлы.
5. **Главная ценность OpenViking для нас — не память, а устройство уровней L0/L1/L2 и схема записей.** Для файла кода L0 — это скелет из tree-sitter без вызова LLM. Для папки L0/L1 генерирует LLM по кратким описаниям детей. Поиск сначала находит подходящие папки и потом спускается внутрь них. Записи памяти — Markdown-файлы по YAML-схеме: у каждой записи стабильное имя файла (`preferences/<user>/<topic>.md`), а у каждого поля своё правило слияния (`immutable`, `patch`, `replace`, `sum`). Для знаний проекта в git это лучшая из увиденных схем. Код брать нельзя (AGPL), идеи — можно.
6. **Ни одна из трёх систем не решает задачу «общая память команды через git».** В agentmemory «командная память» — это общий сервер или P2P-синхронизация по HTTP с правилом «последняя запись побеждает». «Git-снимки» там — один `state.json` в приватном репозитории. У claude-mem командная синхронизация вынесена в закрытую платную часть. У OpenViking есть версии через встроенный git, но без слияния, push и pull. Слой 3b нам придётся проектировать самим.
7. **Автоматические `CLAUDE.md` по папкам у claude-mem (выключены по умолчанию) в git класть нельзя.** В файле лежит таблица ID наблюдений из **личной** базы автора. У другого участника эти ID ничего не значат, а таблица меняется после каждой сессии и даёт конфликты при слиянии. Полезен сам приём: автоматический блок живёт между тегами `<claude-mem-context>…</claude-mem-context>` и заменяется целиком, не трогая ручной текст рядом.
8. **Встроенная память Claude Code сознательно не хранит то, что можно вывести из кода.** Архитектура, пути файлов, исправления при отладке в неё не попадают. Она личная, живёт на одной машине и не лежит в git. Значит, наш журнал работы (что пробовали, что сломалось, где тупик) и знания проекта (решения и правила для всей команды) закрывают как раз её пробелы. Опасная интеграция: мост agentmemory `claude-bridge` при включении **перезаписывает** `MEMORY.md` Claude Code целиком, причём памятью всех проектов. Так делать нельзя.
9. **Бенчмарки всех трёх систем — про память о переписке, а не про код.** Цифры получены самими авторами. У agentmemory 95.2% R@5 на LongMemEval-S — это только поиск нужной сессии среди ~48 без ответа на вопрос. На собственном «кодовом» наборе agentmemory обычный grep почти догоняет гибридный поиск: R@5 0.967 против 1.000. У OpenViking LoCoMo измерен на моделях Doubao. Для выбора решения эти цифры почти ничего не дают.
10. **Приватность везде неполная.** agentmemory вырезает `<private>` и маскирует ~15 шаблонов секретов, но импорт старых логов (`import-jsonl`) этот фильтр **обходит**. claude-mem вырезает `<private>` и служебные теги, а секреты не маскирует [не нашёл]. Если весь запрос пользователя приватный, наблюдения за этот ход не пишутся. OpenViking в плагине Claude Code `<private>` не обрабатывает [не нашёл], есть только пользовательские regex-фильтры.

---

## Встроенная память Claude Code: что уже есть

Проверено по официальной документации (`code.claude.com/docs/en/memory`) **[заявлено]**:

| Механизм | Кто пишет | Где | Общая через git? | Что загружается |
|---|---|---|---|---|
| `CLAUDE.md` / `.claude/CLAUDE.md` | человек | в репозитории | да | целиком при старте (рекомендуется < 200 строк) |
| `CLAUDE.local.md` | человек | в репозитории, в `.gitignore` | нет | целиком |
| `.claude/rules/*.md` с `paths:` | человек | в репозитории | да | только когда агент читает подходящие файлы |
| `CLAUDE.md` в подпапках | человек | в репозитории | да | когда агент читает файлы в этой папке |
| Auto memory | Claude | `~/.claude/projects/<repo>/memory/` (`MEMORY.md` + тема-файлы) | **нет**, одна машина | первые 200 строк или 25 КБ `MEMORY.md`; тема-файлы — по запросу |

Что важно для нас:
- Auto memory хранит 4 вида заметок: `user`, `feedback`, `project`, `reference`. Документация прямо говорит, что Claude **пропускает** всё, что можно вывести из кода: архитектуру, пути файлов, исправления при отладке. Наш журнал работы (3a) закрывает именно эти пробелы.
- Auto memory общая для всех worktree одного репозитория, но не для людей. Наши знания проекта (3b) нужны для обмена между людьми через git.
- При записи Claude Code сам проставляет поле `modified` во frontmatter. Это хороший пример метки свежести.
- **Правило для нас:** в `~/.claude/projects/*/memory/` не пишем. `CLAUDE.md` генерируем только внутри своего тегированного блока, если вообще генерируем. Для правил, привязанных к путям, лучше использовать `.claude/rules/` с `paths:`: Claude Code загрузит их сам и тогда, когда нужно.

---

## agentmemory

### Что это и для чего

Сервер памяти для coding-агентов. Хуки Claude Code шлют события по HTTP на локальный сервер (порт 3111). Сервер хранит наблюдения, ищет по ним (BM25, по желанию векторы и граф) и отдаёт 54 MCP-инструмента. Сам сервер — набор функций внутри движка **iii-engine** (отдельный процесс на Rust, WebSocket-порт 49134). В `AGENTS.md` прямо написано: всё только через `registerFunction/trigger`, «никогда не обходить iii-engine» ([AM:AGENTS.md](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/AGENTS.md)).

### Как устроено на самом деле

**Запись наблюдений хуками.** **[код]**
- 12 хуков: `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PreCompact`, `SubagentStart/Stop`, `Notification`, `TaskCompleted`, `Stop`, `SessionEnd` ([AM:plugin/hooks/hooks.json](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/plugin/hooks/hooks.json)).
- Хук — маленький Node-скрипт. Он читает JSON из stdin, отправляет `fetch` без ожидания ответа и через 500 мс завершает процесс, чтобы не тормозить Claude Code ([AM:src/hooks/post-tool-use.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/hooks/post-tool-use.ts)). Вывод инструмента обрезается до 8000 символов, картинки вынимаются отдельно.
- Проект определяется по **имени папки** корня git (`basename`) ([AM:src/hooks/_project.ts#L17](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/hooks/_project.ts#L17)). Два разных репозитория с именем `app` сольются в один проект.
- Защита от рекурсии: если хук вызван из дочерней сессии Agent SDK, он ничего не делает. Иначе сжатие через Claude породило бы новые наблюдения.

**Обработка на сервере** (`mem::observe`, [AM:src/functions/observe.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/observe.ts)). **[код]**
1. Дубли: хеш SHA-256 от «сессия + инструмент + первые 500 символов входа». Хеш хранится **в памяти процесса 5 минут** ([AM:src/functions/dedup.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/dedup.ts)). Повторное чтение того же файла в течение 5 минут не запишется, даже если файл изменился. После перезапуска сервера дедупликация сбрасывается.
2. Приватность: `<private>…</private>` вырезается, ~15 шаблонов секретов (ключи `sk-`, `ghp_`, AWS `AKIA…`, JWT, Bearer и т.п.) заменяются на `[REDACTED_SECRET]` ([AM:src/functions/privacy.ts#L22](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/privacy.ts#L22)).
3. Источник: каждому наблюдению ставится `origin.channel` — `user` (запрос), `tool` (инструмент), `agent`, `import`.
4. Сжатие: **по умолчанию без LLM** ([AM:observe.ts#L319](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/observe.ts#L319)). «Синтетическое сжатие» — это тип по имени инструмента, заголовок = имя инструмента, текст = склейка запроса, входа и выхода, обрезанная до 400 символов, `importance: 5`, `confidence: 0.3`, `concepts: []` ([AM:src/functions/compress-synthetic.ts#L99](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/compress-synthetic.ts#L99)). LLM-сжатие по XML-схеме (тип, факты, понятия, важность 1–10) включается только при ключе провайдера и `AGENTMEMORY_AUTO_COMPRESS=true` ([AM:src/prompts/compression.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/prompts/compression.ts)).

**Сжатие и объединение.** **[код]**
- Сводка сессии по `Stop`/`SessionEnd` без LLM-провайдера **не делается**: функция возвращает `no_provider` ([AM:src/functions/summarize.ts#L264](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/summarize.ts#L264)). Провайдер по умолчанию — `noop`. Подписку Claude можно подключить через Agent SDK, но README сам предупреждает о риске рекурсии хуков.
- Объединение `mem::consolidate` группирует наблюдения по понятиям ([AM:src/functions/consolidate.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/consolidate.ts)). У синтетических наблюдений понятий нет, поэтому без LLM объединять нечего.
- «Уроки» из импортированных логов вытаскиваются регулярными выражениями по словам `always|never|don't|make sure|prefer|avoid` ([AM:src/functions/replay.ts#L87](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/replay.ts#L87)).

**Хранение.** **[код]**
- Ключ-значение в iii-engine (`store_method: file_based`, `./data/state_store.db`) ([AM:iii-config.yaml](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/iii-config.yaml)). Области: `mem:obs:<session>`, `mem:memories`, `mem:lessons` и ещё ~50 других ([AM:src/state/schema.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/state/schema.ts)).
- Индекс BM25 и векторный индекс хранятся в памяти процесса и периодически сохраняются. Есть «лёгкий» режим MCP без движка: JSON-файл, но только 5–6 инструментов и без хуков ([AM:src/mcp/standalone.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/mcp/standalone.ts)).

**Поиск.** **[код]** BM25 со стеммингом и синонимами — всегда. Векторы — если настроен провайдер эмбеддингов (по умолчанию нет). Граф — если есть данные графа. Слияние по RRF (k=60) с бонусом за совпадение в нескольких потоках ([AM:src/state/hybrid-search.ts#L20](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/state/hybrid-search.ts#L20)).

**Выдача контекста в новой сессии.** **[код]**
- По умолчанию **выключена**: `session-start` только регистрирует сессию и ничего не выводит, если нет `AGENTMEMORY_INJECT_CONTEXT=true` ([AM:src/hooks/session-start.ts#L19](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/hooks/session-start.ts#L19)). Причина описана в комментарии [AM:src/hooks/pre-tool-use.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/hooks/pre-tool-use.ts): инъекция ~1000 токенов на каждый вызов инструмента съедала лимиты подписки.
- Если включить, `mem::context` собирает: закреплённые «слоты», профиль проекта, до 10 уроков и сводки (или 5 самых важных наблюдений) **последних 10 сессий** проекта в пределах бюджета 2000 токенов ([AM:src/functions/context.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/context.ts)). Поиска по смыслу здесь нет, хотя README пишет «Hybrid search» на SessionStart.

### Механизмы качества

| Механизм | Как сделано | Метка |
|---|---|---|
| Замена версий | `mem::remember`: берёт 50 кандидатов из BM25 и считает Jaccard по словам длиннее 2 букв. Сходство > 0.7 → старая запись `isLatest=false`, убирается из обоих индексов, у новой `version+1`, `parentId`, `supersedes` ([AM:src/functions/remember.ts#L135](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/remember.ts#L135)). Замена не пересекает границы проектов | [код] |
| `similarTo` | Сходство 0.4–0.7 → запись сохраняется, в ответ приходит подсказка «похоже на X» ([AM:remember.ts#L257](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/remember.ts#L257)). Только совет, сама система ничего не делает | [код] |
| Источник записи | `origin.channel` у наблюдений и памяти; `sourceObservationIds` у памяти; `mem::verify` возвращает «цитаты» — исходные наблюдения ([AM:src/functions/verify.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/verify.ts)) | [код] |
| Дубли | Наблюдения — хеш в памяти процесса на 5 минут. Память — через замену версий | [код] |
| «Противоречия» | `auto-forget`: Jaccard > 0.9 у записей с общим понятием → старшая помечается устаревшей ([AM:src/functions/auto-forget.ts#L12](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/auto-forget.ts#L12)). На деле это поиск **почти дублей**, а не противоречий | [код] |
| Забывание | TTL (`forgetAfter`) у памяти. Удаление наблюдений старше 180 дней с важностью ≤ 2 ([AM:auto-forget.ts#L165](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/auto-forget.ts#L165)). У синтетических наблюдений важность всегда 5, поэтому без LLM это правило не срабатывает никогда | [код] |
| Оценки полезности | Журнал обращений (`recordAccessBatch`). «Удержание» по формуле: экспоненциальное затухание + бонус за недавние обращения ([AM:src/functions/retention.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/retention.ts)). У уроков `confidence` растёт при повторном подтверждении и падает на 0.05 в неделю ([AM:src/functions/lessons.ts#L55](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/lessons.ts#L55)) | [код] |
| Аудит | `recordAudit` на все изменяющие операции, помесячно | [код] |

### Импорт логов сессий Claude Code (JSONL)

**[код]** Команда `import-jsonl` обходит `~/.claude/projects/**.jsonl` ([AM:src/functions/replay.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/replay.ts), разбор — [AM:src/replay/jsonl-parser.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/replay/jsonl-parser.ts)). Сопоставляет `tool_use` и `tool_result` по id, строит синтетические наблюдения с `origin: import` и индексирует их. Три замечания:
- **Фильтр приватности при импорте не вызывается** (`stripPrivateData` в этих файлах не используется) [код]. Секреты и `<private>` из старых логов попадут в базу.
- ID наблюдений генерируются заново при каждом разборе ([AM:jsonl-parser.ts#L146](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/replay/jsonl-parser.ts#L146)), поэтому повторный импорт того же файла создаёт дубли [код].
- README справедливо предупреждает: Claude Code удаляет JSONL старше `cleanupPeriodDays` (по умолчанию 30 дней) [заявлено].

Сам парсер JSONL небольшой и отделяется легко. Его можно взять за основу для нашего импорта истории, дописав фильтр приватности и стабильные ID (например, `uuid` строки JSONL).

### Общая память команды через git

**Не подходит.** **[код]**
- «Командная память» `mem::team-share` — это отдельная область ключей на **том же сервере** ([AM:src/functions/team.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/team.ts)). Команде нужен общий удалённый сервер.
- P2P-синхронизация `mesh` — HTTP-обмен между серверами с правилом «последняя по `updatedAt` запись побеждает» ([AM:src/functions/mesh.ts#L77](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/mesh.ts#L77)). Адреса в локальной сети и `localhost` при этом запрещены.
- «Git-снимки» кладут **весь** `state.json` в отдельный приватный репозиторий ([AM:src/functions/snapshot.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/snapshot.ts)). Один большой JSON нельзя нормально слить.
- Экспорт в Obsidian — только в одну сторону, в `~/.agentmemory/vault` ([AM:src/functions/obsidian-export.ts](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/obsidian-export.ts)).

### Сочетание со встроенной памятью Claude Code

`claude-bridge` (включается `CLAUDE_MEMORY_BRIDGE=true`) **перезаписывает** `~/.claude/projects/<slug>/memory/MEMORY.md` своим текстом. Туда попадают все последние записи памяти **всех проектов**: фильтра по проекту нет ([AM:src/functions/claude-bridge.ts#L121](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/claude-bridge.ts#L121), [#L142](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/functions/claude-bridge.ts#L142)). Индекс auto memory, который ведёт сам Claude, при этом теряется. «Двусторонняя синхронизация» из README на деле — чтение разделов в служебный ключ и полная перезапись файла [код].

### Полезное для наших слоёв

| Слой | Что взять |
|---|---|
| Карта кода | — (README советует ставить рядом CodeGraph или Graphify) |
| Последствия правок | — |
| Журнал работы (3a) | Хуки «отправил и вышел за 500 мс»; защита от рекурсии Agent SDK; `origin.channel`; регулярки секретов; парсер JSONL Claude Code |
| Знания проекта (3b) | Цепочка версий `supersedes/parentId/isLatest` + «устаревшее уходит из поиска, но остаётся в истории»; подсказка `similarTo` при создании записи; `sourceObservationIds` как ссылка знания на журнал; `confidence` с усилением при подтверждении |
| Документы | — |
| Проверка | `mem::verify` — «покажи, из каких наблюдений выведено знание» |

### Можно ли взять код

- Лицензия самого agentmemory — Apache-2.0: брать можно, с указанием авторства и файлом NOTICE.
- **Как движок — нет.** Весь серверный код — функции внутри iii-engine. Движок лицензирован под ELv2, это отдельный бинарник и демон с 4 портами. На Windows CLI не умеет ставить его сам ([AM:src/cli.ts#L1306](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/src/cli.ts#L1306)), README предлагает скачать `iii.exe` вручную, взять Docker или WSL2. Для нашего плагина это неприемлемый порог установки.
- **Отдельные файлы — да.** Это чистые функции без зависимости от iii: `privacy.ts` (регулярки), `jsonl-parser.ts`, `compress-synthetic.ts`, логика замены в `remember.ts` (без обёртки `sdk`), `dedup.ts`, BM25 из `src/state/search-index.ts`.

### Слабые места

- Без ключа LLM система — поиск BM25 по обрезанным логам инструментов.
- Сходство по словам вместо смысла: возможны ложные замены версий.
- Проект определяется по имени папки, поэтому возможны коллизии.
- Очень широкий охват (54 инструмента, 134 REST-адреса, лизинги, сигналы, «сентинели», «кристаллы»), при этом ~30 тыс. звёзд у проекта, которому 7 месяцев. Много функций, и многие из них по умолчанию не работают.
- Нет общей памяти через git.

### Бенчмарки

LongMemEval-S (R@5 95.2%) измеряет **только поиск нужной сессии разговора** среди ~48, без генерации ответа ([AM:benchmark/LONGMEMEVAL.md](https://github.com/rohitg00/agentmemory/blob/ab3e4efd282659b87d31b36c8514c6bc6b871f0b/benchmark/LONGMEMEVAL.md)). На собственном кодовом наборе `coding-agent-life-v1` (15 сессий) grep даёт R@5 0.967 против 1.000 у гибридного поиска. «92% меньше токенов» и «$10 в год» — расчёт самих авторов. Всё это самоотчёты о памяти переписки [заявлено].

### Вердикт

**Движок — не брать. Код — взять точечно** (регулярки секретов, парсер JSONL, логика замены версий и `similarTo`). **Идеи — взять** (цепочка версий, источник записи, «устаревшее вне поиска, но в истории»).

---

## claude-mem

### Что это и для чего

Плагин Claude Code, самый популярный из трёх (~95 тыс. звёзд). Хуки отправляют события в локальный **worker** (HTTP-сервер на Bun). Worker запускает **отдельную сессию Claude** — «наблюдателя». Наблюдатель смотрит на поток вызовов инструментов и пишет структурированные наблюдения. В начале следующей сессии агент получает их оглавление.

### Как устроено на самом деле

**Запись наблюдений хуками.** **[код]**
- Хуки: `Setup`, `SessionStart` (запуск worker + выдача контекста), `UserPromptSubmit`, `PostToolUse` (async), `PreToolUse` на `Read` (async), `Stop` (сводка, async), `SessionEnd` ([CM:plugin/hooks/hooks.json](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/plugin/hooks/hooks.json)). Все команды запускаются через `"shell": "bash"` и длинный bash-скрипт поиска версии плагина. На Windows без Git Bash это не заработает.
- Обработчик `PostToolUse` пересылает `tool_name/tool_input/tool_response` в worker ([CM:src/cli/handlers/observation.ts](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/cli/handlers/observation.ts)). Worker вырезает теги из входа и выхода, пишет **сырой** вызов в таблицу `tool_uses` (ключ — `tool_use_id`) и ставит его в очередь наблюдателю ([CM:src/services/worker/http/shared.ts#L136](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/worker/http/shared.ts#L136)).
- Инструменты `TodoWrite`, `Skill`, `AskUserQuestion` и т.п. пропускаются (`CLAUDE_MEM_SKIP_TOOLS`).

**`<private>` и служебные теги.** **[код]**
- Вырезаются `<private>`, `<claude-mem-context>`, `<system-reminder>`, `<persisted-output>`, `<system_instruction>` ([CM:src/utils/tag-stripping.ts](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/utils/tag-stripping.ts)). Вырезание собственного `<claude-mem-context>` не даёт памяти «записывать саму себя» по кругу.
- Если запрос пользователя после вырезания пуст (весь ход приватный), наблюдения и сводка за этот ход **не создаются** ([CM:src/services/worker/validation/PrivacyCheckValidator.ts#L40](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/worker/validation/PrivacyCheckValidator.ts#L40)).
- Маскирования секретов в выводе инструментов нет [не нашёл]. Весь вывод уходит модели-наблюдателю. Если наблюдатель настроен на OpenRouter или Gemini, секреты уйдут третьей стороне.

**Сжатие («наблюдатель»).** **[код]**
- По умолчанию: провайдер `claude`, модель `claude-haiku-4-5`, авторизация — **подписка пользователя** ([CM:src/shared/SettingsDefaultsManager.ts#L193](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/shared/SettingsDefaultsManager.ts#L193)). Работает до 2 параллельных подпроцессов Claude. Установщик предлагает платный хостинг наблюдателя (CMEM Pro) [заявлено в README].
- Промпт и схема задаются «режимом» ([CM:plugin/modes/code.json](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/plugin/modes/code.json)):
  - **типы наблюдений:** `bugfix`, `feature`, `refactor`, `change`, `discovery`, `decision`, `security_alert`, `security_note`, `sensitive`;
  - **понятия:** `how-it-works`, `why-it-exists`, `what-changed`, `problem-solution`, `gotcha`, `pattern`, `trade-off`;
  - правило «записывай, что узнали, построили или исправили, а не что делает наблюдатель», правило «пропускай рутину» (пустые проверки, листинги, установки без ошибок).
- Сводка сессии по `Stop`: поля `request`, `investigated`, `learned`, `completed`, `next_steps`, `files_read`, `files_edited`, `notes` ([CM:src/services/sqlite/SessionStore.ts#L1064](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/sqlite/SessionStore.ts#L1064)).
- Объединения наблюдений между сессиями (консолидации) нет [не нашёл].

**Хранение.** **[код]**
- SQLite через `bun:sqlite`, `~/.claude-mem/claude-mem.db`, WAL ([CM:src/services/sqlite/connection.ts](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/sqlite/connection.ts)). Таблицы `sdk_sessions`, `observations` (тип, заголовок, подзаголовок, факты, повествование, понятия, `files_read`, `files_modified`, `content_hash`…), `session_summaries`, `user_prompts`, `tool_uses`, FTS5-индексы ([CM:SessionStore.ts#L1048](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/sqlite/SessionStore.ts#L1048)).
- Векторы — в Chroma, который запускается как `chroma-mcp` через `uvx` (нужны Python и uv), включён по умолчанию ([CM:src/services/sync/ChromaMcpManager.ts](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/sync/ChromaMcpManager.ts)).

**Поиск.** **[код]** Сначала Chroma. Если она пуста — SQLite. Если Chroma не поднялась — FTS5 по ключевым словам ([CM:src/services/worker/SearchManager.ts#L565](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/worker/SearchManager.ts#L565)). В `SearchOrchestrator` без Chroma текстовый запрос возвращает пустой результат ([CM:src/services/worker/search/SearchOrchestrator.ts#L75](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/worker/search/SearchOrchestrator.ts#L75)). Два пути поиска ведут себя по-разному.

**Выдача контекста в новой сессии.** **[код]**
- `SessionStart` выводит оглавление: 50 последних наблюдений проекта в формате `ID ВРЕМЯ ТИП ЗАГОЛОВОК`, последнюю сводку и легенду «детали — `get_observations([IDs])`» ([CM:src/services/context/ContextBuilder.ts](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/context/ContextBuilder.ts), [CM:src/services/context/formatters/AgentFormatter.ts](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/context/formatters/AgentFormatter.ts)). Отбор — по свежести, а не по смыслу.
- Записи, сохранённые вручную (`/api/memory/save`), не имеют понятий. Они не проходят фильтр понятий режима и в оглавление на старте **не попадают** ([CM:src/services/context/ObservationCompiler.ts#L59](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/context/ObservationCompiler.ts#L59)).
- Подмешивание по смыслу на каждый запрос (`CLAUDE_MEM_SEMANTIC_INJECT`) есть, но выключено.

**Ссылки на наблюдения по ID и «сначала оглавление».** **[код]** MCP-сервер объявляет порядок работы ([CM:src/servers/mcp-server.ts#L443](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/servers/mcp-server.ts#L443)):
1. `search(query)` → таблица ID (≈50–100 токенов на строку);
2. `timeline(anchor=ID)` → что было до и после;
3. `get_observations([ID])` → полный текст только выбранных;
4. `get_tool_uses([ID])` → сырой вход и выход инструмента, «в последнюю очередь».

**Контекст при чтении файла** (`PreToolUse` на `Read`). **[код]** Если у файла (> 1500 байт) есть наблюдения, **и файл не менялся после последнего наблюдения** (сравнение `mtime`), агенту подкладывается хронология «ID · дата · заголовок». Повторно за сессию она не подкладывается ([CM:src/cli/handlers/file-context.ts#L263](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/cli/handlers/file-context.ts#L263)). Документация пишет, что чтение **блокируется** (`DENY`), а код отвечает `permissionDecision: 'allow'` с дополнительным контекстом ([#L191](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/cli/handlers/file-context.ts#L191)). Хук помечен `async: true`. Успевает ли контекст прийти до чтения — не проверял.

**Автоматические `CLAUDE.md` в папках.** **[код]** Выключено по умолчанию (`CLAUDE_MEM_FOLDER_CLAUDEMD_ENABLED=false`). После обработки наблюдений для каждой затронутой папки (кроме корня репозитория, `node_modules`, `build`, `.git` и папок, где `CLAUDE.md` правят прямо сейчас) запрашивается хронология наблюдений по папке. Она пишется в `CLAUDE.md` (или `CLAUDE.local.md`) между тегами `<claude-mem-context>…</claude-mem-context>`: ручной текст вне тегов сохраняется, запись атомарная через `.tmp` + `rename` ([CM:src/utils/claude-md-utils.ts#L57](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/utils/claude-md-utils.ts#L57), [#L227](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/utils/claude-md-utils.ts#L227)). Содержимое — таблица `| ID | Time | T | Title | Read |`. Эти ID указывают в **личную** базу автора наблюдений, время разбирается из строк «3:25 PM».

**Карта кода внутри claude-mem.** **[код]** Есть MCP-инструменты `smart_search`, `smart_outline`, `smart_unfold`: структура файла через tree-sitter со «свёрнутыми» телами функций и раскрытие одного символа ([CM:mcp-server.ts#L701](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/servers/mcp-server.ts#L701)). Это пересекается с нашей картой кода.

### Механизмы качества

| Механизм | Как сделано | Метка |
|---|---|---|
| Замена версий | Нет. Наблюдения только добавляются | [не нашёл] |
| Источник записи | `platform_source` (claude/codex/cursor…), `agent_id/agent_type`, `generated_by_model`, `prompt_number`; связь наблюдения с сырым `tool_uses` по сессии | [код] |
| Дубли | `UNIQUE(memory_session_id, content_hash)` — хеш от «сессия + заголовок + текст», `ON CONFLICT DO NOTHING` ([CM:SessionStore.ts#L3149](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/sqlite/SessionStore.ts#L3149)). Только внутри одной сессии; одинаковые выводы из разных сессий не склеиваются. Наблюдатель в промпте просят не повторяться | [код] |
| Забывание | Нет, только ручное удаление и исключение проектов | [не нашёл] |
| Оценки полезности | Столбец `relevance_count` есть в схеме, но в `src/` его никто не увеличивает | [код] |

### Общая память команды через git

**Не подходит.** База личная (`~/.claude-mem`). Синхронизация между устройствами идёт только через облако `cmem.ai` (`CLAUDE_MEM_CLOUD_SYNC_*`). «Team/org memory sync» прямо указан в [CM:docs/ip-boundary.md](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/docs/ip-boundary.md) как закрытая коммерческая часть. `CLAUDE.md` по папкам в git класть нельзя по причинам выше.

### Сочетание со встроенной памятью Claude Code

С auto memory не пересекается: в `~/.claude/projects/*/memory` не пишет [не нашёл]. Наблюдения, где агент правит файлы `session-memory`, пропускаются. С `CLAUDE.md` взаимодействует через тегированный блок (опционально) и через `CLAUDE.local.md`.

### Полезное для наших слоёв

| Слой | Что взять |
|---|---|
| Карта кода | Идея `smart_outline`/`smart_unfold` (свёрнутая структура файла + раскрытие одного символа). Мы и так это планируем |
| Последствия правок | — |
| Журнал работы (3a) | Схема наблюдения (тип + понятия + факты + файлы); промпт «пиши, что узнали, а не что делал; пропускай рутину»; двойная запись: сырой `tool_uses` + сжатое наблюдение; вырезание своих же инъекций; «приватный ход → ничего не пишем» |
| Выдача | Оглавление с ID на старте + `search → timeline → get → raw`; контекст к файлу при чтении, только если файл **не менялся** после наблюдения, и не чаще раза за сессию |
| Знания проекта (3b) | Тип `decision` и понятия `trade-off`, `gotcha` как кандидаты на перенос в знания; сводка сессии с полями `learned`/`next_steps` |
| Документы | Приём тегированного блока в Markdown, который заменяется целиком, не трогая ручной текст |
| Проверка | — |

### Можно ли взять код

- Лицензия Apache-2.0 (open-core). Брать можно с указанием авторства.
- **Как движок — нет.** Для этого нужны Bun (`bun:sqlite` в хранилище), worker-демон с HTTP API, по умолчанию `uvx`/Python для Chroma и отдельный процесс Claude, который тратит подписку пользователя. Телеметрия PostHog включена по умолчанию, отключается через `DO_NOT_TRACK` или `CLAUDE_MEM_TELEMETRY=0` ([CM:src/services/telemetry/consent.ts#L43](https://github.com/thedotmack/claude-mem/blob/ade13f3067d3de486f41dbd8ed90ac5c854de718/src/services/telemetry/consent.ts#L43)). Настройки по умолчанию включают интеграции «Grok Bot» с зашитыми ID агентов, Telegram, воронку платной подписки, а README рекламирует криптотокен CMEM. Для командного инструмента это лишний риск.
- **Отдельные файлы — да, небольшие:** `tag-stripping.ts`, `replaceTaggedContent` из `claude-md-utils.ts`, промпты режима `code.json` (как образец), логика «не менялся ли файл» из `file-context.ts`.

### Слабые места

- Нет замены версий, забывания, объединения между сессиями. Всё копится.
- Качество целиком зависит от модели-наблюдателя. Это тратит токены и добавляет задержку (наблюдения асинхронные).
- Отбор на старте только по свежести.
- Много вспомогательного кода (установщик, Windows-обходы, облако, боты), почти 85 тыс. строк.
- Документация расходится с кодом (блокировка чтения).

### Бенчмарки

Академических бенчмарков в репозитории нет. «~10x экономия токенов» у трёхшагового поиска — оценка авторов [заявлено].

### Вердикт

**Движок — не брать. Идеи — взять:** оглавление с ID, трёхшаговый поиск, схема наблюдения, контекст к файлу с проверкой свежести, тегированный блок, правила приватности. **Код — только мелкие функции.**

---

## OpenViking

### Что это и для чего

«База контекста» от ByteDance/Volcengine. Всё, что знает агент, лежит в виртуальной файловой системе `viking://`: `resources/` (документы, репозитории), `user/<id>/memories/` (память), `skills/`. Агент ходит по ней как по папкам (`ls`, `tree`, `read`, `grep`, `find`). Облачный вариант Volcengine мы не рассматриваем. Смотрим только сервер, развёрнутый у себя (`openviking-server`, порт 1933).

### Как устроено на самом деле

**Уровни L0/L1/L2.** **[код]**
- L0 — одно предложение («стоит ли сюда смотреть»), лежит в `.abstract.md` папки. L1 — обзор (`.overview.md`). L2 — сам файл, читается по запросу ([OV:openviking/retrieve/hierarchical_retriever.py#L58](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/retrieve/hierarchical_retriever.py#L60)).
- **Для файла кода** краткое описание — это **скелет из tree-sitter, без LLM**. LLM (`semantic.code_summary`) вызывается, только если скелет не получился ([OV:openviking/storage/queuefs/semantic_processor.py#L1278](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/storage/queuefs/semantic_processor.py#L1278)).
- **Для папки** L1 генерирует LLM по кратким описаниям файлов и L0 подпапок ([OV:openviking/prompts/templates/semantic/overview_generation.yaml](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/prompts/templates/semantic/overview_generation.yaml)).
- Обновление вверх по дереву: когда меняется ребёнок, родителю увеличивают счётчик `pending_child_changes` и ставят его в очередь на пересчёт. Черновик улучшения (`freshness-aware-parent-bubbling`) предлагает не пересчитывать родителя, если L0 ребёнка не изменился, а широкие папки пересчитывать только после накопления доли изменений ([OV:docs/design/freshness-aware-parent-bubbling-design.md](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/docs/design/freshness-aware-parent-bubbling-design.md)) [заявлено, дизайн-черновик].

**Поиск со сужением до папки.** **[код]** ([OV:hierarchical_retriever.py](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/retrieve/hierarchical_retriever.py))
1. Запрос превращается в вектор (плотный и, если есть, разреженный).
2. Стартовые папки — те, что указаны явно (`target_directories`), или папки по умолчанию для типа контекста.
3. Быстрый режим — один векторный поиск в пределах этих папок.
4. «Думающий» режим (есть reranker): глобальный поиск по L0/L1 даёт папки-кандидаты, дальше очередь с приоритетом спускается в детей лучших папок ([#L421](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/retrieve/hierarchical_retriever.py#L421)). Спуск останавливается, когда топ-результаты перестают меняться 3 раунда.
5. Оценка ребёнка может смешиваться с оценкой родителя (`score_propagation_alpha`), а итог — с «горячестью» записи (частота обращений × затухание с полупериодом 7 дней, [OV:openviking/retrieve/memory_lifecycle.py#L19](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/retrieve/memory_lifecycle.py#L19)). **По умолчанию оба смешивания выключены**: `hotness_alpha=0.0`, `score_propagation_alpha=1.0` ([OV:openviking_cli/utils/config/retrieval_config.py](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking_cli/utils/config/retrieval_config.py)).

**Запись из сессий и сжатие.** **[код]**
- Плагин Claude Code ([OV:examples/claude-code-memory-plugin/hooks/hooks.json](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/examples/claude-code-memory-plugin/hooks/hooks.json)): `UserPromptSubmit` — поиск и подмешивание памяти **на каждый запрос**; `Stop` — захват новых ходов из JSONL-лога сессии (`transcript_path`); `PreCompact`/`SessionEnd` — «коммит» сессии; `SubagentStart/Stop` — отдельная сессия памяти для субагента; `PreToolUse` — не даёт родным инструментам писать в `viking://`.
- Перед отправкой из текста вырезаются собственные инъекции (`<openviking-context>`, `<relevant-memories>`) и `<system-reminder>` ([OV:examples/claude-code-memory-plugin/scripts/shared/capture-utils.mjs#L590](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/examples/claude-code-memory-plugin/scripts/shared/capture-utils.mjs#L590)). Обработки `<private>` нет [не нашёл], есть только пользовательские regex-фильтры захвата.
- «Коммит» сессии: архив сообщений → LLM-извлекатель в режиме ReAct с инструментами `read/search/ls` по **существующей** памяти → операции записи, правки (patch) и удаления над Markdown-файлами ([OV:openviking/session/memory/extract_loop.py](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/session/memory/extract_loop.py), описание — [OV:docs/design/session-memory-extraction-flow.md](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/docs/design/session-memory-extraction-flow.md)).

**Схема записей памяти.** **[код]** Каждый тип памяти описан YAML-файлом ([OV:openviking/prompts/templates/memory/](https://github.com/volcengine/OpenViking/tree/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/prompts/templates/memory)): `profile`, `preferences`, `entities`, `events`, `cases`, `experiences`, `skills`, `tools`, `trajectories`. В схеме задаются:
- `directory` и `filename_template`, например `preferences/{{ user }}/{{ topic }}.md` — **стабильное имя файла = личность записи**. Новое знание на ту же тему правит тот же файл, а не создаёт новый;
- `content_template` — как выглядит Markdown;
- у каждого поля `merge_op`: `immutable` (не меняется после создания), `patch` (правка блоками SEARCH/REPLACE), `replace`, `sum`, `link_merge` ([OV:openviking/session/memory/merge_op/](https://github.com/volcengine/OpenViking/tree/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/session/memory/merge_op));
- `overview_template` — автоматическое оглавление папки со ссылками на файлы.

Пример `cases.yaml`: запись «случая» с полями `Task Signature`, `Input`, `Rubric`, `Evidence` и ссылками на выведенные из него «опыты» (`experiences`) ([OV:.../memory/cases.yaml](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/prompts/templates/memory/cases.yaml)).

**Хранение.** **[код]** Файлы — через RAGFS (Rust, `crates/ragfs`) на локальный диск или S3. Векторы — собственный движок на C++ (`src/`) или VikingDB. Версии — встроенный git (gitoxide) на уровне аккаунта: только `commit/restore/show`, **без веток, слияния, push и pull** ([OV:docs/design/git-version-control-design.md](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/docs/design/git-version-control-design.md), код — [OV:crates/ragfs/src/git/](https://github.com/volcengine/OpenViking/tree/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/crates/ragfs/src/git)).

**Выдача контекста.** **[заявлено]** На старте сессии — профиль, индекс памяти и каталог навыков, с лимитом 9500 байт, чтобы блок оставался внутри лимита Claude Code в 10 000 символов. На каждый запрос — до 10 результатов поиска в бюджете 2000 токенов ([OV:docs/en/agent-integrations/02-claude-code.md](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/docs/en/agent-integrations/02-claude-code.md)).

**Привязка к проекту.** **[заявлено]** «Peer» проекта берётся из **нормализованного URL `origin`** (`github.com-volcengine-openviking`), а не из имени папки. Можно задать его в `.openviking/config.json`, который команда коммитит, а личные настройки держать в `config.local.json` в `.gitignore` ([там же, #L109](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/docs/en/agent-integrations/02-claude-code.md#L109)).

### Механизмы качества

| Механизм | Как сделано | Метка |
|---|---|---|
| Замена версий | Не через «новая запись заменяет старую», а через правку того же файла (`patch`) по стабильному имени. История — git-снимки аккаунта | [код] |
| Источник записи | Ссылки и обратные ссылки между файлами; `experience_lineage` (из какого случая выведен опыт); маршрутизация «свой / чужой» (`peer`) с проверкой, что `peer_id` действительно был в сессии ([OV:memory_isolation_handler.py#L70](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/openviking/session/memory/memory_isolation_handler.py#L70)) | [код] |
| Дубли | Извлекатель сначала читает и ищет существующие записи, потом правит их. Поля идентичности `immutable`; в описании поля прямо сказано, какие темы считать одной. Решает LLM | [код] |
| Забывание | Удаления по давности нет. «Горячесть» влияет только на ранжирование и по умолчанию выключена | [код] |
| Оценки полезности | `active_count` (сколько раз запись достали) + `updated_at` | [код] |

### Общая память команды через git

**Не подходит напрямую.** Память хранится на сервере по аккаунтам и пользователям. Встроенный git — это снимки для отката, а не обмен между людьми. **Но** сам формат (Markdown-файл на запись, стабильные имена, правила слияния по полям, оглавление папки как производный файл) хорошо ложится на git, если хранить его в репозитории. Это главный источник идей для 3b.

### Полезное для наших слоёв

| Слой | Что взять |
|---|---|
| Карта кода | L0 файла кода = скелет tree-sitter без LLM; L0/L1 папок — из L0 детей; пересчёт родителя, только если L0 ребёнка изменился |
| Последствия правок | — |
| Журнал работы (3a) | Вырезание собственных инъекций; отдельная сессия памяти для субагента; вынос больших выводов инструментов в отдельное хранилище со ссылкой (порог 20 000 символов) |
| Знания проекта (3b) | YAML-схема типа записи: папка + шаблон имени файла + шаблон Markdown + `merge_op` для каждого поля; производное оглавление папки; идентичность проекта по `origin` и файл настроек в репозитории |
| Документы | Единый адрес (`viking://…`) для кода, документов и памяти; поиск со спуском по папкам; уровни L0/L1/L2 для Markdown и PDF |
| Проверка | «Случай» с рубрикой и доказательствами (`cases`) — образец того, как записывать проверяемые выводы |

### Можно ли взять код

- **Нет.** Основной проект под AGPL-3.0: встраивание в наш плагин обязало бы открыть его под AGPL. Под Apache-2.0 только CLI, RAGFS и примеры. Плагин Claude Code лежит в `examples/` (Apache-2.0), но это тонкий клиент к их серверу.
- Как движок тоже не подходит: Python-сервер, обязательные модель эмбеддингов и LLM, свой векторный движок на C++, «Alpha» в classifiers.

### Слабые места

- Тяжёлая установка и обязательные внешние модели.
- Подмешивание памяти на каждый запрос стоит токенов (по умолчанию включено).
- Память рассчитана на «пользователя и его собеседников», а не на команду разработчиков.
- `<private>` не поддержан.
- Иерархическое смешивание оценок по умолчанию выключено: реклама «поиска по папкам» — это в основном «поиск внутри указанной папки».

### Бенчмарки

LoCoMo (память длинной **переписки**): 80–83% против 24–57% у встроенной памяти, в том числе Claude Code 57.21% → 80.32%. tau2-bench (многошаговые задачи в ритейле и авиабилетах): +6.9 и +11.9 п.п. Измеряли сами авторы на своих моделях Doubao ([OV:README.md](https://github.com/volcengine/OpenViking/blob/af11ccb1d7fc9b65f6d77c966e65953c0ed6282b/README.md)) [заявлено]. К памяти о коде это прямого отношения не имеет.

### Вердикт

**Код — не брать (AGPL). Идеи — взять**, и это самые ценные идеи из трёх систем: уровни L0/L1/L2 с дешёвым L0 для кода, поиск в пределах папки, YAML-схема записей со стабильными именами и правилами слияния полей.

---

## Ответ на главный вопрос

### Можно ли встроить agentmemory или claude-mem как готовый движок журнала работы (3a)?

**Нет, ни ту, ни другую.**

| Требование к 3a | agentmemory | claude-mem |
|---|---|---|
| Ставится вместе с плагином, без ручных шагов | Нет: нужен iii-engine (на Windows — вручную) | Частично: установщик сам ставит Bun и uv, но через bash-хуки |
| Работает под Windows | Только через ручной `iii.exe`, Docker или WSL2 | Заявлено, нужен Git Bash |
| Без лишних фоновых процессов | Демон движка + сервер, 4 порта | Worker + Chroma + подпроцессы Claude |
| Без затрат токенов по умолчанию | Да, но тогда почти без «умных» функций | Нет: наблюдатель тратит подписку |
| Лицензия, пригодная для встраивания | Apache-2.0, но движок ELv2 | Apache-2.0 (open-core) |
| Отделяемость ядра | Нет: всё на iii SDK | Нет: всё на `bun:sqlite` и worker |
| Приватность | `<private>` + секреты, но импорт обходит фильтр | `<private>`, секретов нет |

**Рекомендация.** Написать журнал работы самим на Node, без отдельного демона:
- хук пишет событие в локальное хранилище (SQLite или JSONL в `~/.<наш-плагин>/`), не блокируя Claude Code;
- «сжатие» делается в конце сессии или лениво, с явной настройкой, какой моделью и за чьи токены;
- выдача по образцу claude-mem: оглавление с ID + поиск → хронология → подробности;
- взять код: регулярки секретов и парсер JSONL из agentmemory (Apache-2.0), `tag-stripping` из claude-mem (Apache-2.0), с указанием авторства.

### Что пригодно для знаний проекта (3b)?

Готового решения нет ни у кого. Складываем из идей:
1. **Формат записи — по OpenViking.** Один факт или решение = один Markdown-файл. Стабильное имя из типа и темы (`decisions/<slug>.md`). YAML-frontmatter с полями и правилом слияния для каждого поля. Идентичность задаётся полями `immutable`.
2. **Версии — по agentmemory, но в git.** Поля `status: active|superseded`, `supersedes: [id]`, `superseded_by`. Устаревшее не удаляется, но выпадает из выдачи. Перед созданием новой записи — проверка похожих (`similarTo`), лучше по смыслу, а не по словам.
3. **Источник — по agentmemory и claude-mem.** `sources:` со ссылками на наблюдения журнала (локальные ID для другого человека бесполезны, поэтому нужны ещё коммит, файлы и дата), автор, `modified` (как в auto memory Claude Code).
4. **Отбор из журнала — по claude-mem.** Кандидаты — наблюдения типов `decision`, `gotcha`, `trade-off`, `problem-solution` и поля сводки `learned`. Перенос только явным действием (как «forked memory» у razzant в отчёте 03).
5. **Слияние.** Оглавления (как `.overview.md` у OpenViking или `MEMORY.md`) — производные файлы. Их не коммитим или пересобираем после `git pull`, чтобы не было конфликтов. Сами записи — маленькие отдельные файлы: конфликт возможен только при правке одной записи двумя людьми.

---

## Сводная таблица приёмов

| # | Приём | Откуда | Для какого слоя | Проверено | Что делаем |
|---|---|---|---|---|---|
| 1 | Хук «отправил и вышел» (≤ 500 мс), без ожидания ответа | agentmemory | 3a | [код] | взять идею |
| 2 | Защита от рекурсии, когда сжатие делает дочерний Claude | agentmemory | 3a | [код] | взять идею |
| 3 | Регулярки секретов + `<private>` | agentmemory | 3a | [код] | взять код (Apache-2.0) |
| 4 | Вырезание своих инъекций и `system-reminder` перед записью | claude-mem, OpenViking | 3a | [код] | взять идею |
| 5 | «Весь ход приватный → наблюдений нет» | claude-mem | 3a | [код] | взять идею |
| 6 | Двойная запись: сырой вызов инструмента + сжатое наблюдение | claude-mem | 3a | [код] | взять идею |
| 7 | Схема наблюдения: тип + понятия + факты + файлы | claude-mem | 3a → 3b | [код] | взять идею |
| 8 | Промпт наблюдателя «что узнали, а не что делали; пропускай рутину» | claude-mem | 3a | [код] | взять идею |
| 9 | Оглавление с ID на старте, подробности по запросу (`search → timeline → get → raw`) | claude-mem | 3a, 3b | [код] | взять идею |
| 10 | Контекст к файлу при чтении, только если файл не менялся после наблюдения | claude-mem | 3a, карта | [код] | взять идею |
| 11 | Тегированный автоблок в Markdown, заменяемый целиком | claude-mem | документы, `CLAUDE.md` | [код] | взять идею / мелкий код |
| 12 | Цепочка версий `supersedes / isLatest`, устаревшее вне поиска, но в истории | agentmemory | 3b | [код] | взять идею |
| 13 | Подсказка `similarTo` при сохранении | agentmemory | 3b | [код] | взять идею (со сходством по смыслу) |
| 14 | `sourceObservationIds` + `verify` («из чего выведено») | agentmemory | 3b, проверка | [код] | взять идею |
| 15 | Импорт JSONL-логов Claude Code | agentmemory | 3a | [код] | взять код + дописать фильтр и стабильные ID |
| 16 | `confidence` с усилением при подтверждении и затуханием | agentmemory | 3b | [код] | обсудить |
| 17 | Уровни L0/L1/L2; L0 кода — скелет tree-sitter без LLM | OpenViking | карта, документы | [код] | взять идею |
| 18 | Поиск со спуском по папкам / в пределах папки | OpenViking | карта, документы | [код] | взять идею |
| 19 | Пересчёт родителя, только если L0 ребёнка изменился | OpenViking | карта | [заявлено, черновик] | взять идею |
| 20 | YAML-схема записи: папка + шаблон имени + `merge_op` по полям | OpenViking | 3b | [код] | взять идею |
| 21 | Оглавление папки как производный файл | OpenViking | 3b | [код] | взять идею, не коммитить |
| 22 | Идентичность проекта по URL `origin`, файл настроек в репозитории | OpenViking | все | [заявлено] | взять идею |
| 23 | Отдельная сессия памяти для субагента | OpenViking | 3a | [заявлено] | обсудить |
| 24 | Вынос больших выводов инструментов в отдельное хранилище со ссылкой | OpenViking | 3a | [заявлено] | взять идею |
| 25 | Мост в `MEMORY.md` Claude Code с перезаписью | agentmemory | — | [код] | **не делать** |
| 26 | `CLAUDE.md` по папкам с личными ID в git | claude-mem | — | [код] | **не делать** |
| 27 | Подмешивание памяти на каждый запрос / вызов инструмента по умолчанию | agentmemory (выключили), OpenViking | — | [код] | **не делать по умолчанию** |

---

## Что из этого следует для нашей архитектуры

1. **3a пишем сами**, лёгкий и без демона. Код берём точечно из agentmemory и claude-mem (Apache-2.0, с NOTICE). Настройка «какой моделью сжимать и за чьи токены» явная. По умолчанию сжатие дешёвое или отложенное.
2. **3b проектируем с нуля** на формате «один файл на запись». Формат по мотивам схем OpenViking, версии по мотивам agentmemory. Индексы — производные и не коммитятся.
3. **Идентичность проекта** берём по нормализованному `origin`, а не по имени папки.
4. **Со встроенной памятью Claude Code не спорим.** Её файлы не трогаем. Правила команды, привязанные к путям, выгодно отдавать через `.claude/rules/*.md` с `paths:`, которые Claude Code загрузит сам.
5. **Бенчмарки памяти переписки для выбора не используем.** Для своей системы нужен свой маленький набор задач «про код»: найти прошлое решение, не повторить отвергнутый подход, заметить устаревшее знание.
