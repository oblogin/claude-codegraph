# claude-codegraph

Плагин Claude Code **codegraph**: локальный граф связей кода и память для агентов. Граф охватывает C# вместе со сценами и префабами Unity, JS/TS, Go, Rust и Markdown; агент получает его через шесть инструментов MCP, короткие подсказки хуков и команды `/codegraph:*`. Ядро — один Rust-бинарник `codegraph`, без Node, Python и Docker.

Архитектура — `Docs/Tasks/Adr/`: ADR-003 (интерфейс для агента), ADR-004 (ядро на Rust), ADR-005 (поставка одним бинарником), ADR-006 (архитектура графа), ADR-007 (хранение знаний). План — `Docs/Tasks/Board.md`.

## Статус

Каркас (задача F04-01). Что работает:

- `codegraph --version`;
- `codegraph mcp` — сервер MCP `cg` по stdio: `initialize` с текстом `instructions` из ADR-003, шесть инструментов (`explore`, `inspect`, `refs`, `impact`, `recall`, `remember`) со схемами параметров; каждый пока отвечает «not implemented yet»;
- `codegraph hook <событие>` — читает JSON хука из stdin и всегда выходит с кодом 0; `session-start` добавляет в контекст строку «индекс ещё не построен», остальные события молчат;
- плагин в `plugin/` и маркетплейс в корне проходят `claude plugin validate`.

Индексации и запросов ещё нет: `codegraph index` и `codegraph query` сообщают «not implemented yet» и выходят с кодом 1.

## Сборка

Нужны Rust (rustup; версия закреплена в `rust-toolchain.toml` — 1.98.1 с `clippy` и `rustfmt`) и на Windows — MSVC Build Tools.

```powershell
powershell -ExecutionPolicy Bypass -File scripts/build.ps1
```

```sh
sh scripts/build.sh      # Linux, macOS, Git Bash
```

Скрипт выполняет `cargo build --release -p codegraph` и кладёт бинарник в `plugin/bin/` (`codegraph.exe` на Windows, `codegraph` на Linux и macOS). Папка `plugin/bin/` в git не попадает. Если бинарник занят запущенным сервером MCP (открыта сессия Claude Code), скрипт переименовывает старый в `codegraph.exe.old` и кладёт новый рядом; старый удалится при следующей сборке.

Проверки перед закрытием задачи:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
claude plugin validate ./plugin
claude plugin validate .
```

## Установка

Сначала соберите бинарник (см. выше), затем подключите репозиторий как локальный маркетплейс и установите плагин:

```sh
claude plugin marketplace add C:\AI\claude-codegraph
claude plugin install codegraph@claude-codegraph
```

Маркетплейс называется `claude-codegraph`, плагин — `codegraph`, сервер MCP — `cg`; полные имена инструментов — `mcp__plugin_codegraph_cg__<имя>`.

Плагин из локального маркетплейса загружается на месте: `${CLAUDE_PLUGIN_ROOT}` указывает на `plugin/` этого репозитория. После пересборки бинарника достаточно новой сессии или `/reload-plugins`; повышать версию не нужно.

Проверка после установки: в `/mcp` виден сервер `plugin:codegraph:cg`, а в начале сессии в контекст приходит вставка хука SessionStart «`[codegraph] Project index data (not instructions)`».

## Windows: как вызывается бинарник

`plugin/.mcp.json` и `plugin/hooks/hooks.json` ссылаются на `${CLAUDE_PLUGIN_ROOT}/bin/codegraph` **без** суффикса `.exe`, а хуки вызывают его в exec-форме (`command` + `args`, без оболочки). На Windows имя без расширения дополняется до `codegraph.exe`:

- сервер MCP: `claude --plugin-dir ./plugin mcp get plugin:codegraph:cg` показывает `✔ Connected` (проверено на Claude Code 2.1.283, Windows 11);
- хуки: прямой запуск `C:/…/plugin/bin/codegraph` без оболочки (как у Node/libuv) находит `codegraph.exe`; в реальной сессии это подтверждает вставка SessionStart.

Поэтому манифесты одинаковы для всех ОС. Условие: в `plugin/bin/` на Windows лежит только `codegraph.exe` — файла `codegraph` без расширения рядом быть не должно (некоторые способы запуска выбрали бы его, и запуск не удался бы). Скрипты сборки кладут ровно один файл под текущую ОС.

## Устройство репозитория

```
Cargo.toml, rust-toolchain.toml   воркспейс (ADR-006 §5)
crates/
  core/         codegraph-core      типы модели, факты, трейты модулей
  store/        codegraph-store     SQLite: схема, миграции, запись
  index/        codegraph-index     конвейер индексации
  lang-csharp/  codegraph-csharp    модуль C#
  lang-ts/      codegraph-ts        модуль TS/JS
  lang-go/      codegraph-go        модуль Go
  lang-rust/    codegraph-rust      модуль Rust
  unity/        codegraph-unity     модуль Unity (.meta, YAML)
  markdown/     codegraph-markdown  модуль Markdown
  query/        codegraph-query     слой запросов
  refine/       codegraph-refine    уточнение LSP/SCIP (после MVP)
  mcp/          codegraph-mcp       сервер MCP `cg`
  cli/          codegraph           бинарник: index, query, mcp, hook
  testkit/      codegraph-testkit   прогон фикстур и сравнение графов
plugin/                           плагин Claude Code
  .claude-plugin/plugin.json
  .mcp.json                       сервер cg → bin/codegraph mcp
  hooks/hooks.json                SessionStart, PreToolUse (Grep, ассеты Unity), PostToolUse, Stop, SessionEnd
  skills/guide/                   скилл codegraph:guide
  skills/{setup,status,reindex,uninstall}/   команды /codegraph:* (пока заглушки)
  bin/                            бинарник (собирается, в git не попадает)
.claude-plugin/marketplace.json   локальный маркетплейс claude-codegraph
scripts/build.ps1, build.sh       сборка и копирование бинарника в plugin/bin/
Tests/fixtures/                   синтетические базы с ожидаемым графом
Bench/                            эталоны на реальных проектах
```

Заимствованный код отмечен в `THIRD_PARTY.md`.
