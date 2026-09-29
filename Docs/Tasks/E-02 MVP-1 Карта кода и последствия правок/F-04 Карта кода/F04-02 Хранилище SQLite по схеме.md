---
id: F04-02
type: task
role: developer
status: in-progress
priority: must
size: M
parent: "[[F-04 Карта кода]]"
depends_on:
  - "[[F04-01 Каркас воркспейса и плагина]]"
created: 2026-09-29
---
# F04-02 Хранилище SQLite по схеме

> **Что:** крейт хранилища: DDL из ADR-006, версии схемы, один писатель с замком, upsert узлов по ключу, удаление фактов файла.
> **Зачем:** граф должен пережить переиндексацию без потерянных строк и одноимённых перезаписей (урок ue-xray).
> **Готово, когда:** тесты хранилища зелёные, база создаётся в `.codegraph/index.db`.

## Критерии готовности

- [ ] Таблицы и индексы ровно по DDL ADR-006, версия схемы в `meta`
- [ ] Upsert узлов `ON CONFLICT(key) DO UPDATE`, `INSERT OR REPLACE` нигде нет
- [ ] Замок `.codegraph/index.lock` + `BEGIN IMMEDIATE`; параллельные читатели работают
- [ ] FTS по узлам; тест поиска
- [ ] Тест: переиндексация файла не оставляет сирот и не ломает внешние ключи

## Детали

### План реализации

**Исполнитель:** pipeline:developer (sonnet) · **Цепочка:** новая · **Почему этот исполнитель:** логика прямолинейная, всё нетривиальное (порядок удаления, FTS, замок) расписано ниже; ревью — `pipeline:reviewer` с `model: opus` (хранилище — ядро).

**Цель одним абзацем.** Крейт `codegraph-store` открывает или создаёт `.codegraph/index.db` в корне проекта со схемой ровно по ADR-006 §4, ведёт версию схемы, даёт одного писателя (файл-замок + `BEGIN IMMEDIATE`) и параллельных читателей, умеет заменять факты одного файла без сирот и без перезаписи одноимённых узлов, поддерживает полнотекстовый поиск по узлам и кэш фактов. Всё покрыто тестами `cargo test -p codegraph-store`.

**Контекст.**
- Каркас: `crates/store/src/lib.rs` — пока только doc-комментарий; `crates/store/Cargo.toml` зависит от `codegraph-core`. `crates/core/src/lib.rs` — `Key`, `Hash128 = [u8;16]`, `Pos`, `Span`.
- ADR-006 §4 (главный источник, перечитай целиком строки 168–301 файла `Docs/Tasks/Adr/ADR-006 Архитектура графа.md`). Правила, которые нельзя нарушить:
  - один файл `.codegraph/index.db`; прагмы на **каждом** открытом соединении: `journal_mode=WAL`, `foreign_keys=ON`, `synchronous=NORMAL`, `busy_timeout=5000`, `journal_size_limit=33554432`;
  - узлы обновляются только `INSERT … ON CONFLICT(key) DO UPDATE` — rowid узла стабилен, пока жив ключ; `INSERT OR REPLACE` запрещён везде;
  - факты и рёбра файла — «удалить по `file_id` + вставить»;
  - писатель один: файл-замок `.codegraph/index.lock` + `BEGIN IMMEDIATE`; читатели работают параллельно;
  - FTS `nodes_fts` — contentless (`content=''`, `contentless_delete=1`), rowid = `nodes.id`, ведётся **кодом** в той же транзакции (триггеров нет);
  - инварианты: у не-`external` узла ≥ 1 строки `decls`; `external`-узел без входящих рёбер удаляется; у каждой `refs` после разрешения ровно одно из «ребро с этим `ref_id`» / «строка `unresolved`».
- ADR-006 §1 (виды узлов/рёбер, слои) и §2 (ключи, флаги) — для enum-ов в core.
- Образец работы с SQLite: `C:/ue-xray/crates/xray-core/src/db.rs` строки ~395–425 (`init`: прагмы и комментарии, почему `busy_timeout` и `journal_size_limit`). Только читать; взятое — строкой в `THIRD_PARTY.md` (раздел ue-xray, коммит `372c0c874f9f4dec18a33a44739665bccc51d4be`).
- Образец стиля кода и тестов: `crates/mcp/src/server.rs`, `crates/mcp/tests/server.rs`, `crates/cli/tests/binary.rs`.
- Зависимости: добавь в `[workspace.dependencies]` корневого `Cargo.toml` `rusqlite = { version = "<последняя 0.x, которую резолвит cargo>", features = ["bundled"] }` и `thiserror = "2"`, в `crates/store/Cargo.toml` — `rusqlite.workspace = true`, `thiserror.workspace = true`, `serde_json.workspace = true`; dev-dependency `tempfile.workspace = true`. Проверь фактическую версию `rusqlite` в `Cargo.lock` и сигнатуры по docs.rs **этой** версии (`Connection::open_with_flags`, `Transaction`, `TransactionBehavior::Immediate`, `OptionalExtension::optional`). Замок — стандартный `std::fs::File::try_lock()` (стабилен с Rust 1.89; MSRV воркспейса 1.93) — отдельный крейт не нужен.

**Подход.** Один публичный тип `Store` (соединение + корень проекта) и `Writer<'a>` — транзакция + удерживаемый файл-замок; всё, что пишет, — только через `Writer`. Схема — константа SQL ровно из ADR, миграции — массив `(версия, SQL)`. Строки для записи — простые структуры `…Record` в `store`, без зависимости от будущих `FileFacts` (конвертацию сделает F04-03). Отвергнуто: ORM (лишняя зависимость, скрывает SQL); триггеры для FTS (ADR прямо требует вести кодом; contentless-таблица с триггерами на UPDATE ведёт себя неочевидно); `fs2`/`fd-lock` (есть в std).

Как части связаны: `Store::open` → прагмы → `migrate` → `meta`. `Store::writer()` → берёт замок (с тайм-аутом) → `BEGIN IMMEDIATE` → `Writer`. `Writer::replace_file(...)` делает всю замену факта файла атомарно внутри этой транзакции; `Writer::commit()` увеличивает `meta.generation`, коммитит и отпускает замок. `Drop` у `Writer` без `commit` → откат + отпуск замка.

**Шаги.** После каждого шага: `cargo build -p codegraph-store` и `cargo test -p codegraph-store` зелёные.

1. **`crates/core/src/model.rs`** (новый) + `pub mod model; pub use model::*;` в `crates/core/src/lib.rs`.
   ```rust
   #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
   pub enum Layer { Fs = 0, Code = 1, Unity = 2, Docs = 3, Memory = 4 }
   impl Layer { pub fn as_i64(self) -> i64; pub fn from_i64(v: i64) -> Option<Self>; }

   #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
   pub enum NodeKind { File, Unit, Namespace, Module, Class, Struct, Interface, Trait, Enum, Delegate, TypeAlias,
       Function, Method, Constructor, Macro, Field, Property, Event, Constant, EnumMember, Variable,
       Asset, GameObject, Component, PrefabInstance, Document, Section, Note }
   impl NodeKind { pub fn as_str(self) -> &'static str; /* snake_case как в ADR §1: "type_alias", "game_object", "prefab_instance", "enum_member" */
                   pub fn layer(self) -> Layer; }
   impl std::str::FromStr for NodeKind { type Err = UnknownName; }

   pub enum EdgeKind { Contains, Imports, DependsOn, Calls, Instantiates, References, UsesType, Reads, Writes,
       Inherits, Implements, Overrides, Raises, Handles, RunsScript, InstanceOf, VariantOf, RemovesComponent,
       UnityRef, UnityEvent, AnimEvent, Serializes, ReferencesType, LinksTo, Mentions, CoChange, About }
   // as_str: "contains","imports","depends_on",...,"unity_ref","co_change","about" — ровно имена из таблицы ADR §1; FromStr.

   pub enum Source { TreeSitter, UnityYaml, Config, Heuristic, Git, Lsp, Scip, Model }
   // as_str: "treesitter","unity-yaml","config","heuristic","git","lsp","scip","model"; FromStr.

   pub enum ConfClass { Extracted = 0, Inferred = 1, Ambiguous = 2 }   // as_i64 / from_i64

   #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
   pub struct NodeFlags(pub u32);
   impl NodeFlags {
       pub const EXTERNAL: u32 = 1 << 0; pub const EXPORTED: u32 = 1 << 1; pub const STATIC: u32 = 1 << 2;
       pub const ABSTRACT: u32 = 1 << 3; pub const PARTIAL: u32 = 1 << 4; pub const TEST: u32 = 1 << 5;
       pub const ENTRY: u32 = 1 << 6; pub const GENERATED: u32 = 1 << 7;
       pub fn has(self, bit: u32) -> bool;
   }
   #[derive(Debug, Clone, PartialEq, Eq)] pub struct UnknownName(pub String);  // + Display + Error
   ```
   - Тест в core: для каждого варианта `NodeKind`/`EdgeKind`/`Source` — `as_str` → `from_str` возвращает тот же вариант (цикл по массиву всех вариантов `ALL: &[Self]`, объяви его константой).
   - Не делать: факты, трейты модулей, `Resolver` — это F04-03.

2. **`crates/store/src/schema.rs`** — `pub(crate) const SCHEMA_V1: &str` = DDL из ADR-006 §4 **дословно** (все `CREATE TABLE/INDEX/VIRTUAL TABLE`, без `PRAGMA` — прагмы отдельно). `pub(crate) const MIGRATIONS: &[(u32, &str)] = &[(1, SCHEMA_V1)]; pub const SCHEMA_VERSION: u32 = 1; pub const KEY_SCHEME: &str = "1"; pub const HASH_SCHEME: &str = "xxh3-128/1";`

3. **`crates/store/src/error.rs`**:
   ```rust
   #[derive(Debug, thiserror::Error)]
   pub enum StoreError {
       #[error("sqlite: {0}")] Sqlite(#[from] rusqlite::Error),
       #[error("io: {0}")] Io(#[from] std::io::Error),
       #[error("index schema v{found} is newer than supported v{supported}; update codegraph")] NewerSchema { found: u32, supported: u32 },
       #[error("index is locked by another writer ({0})")] Locked(String),     // путь к замку
       #[error("index not built at {0}")] NotIndexed(String),
       #[error("node not found: {0}")] MissingNode(String),                   // ключ
       #[error("bad data: {0}")] BadData(String),
   }
   pub type Result<T> = std::result::Result<T, StoreError>;
   ```

4. **`crates/store/src/lib.rs`** — `Store`:
   ```rust
   pub struct Store { conn: rusqlite::Connection, root: PathBuf, dir: PathBuf }
   impl Store {
       pub fn open(project_root: &Path) -> Result<Store>;            // создаёт .codegraph/ и базу, применяет миграции
       pub fn open_read_only(project_root: &Path) -> Result<Store>;  // нет базы → NotIndexed; миграции НЕ применяет; более новая схема → NewerSchema
       pub fn db_path(project_root: &Path) -> PathBuf;               // <root>/.codegraph/index.db
       pub fn schema_version(&self) -> Result<u32>;
       pub fn meta(&self, k: &str) -> Result<Option<String>>;
       pub fn generation(&self) -> Result<u64>;
       pub fn writer(&mut self) -> Result<Writer<'_>>;                             // тайм-аут замка 5 с
       pub fn writer_with_timeout(&mut self, t: Duration) -> Result<Writer<'_>>;
       // чтение
       pub fn node(&self, key: &str) -> Result<Option<NodeRow>>;
       pub fn node_id(&self, key: &str) -> Result<Option<i64>>;
       pub fn file(&self, path: &str) -> Result<Option<FileRow>>;
       pub fn decls_of(&self, key: &str) -> Result<Vec<DeclRow>>;
       pub fn edges_to(&self, key: &str) -> Result<Vec<EdgeRow>>;    // входящие, dst = key
       pub fn edges_from(&self, key: &str) -> Result<Vec<EdgeRow>>;
       pub fn search(&self, query: &str, limit: usize) -> Result<Vec<NodeRow>>;  // FTS, см. шаг 7
       pub fn fact_cache_get(&self, k: &FactCacheKey) -> Result<Option<Vec<u8>>>;
       pub fn check_invariants(&self, after_resolution: bool) -> Result<Vec<Violation>>;
   }
   ```
   Логика `open`:
   1. `create_dir_all(root/.codegraph)`; `Connection::open(db)`.
   2. `apply_pragmas(&conn)` — одной `execute_batch` пять прагм из ADR (для read-only соединения `journal_mode` не меняй — только чтение, остальные четыре ставь).
   3. `migrate(&mut conn)`: `CREATE TABLE IF NOT EXISTS meta(...)` нельзя — `meta` входит в схему. Поэтому: проверь `SELECT count(*) FROM sqlite_master WHERE type='table' AND name='meta'`; нет → версия 0; есть → `SELECT v FROM meta WHERE k='schema_version'` → парсинг в u32 (ошибка парсинга → `BadData`). Если `found > SCHEMA_VERSION` → `NewerSchema`. Для каждой миграции с версией > found — в **одной** транзакции: `execute_batch(sql)`, затем upsert в `meta` ключей `schema_version`, `key_scheme`, `hash_scheme`, `tool_version` (= `env!("CARGO_PKG_VERSION")`), `root` (канонический путь корня с `/`), `generation` (если нет — `"0"`). Ошибка любой миграции → вернуть ошибку (транзакция откатится), не глотать.
   4. `meta` upsert — `INSERT INTO meta(k,v) VALUES(?1,?2) ON CONFLICT(k) DO UPDATE SET v=excluded.v`.
   Логика `writer_with_timeout`: `File::options().create(true).write(true).open(dir/index.lock)`; цикл `try_lock()`: `Ok` → дальше; `Err(TryLockError::WouldBlock)` → `sleep(25ms)` до дедлайна, потом `Locked(path)`; другая ошибка → `Io`. Затем `self.conn.transaction_with_behavior(TransactionBehavior::Immediate)`. `Writer { tx, _lock: File }`.

5. **`crates/store/src/rows.rs`** — записи для записи и строки для чтения:
   ```rust
   pub struct FileRecord { pub path: String, pub module: String, pub unit: Option<String>, pub size: u64, pub mtime_ns: i64,
       pub content_hash: Hash128, pub module_ver: u32, pub ctx_hash: Hash128, pub iface_hash: Option<Hash128>, pub status: u8 }
   pub struct NodeRecord { pub key: Key, pub kind: NodeKind, pub name: String, pub qname: Option<String>, pub parent: Option<Key>,
       pub span: Option<Span>, pub sig: Option<String>, pub sig_hash: Option<Hash128>, pub body_hash: Option<Hash128>,
       pub flags: NodeFlags, pub props: Option<serde_json::Value>, pub fts_path: Option<String>, pub fts_text: Option<String> }
   pub struct DeclRecord { pub node: NodeRecord, pub span: Span, pub body_hash: Option<Hash128> }  // место объявления в этом файле
   pub struct RefRecord { pub from: Key, pub kind: String, pub name: String, pub ctx: Option<serde_json::Value>, pub at: Pos }
   pub struct EdgeRecord { pub src: Key, pub dst: Key, pub kind: EdgeKind, pub source: Source, pub conf_class: ConfClass,
       pub conf: f32, pub ref_id: Option<i64>, pub origin_file: Option<String>, pub at: Option<Pos>, pub props: Option<serde_json::Value> }
   pub struct FactCacheKey { pub content_hash: Hash128, pub module: String, pub module_ver: u32, pub ctx_hash: Hash128 }
   // строки чтения: FileRow, NodeRow (с id: i64, key, kind: NodeKind, layer, name, qname, parent_key: Option<String>, file_path: Option<String>, span, flags, props), DeclRow, EdgeRow (src_key, dst_key, kind, source, conf_class, conf, line, col), Violation { kind: &'static str, detail: String }
   ```
   `layer` узла берётся из `NodeKind::layer()` — отдельно не передаётся.

6. **`crates/store/src/writer.rs`** — `Writer<'a>`:
   ```rust
   pub struct ReplaceStats { pub nodes_upserted: usize, pub nodes_removed: usize, pub refs: usize }
   impl<'a> Writer<'a> {
       pub fn replace_file(&mut self, file: &FileRecord, decls: &[DeclRecord], refs: &[RefRecord]) -> Result<(i64, ReplaceStats)>; // → file_id
       pub fn delete_file(&mut self, path: &str) -> Result<usize>;           // → число удалённых узлов
       pub fn upsert_external(&mut self, node: &NodeRecord) -> Result<i64>;  // узел-заглушка без decls, флаг EXTERNAL обязателен (иначе BadData)
       pub fn insert_edge(&mut self, e: &EdgeRecord) -> Result<i64>;         // src/dst по ключам; нет узла → MissingNode
       pub fn set_unresolved(&mut self, ref_id: i64, reason: &str, candidates: Option<&serde_json::Value>) -> Result<()>; // ON CONFLICT(ref_id) DO UPDATE SET reason, candidates, attempts = attempts+1
       pub fn gc_external(&mut self) -> Result<usize>;                      // external без входящих рёбер
       pub fn fact_cache_put(&mut self, k: &FactCacheKey, facts: &[u8]) -> Result<()>;  // ON CONFLICT DO UPDATE SET facts, last_used
       pub fn set_meta(&mut self, k: &str, v: &str) -> Result<()>;
       pub fn commit(self) -> Result<u64>;   // generation += 1 в meta, commit, возвращает новую generation; замок отпускается при drop File
   }
   ```
   **`replace_file` — порядок строго такой** (всё в текущей транзакции):
   1. `file_id` = `INSERT INTO files(...) VALUES(...) ON CONFLICT(path) DO UPDATE SET module=excluded.module, …, indexed_at=excluded.indexed_at RETURNING id`. `indexed_at` — секунды UNIX.
   2. `old_nodes` = `SELECT DISTINCT node_id FROM decls WHERE file_id=?` (запомнить в `Vec<i64>`).
   3. `DELETE FROM edges WHERE origin_file_id=?`; `DELETE FROM refs WHERE file_id=?` (каскадом уйдут `edges` с этим `ref_id`, `unresolved`, `ref_deps`); `DELETE FROM decls WHERE file_id=?`.
   4. Для каждой `DeclRecord` (в порядке входа) — `upsert_node` (ниже) с `parent_id = NULL`, затем `INSERT INTO decls(node_id,file_id,start_line,start_col,end_line,end_col,body_hash)`. Повтор одного ключа в `decls` этого файла (две ветки `#if`) — две строки `decls` с разными позициями; одинаковая позиция → первичный ключ конфликтует → `BadData` с ключом.
   5. Второй проход по `decls`: `parent` есть → найти id по ключу → `UPDATE nodes SET parent_id=? WHERE id=?`; родителя нет в базе → оставить NULL и увеличить счётчик предупреждений (вернуть в `ReplaceStats`, поле `missing_parents: usize` — добавь).
   6. Для каждой `RefRecord`: `src_id` по ключу `from` (нет → `MissingNode`), `INSERT INTO refs(file_id,src_id,kind,name,ctx,line,col)`.
   7. Сборка мусора: для каждого id из `old_nodes`, у которого теперь `NOT EXISTS (SELECT 1 FROM decls WHERE node_id=id)` и `flags & EXTERNAL = 0` → `delete_node(id)`.
   8. Для всех узлов, затронутых шагами 4 и 7 (новые + старые живые), пересчитать основное место: `UPDATE nodes SET file_id=d.file_id, start_line=…, end_col=… FROM (SELECT d.file_id, d.start_line, … FROM decls d JOIN files f ON f.id=d.file_id WHERE d.node_id=nodes.id ORDER BY f.path, d.start_line LIMIT 1) d WHERE nodes.id=?` — если синтаксис `UPDATE … FROM` неудобен, сделай `SELECT` первого места и затем `UPDATE`.
   **`upsert_node`** (приватный): `INSERT INTO nodes(key,kind,layer,name,qname,sig,sig_hash,body_hash,flags,props) VALUES(...) ON CONFLICT(key) DO UPDATE SET kind=excluded.kind, layer=excluded.layer, name=excluded.name, qname=excluded.qname, sig=excluded.sig, sig_hash=excluded.sig_hash, body_hash=excluded.body_hash, flags=excluded.flags, props=excluded.props RETURNING id`. Затем FTS: `DELETE FROM nodes_fts WHERE rowid=?id` и `INSERT INTO nodes_fts(rowid,name,words,qname,path,text) VALUES(?id, name, fts_words(name), qname, fts_path, fts_text)`.
   **`delete_node(id)`** (приватный): сначала `DELETE FROM nodes_fts WHERE rowid=?`, затем `DELETE FROM nodes WHERE id=?` (каскадом уходят `decls`, `edges`, `refs` c этим `src_id`; у детей `parent_id` станет NULL — это по ADR).
   **`delete_file(path)`**: найти `file_id` (нет → `Ok(0)`), шаги 2–3 и 7 как в `replace_file`, затем `DELETE FROM files WHERE id=?`.
   **`fts_words(name)`** (в `crates/store/src/fts.rs`, `pub fn`): разрезать по `_`, `-`, `.`, `:`, пробелам и по границам camelCase (`PlayerController` → `player controller`; `HTTPServer` → `http server`; `getHTTPResponse2` → `get http response2`; `snake_case_name` → `snake case name`), всё в нижний регистр, склеить пробелом.

7. **`search`**: пустой запрос → пустой результат. Иначе: каждое слово запроса экранировать как строку FTS (`"слово"`, внутренние `"` удваивать) и добавить `*` для префикса; искать `SELECT rowid FROM nodes_fts WHERE nodes_fts MATCH ?1 ORDER BY rank LIMIT ?2`, затем узлы по id. Запрос `PlayerController` и запрос `player controller` оба должны находить узел `PlayerController` (первый — по колонке `name`, второй — по `words`).

8. **`check_invariants(after_resolution)`**: нарушения: `node_without_decls` (не external, нет decls); `orphan_external` (external без входящих рёбер); при `after_resolution=true` — `ref_unaccounted` (ни ребра с `ref_id`, ни `unresolved`) и `ref_double` (и то, и другое); плюс результат `PRAGMA foreign_key_check` (каждая строка — `foreign_key`).

9. **`THIRD_PARTY.md`** — строка о прагмах и их обосновании из `xray-core/src/db.rs` (идея, код написан заново).

**Тесты.** Файл `crates/store/tests/store.rs`; вспомогательные функции построения записей — внутри файла. Каждый тест — во временной папке (`tempfile::tempdir()`).

| Тест | Что проверяет | Данные | Ожидание | Критерий |
|---|---|---|---|---|
| `open_creates_schema_and_meta` | создание базы | пустая папка | файл `.codegraph/index.db` есть; `schema_version()==1`; `meta` содержит `key_scheme`, `hash_scheme`, `tool_version`, `generation="0"`; `PRAGMA journal_mode` = `wal`; `PRAGMA foreign_keys` = 1; в `sqlite_master` есть все 9 таблиц и `nodes_fts` | 1 |
| `schema_matches_adr` | DDL дословно | — | для таблиц `files, nodes, decls, refs, ref_deps, edges, unresolved, fact_cache, meta` список колонок (`PRAGMA table_info`) совпадает со списком из ADR (захардкодь ожидаемые имена колонок в тесте); все 13 индексов из ADR существуют | 1 |
| `reopen_is_idempotent` | повторное открытие | открыть, закрыть, открыть | версия 1, данные на месте | 1 |
| `newer_schema_is_refused` | защита версии | вручную `UPDATE meta SET v='99' WHERE k='schema_version'` | `open` и `open_read_only` → `NewerSchema{found:99,supported:1}` | 1 |
| `read_only_without_index` | нет базы | пустая папка | `open_read_only` → `NotIndexed` | 3 |
| `upsert_keeps_rowid` | стабильность id | файл A с узлом `cs:G:Game.Player`, затем снова A с тем же ключом и новым `name`/`sig` | `node_id` одинаков, `name` обновился | 2 |
| `same_name_different_keys_coexist` | урок ue-xray | файл A: `cs:G:Game.Slime`, файл B: `cs:G:Enemies.Slime` (оба `name="Slime"`) | два узла, оба находятся `search("Slime")` | 2 |
| `replace_file_leaves_no_orphans` | нет сирот | A: узлы X, Y (Y — ребёнок X), ссылка из Y; затем A только с X | Y удалён, X с прежним id, ссылок Y нет; `check_invariants(false)` пусто; `PRAGMA foreign_key_check` пусто | 5 |
| `partial_node_lives_while_any_decl` | partial | узел N объявлен в A и B | replace A без N → N жив, `decls_of(N)` = 1 (B), основное место = B; `delete_file(B)` → N удалён | 5 |
| `node_main_location_is_first_by_path` | основное место | N в `b/Z.cs` и `a/Z.cs` | `node(N).file_path == "a/Z.cs"` | 5 |
| `edges_cascade_on_replace` | рёбра по ref_id и origin | A: X вызывает Y (ref + edge с `ref_id`, `origin_file=A`); replace A без ссылки | ребро исчезло; `edges_to(Y)` пусто | 5 |
| `edges_to_other_files_survive_body_change` | чужие рёбра | ребро из B на узел в A; replace A с тем же ключом узла | ребро B→A на месте (id узла стабилен) | 2, 5 |
| `external_node_rules` | заглушки | `upsert_external` без флага EXTERNAL → `BadData`; с флагом, без входящих рёбер → `gc_external()` удаляет 1 | — | 5 |
| `fts_search_by_name_and_words` | поиск | узлы `PlayerController`, `HTTPServer`, `snake_case_name` | `search("PlayerController")`, `search("player controller")`, `search("player")`, `search("http server")` находят нужные; `search("")` пусто | 4 |
| `fts_follows_rename_and_delete` | FTS не протухает | узел переименован (новый ключ) / удалён | старое имя не находится (проверка contentless_delete — Г12) | 4 |
| `fts_words_splits` | функция | таблица пар вход → выход из шага 6 | точные строки | 4 |
| `second_writer_is_locked` | замок | два `Store::open` одной базы; первый держит `writer()`; второй `writer_with_timeout(200ms)` | `Locked`; после `drop` первого — второй получает писателя | 3 |
| `reader_sees_committed_while_writer_open` | параллельные читатели | писатель закоммитил X, открыл новую транзакцию и пишет Y без commit; читатель (`open_read_only`) | читатель видит X, не видит Y, не блокируется дольше 1 с | 3 |
| `drop_without_commit_rolls_back` | откат | писатель пишет X, drop без commit | X нет; замок свободен | 3 |
| `generation_increments_on_commit` | `meta.generation` | два commit | 1, затем 2 | — |
| `fact_cache_roundtrip` | кэш | put по ключу K; get K; get с другим `module_ver` | байты равны; промах → `None` | — |
| `non_ascii_project_root` | Windows | корень `tempdir/проект игры` | open, запись, чтение работают | — |
| `duplicate_decl_position_is_bad_data` | ошибки не глотаются | два `DeclRecord` одного ключа с одинаковой позицией | `BadData` с ключом в тексте | — |
| `core_enums_roundtrip` (в core) | `as_str`/`FromStr` | все варианты | совпадение | — |

Существующие тесты воркспейса (28) должны остаться зелёными.

**Ловушки.**
- `rusqlite` без фичи `bundled` возьмёт системный SQLite (на Windows его нет, а где есть — может быть без FTS5 или старее 3.43, без `contentless_delete`). Только `bundled`. Проверь в тесте `SELECT sqlite_version()` ≥ 3.43 (строкой, сравни числа).
- `STRICT`-таблицы: хэши — `BLOB` → передавай `&[u8]` (`&hash[..]`), не `Vec`/строкой; целые — `i64`; `conf` — `REAL` (f64). Иначе «cannot store TEXT value in BLOB column».
- `foreign_keys` действует на соединение, а не на базу — ставить на **каждом** открытии, иначе каскады тихо не работают.
- Contentless FTS5: `UPDATE` запрещён; удалять **до** удаления узла (`DELETE … WHERE rowid=?`), иначе строка FTS останется навсегда.
- `INSERT … ON CONFLICT … DO UPDATE … RETURNING id` возвращает id и при обновлении — не используй `last_insert_rowid()` (при UPDATE он не меняется).
- Порядок в `replace_file`: сначала запомни `old_nodes`, потом удаляй `decls`; иначе не узнаешь, какие узлы проверять на сиротство.
- Каскад `nodes → refs(src_id)` удалит ссылки **других** файлов, если их объемлющий узел исчез — это правильно по ADR, не «чини» это.
- `Transaction` заимствует `&mut Connection`: `Writer` держит `Transaction<'a>`; `commit(self)` потребляет. Не храни соединение в `Arc<Mutex>` — не нужно.
- Замок: `try_lock` на Windows тоже работает через `LockFileEx`; не удаляй файл `index.lock` после отпуска (гонка при удалении) — пусть лежит.
- Пути в `files.path` — с `/`, относительно корня; сама функция их не нормализует — это обязанность вызывающего, но `debug_assert!(!path.contains('\\'))`.
- Не используй `unwrap()/expect()` в коде крейта (только в тестах).

**Крайние случаи.** Пустой список `decls`/`refs` — файл без символов (валидно: файл есть, узлов нет, старые удалены). Повторная замена тем же содержимым — те же id, `nodes_removed=0`. Ключ длиной > 1000 символов и не-ASCII в имени — сохраняются как есть. Удаление несуществующего файла — `Ok(0)`. Открытие, когда `.codegraph/` — файл, а не папка — ошибка `Io`, не паника.

**Вне рамок.** `SymbolIndex`/`SqlSymbolIndex`, разрешение, конвейер (`codegraph index`), кэш смежности, сериализация фактов (postcard/zstd) — `fact_cache` хранит непрозрачные байты; `VACUUM`/сжатие; команда `codegraph doctor` (только функция `check_invariants`); подключение хранилища к `cli`/`mcp` — это F04-03 и далее. Не трогай `crates/cli`, `crates/mcp`, `plugin/`.

**Готово, когда.**
- Все критерии задачи выполнены и закрыты тестами из таблицы.
- `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` — всё зелёное (PATH: `$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path`).
- `grep -rn "INSERT OR REPLACE" crates/` — пусто.
- `THIRD_PARTY.md` дополнен.

Архитектура — [[ADR-006 Архитектура графа]]; интерфейс — [[ADR-003 Интерфейс для агента]]; ядро и поставка — [[ADR-004 Своё ядро на Rust]], [[ADR-005 Поставка одним бинарником]].

## Проверка

1. `cargo test` крейта хранилища

## Журнал

- **2026-09-29 · lead (Claude):** задача создана при разбиении MVP-1 по ADR-003, ADR-004, ADR-006.
- **2026-09-29 · lead (Claude):** план реализации записан (конвейер pipeline): исполнитель pipeline:developer, ревью — pipeline:reviewer с opus.
- **2026-09-29 · lead (Claude):** взята с --force: у F04-01 остались только правки по ревью и живая проверка, хранилище от них не зависит. Передана исполнителю pipeline:developer (sonnet), цепочка новая.
