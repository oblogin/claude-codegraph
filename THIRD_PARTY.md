# Заимствованный код

Каждое заимствование чужого кода (и кода из нашей `C:/ue-xray`) — строкой здесь: проект, файл, коммит, лицензия, куда взято и как (правило из `CLAUDE.md`, ADR-004, ADR-006 §13). Зависимости с crates.io сюда не пишутся — они видны в `Cargo.toml` и `Cargo.lock`.

## ue-xray

Проект `C:/ue-xray` (наш, лицензия MIT), коммит `372c0c874f9f4dec18a33a44739665bccc51d4be` (2026-06-24).

| Файл в ue-xray | Что взято | Куда | Как |
|---|---|---|---|
| `Cargo.toml` | Раскладка воркспейса: `[workspace.package]`, `[workspace.dependencies]`, версии `rmcp` 1 (`server`, `transport-io`), `tokio` (`rt`, `macros`, `io-std`), `clap` 4 (`derive`), `serde`, `serde_json`, `anyhow`; `rust-version = "1.93"` | `Cargo.toml` | образец с правками: свои крейты, edition 2024, `publish = false` |
| `rust-toolchain.toml` | Закреплённый канал с `clippy`, `rustfmt`, `profile = "minimal"` и причина закрепления | `rust-toolchain.toml` | образец, канал `1.98.1` |
| `crates/xray-mcp/src/server.rs` (`get_info`, `run_stdio`) | Каркас сервера `rmcp`: `#[tool_handler] impl ServerHandler`, `ServerInfo` с `instructions`, запуск `serve(stdio())` и `waiting()` | `crates/mcp/src/server.rs` | код с правками: свой набор инструментов, без кэша БД и профилей |
| `crates/xray-mcp/src/tools.rs` (`#[tool_router]`, `#[tool(... annotations(read_only_hint, open_world_hint))]`, параметры через `Parameters<T>` и `schemars::JsonSchema`) | Объявление инструментов и аннотаций MCP | `crates/mcp/src/server.rs`, `crates/mcp/src/params.rs` | образец; инструменты и параметры — по ADR-003 |
| `crates/xray-mcp/src/server.rs` (тесты `every_tool_carries_correct_annotations`, `doc_lint_tests`) | Идея тестов: у каждого инструмента явные аннотации; поверхность для агента — по-английски | `crates/mcp/tests/server.rs` | идея, тесты написаны заново под ADR-003 |
