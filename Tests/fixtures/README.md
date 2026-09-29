# Тестовые кодовые базы с ожидаемым графом (F02-03)

Маленькие синтетические проекты, граф связей которых известен заранее. Парсер пишется от них: каждая особенность языка проверяется отдельным эталоном и не ломается при следующих правках. Раннер, который индексирует базу и сравнивает результат с ожидаемым, — задача F02-02; здесь только базы и ожидаемые графы.

```
Tests/fixtures/<язык>/<база>/        ← корень базы: все пути в ожидаемом графе — от него
Tests/fixtures/<язык>/<база>/expected.yaml
```

Код баз корректен по синтаксису и по смыслу компилируется, но не собирается и не запускается.

## Формат `expected.yaml` (TS, Go, Rust)

- Формат эталона — `Bench/README.md`, раздел «Формат эталона», без изменений полей. Один YAML-документ (между `---`) = один эталон; в файле — все эталоны базы.
- `project: fixture-<язык>-<база>` (как у баз C#) — ключ для реестра (`Bench/projects.yaml`, `source: synthetic`, `path: Tests/fixtures/<язык>/<база>`); `commit` не пишется, `confidence` всегда `high`.
- `match: line` с `line_tolerance: 0`: строки в синтетике точные, а рядом часто стоят `must` и `must_not` (с допуском 2 находка на одной строке засчиталась бы и там, и там). Эталоны на импорт модуля — `match: file`.
- Строка связи — место ссылки: вызов, `import`/`use`, `impl … for`, встраиваемое поле Go, строка `extends`/`implements`. Для объявлений (`override`, `implement` в Go) — строка объявления метода или типа.
- Транзитивные звенья, динамическая диспетчеризация (`dyn`, `impl Trait`, дженерики после мономорфизации), шаблонный `import()`, вызовы из раскрытия макросов и «вызов колбэка через коллекцию обработчиков» — `may` с пояснением в `how`.
- Одноимённые символы, которые находит `grep` по имени, — `must_not` (эталоны с `trap: true`).

## Формат `expected.yaml` (C# и Unity)

Те же правила, что выше (один документ = один эталон, `project: fixture-<язык>-<база>`, `match: line` с `line_tolerance: 0`, `confidence: high`), плюс:

- **Символы C#** — `Класс.Член` без пространства имён, как в эталонах `chopchop`. Перегрузки — со списком типов параметров: `Formatter.Format(int,int)`. Явная реализация интерфейса — `FileStore.IBackup.Save`. Дженерики — без параметров типа: `Repository.Add` = `Repository<T>.Add`. Лямбда — не узел: ссылка из её тела относится к объемлющему методу (ADR-006 §1). Одноимённые классы (`Log`, `Settings`, `Tween`, `Store`) различаются полем `file`.
- **Узлы в YAML Unity**: `line` — строка самой ссылки (`m_Script`, `m_SourcePrefab`, `m_MethodName`, ключ поля, `propertyPath`, `m_AssetGUID`, `functionName`, `type: {class…}`), `fileID` — документ компонента или `PrefabInstance`, **строкой**, `object` — путь GameObject. Если target — ассет (префаб, сцена), в `target` только `file`.
- Размещения скриптов (`unity_placement`), варианты и вложенность префабов, переименование сериализованных полей (`impact`) — `match: file`; тесты — `match: symbol`.
- Виды связей — только из словаря `Bench/README.md`. Где в ADR-006 есть более точный вид, записан ближайший из словаря, а в `notes` эталона это сказано: `anim_event` → `string_ref`, `references_type` → `reference`, `depends_on` (сборка → сборка) → `import`.

## C# — `csharp/language`

Чистый C# без Unity (`Fx.Language.csproj`, net8.0): по папке на особенность языка, пространство имён `Fx.<Папка>`; одноимённые «чужие» методы и классы — в `Traps/`. 54 файла (53 `.cs` по 10–30 строк), 30 эталонов.

| Эталон | Что проверяет |
|---|---|
| `cs-lang-01-shape-area-overrides` | Переопределения абстрактного метода, в том числе через промежуточный класс; `Room.Area` вне иерархии |
| `cs-lang-02-shape-subclasses` | Наследование; транзитивный наследник — `may` |
| `cs-lang-03-circle-area-callers` | Вызов по статическому типу; виртуальные вызовы через базу — `may`; `square.Area()`, `room.Area()`, `base.Area()` другой цели — ловушки |
| `cs-lang-04-square-area-callers` | `base.Area()` из наследника второго уровня |
| `cs-lang-05-describe-overrides` | `override` против `new` (сокрытие — не переопределение) |
| `cs-lang-06-shape-describe-callers` | `base.Describe()`, вызов с неявным `this` |
| `cs-lang-07-istorage-implementations` | Реализация интерфейса; класс с теми же методами без `: IStorage` — ловушка |
| `cs-lang-08-ibackup-save-explicit` | Явная реализация `void IBackup.Save`; публичный `Save` того же класса реализует только `IStorage.Save` |
| `cs-lang-09-istorage-save-callers` | Вызов через поле-интерфейс и через ограничение `where T : IStorage`; вызов через `IBackup` — ловушка |
| `cs-lang-10-persist-callees` | Outgoing: вызов через интерфейс → реализации; явная реализация другого интерфейса — ловушка |
| `cs-lang-11-format-int-int-overload` | Перегрузка по числу аргументов |
| `cs-lang-12-format-string-overload` | Перегрузка по типу литерала (`int` против `string`) |
| `cs-lang-13-repository-add-callers` | Метод дженерик-класса; `List<T>.Add` внутри — не рекурсия |
| `cs-lang-14-repository-add-callees` | Вызов через ограничение `T : IEntity`; `Form.Validate` — ловушка |
| `cs-lang-15-health-damaged-event` | Событие: `+=`, `-=`, `?.Invoke`, подписка лямбдой; одноимённое событие `Shield.Damaged` |
| `cs-lang-16-healthbar-ondamaged-handler` | Метод-обработчик события (`callback_register`); одноимённый обработчик другого класса |
| `cs-lang-17-healthbar-hide-lambda` | Вызов внутри лямбды-подписчика |
| `cs-lang-18-combatlog-write-lambda` | Два вызова из лямбд; `GameLog.Write`, `AuditLog.Write`, `Log.Write` — ловушки |
| `cs-lang-19-healthbar-refresh-delegate-field` | Метод, присвоенный полю-делегату (`=`, не событие); вызов поля — `may` |
| `cs-lang-20-mathsteps-double-method-group` | Группа методов как `Func<int,int>`; `Money.Double` — ловушка |
| `cs-lang-21-mathsteps-increment-lambda` | Вызов в лямбде, переданной аргументом |
| `cs-lang-22-character-subclasses-partial` | `partial`-класс: база объявлена в одной части; `CharacterSheet` — совпадение префикса |
| `cs-lang-23-hero-attack-partial-callers` | Вызов между частями `partial`-класса; `Enemy.Attack` — ловушка |
| `cs-lang-24-character-heal-extension` | Метод расширения базового класса: вызов на наследнике и статической формой; `Health.Heal` — ловушка |
| `cs-lang-25-string-shout-extension` | Расширение `string`; метод экземпляра `Megaphone.Shout` — ловушка |
| `cs-lang-26-memorystore-save-alias` | `using Store = …MemoryStore` |
| `cs-lang-27-traps-store-save-alias-trap` | Алиас побеждает одноимённый тип из `using Fx.Traps` |
| `cs-lang-28-gamelog-write-static` | Статический вызов и вызов через алиас `using Log = …GameLog`; одноимённые методы экземпляров |
| `cs-lang-29-mathutil-clamp-using-static` | `using static`: вызов без квалификатора; `Limits.Clamp` на соседней строке — ловушка |
| `cs-lang-30-settings-volume-property` | Свойство: чтение, запись, `+=`, `nameof` (`may`); `AudioMixer.Volume` — ловушка |

Связей: `must` 53, `may` 16, `must_not` 40; ловушек — 26 эталонов из 30.

## C# — `csharp/asmdef`

Unity-проект только с кодом (`.cs`, `.asmdef`, `.meta`): видимость между сборками и порядок поиска имён. Сборки: `Game.Core` (`Assets/Core`); `Game.Gameplay` → Core по имени; `Game.UI` → Core по `GUID:`; `Game.Tools` → Core, `autoReferenced: false`; `Assembly-CSharp` (`Assets/Scripts`), `Assembly-CSharp-Editor` (`Assets/Scripts/Editor`), `Assembly-CSharp-firstpass` (`Assets/Plugins`); тестовая `Game.Core.Tests` (EditMode). 38 файлов, 9 эталонов. Проверяет допущение Г14 ADR-006.

| Эталон | Что проверяет |
|---|---|
| `cs-asm-01-gameplay-settings-load` | Два класса `Game.Shared.Settings` с одинаковым полным именем в разных сборках: `Assembly-CSharp` видит только `autoReferenced` |
| `cs-asm-02-tools-settings-load` | То же с другой стороны: `Game.Tools` не ссылается на `Game.Gameplay` |
| `cs-asm-03-core-log-info` | Вызовы из четырёх сборок и из теста; в `Game.UI` имя `Log` находит свой класс раньше `using Game.Core` |
| `cs-asm-04-ui-log-info` | `internal`-класс своей сборки |
| `cs-asm-05-firstpass-tween` | `Assembly-CSharp` видит firstpass; тип глобального пространства имён находится раньше типов из `using` |
| `cs-asm-06-gameplay-tween` | Сборки `.asmdef` не видят firstpass |
| `cs-asm-07-core-asmdef-dependents` | Ссылки `.asmdef` по имени и по `GUID:`; `"name": "Game.Core.Tests"` — совпадение префикса |
| `cs-asm-08-core-stats-add-tests` | `[Test]` в тестовой сборке; соседний тест — ловушка |
| `cs-asm-09-bootstrap-run-from-editor` | `Assembly-CSharp-Editor` видит `Assembly-CSharp` |

Связей: `must` 19, `may` 0, `must_not` 9; ловушек — 8 эталонов из 9.

## Unity — `unity/minigame`

Мини-игра Unity 2022.3 в текстовой сериализации (`ProjectSettings/EditorSettings.asset`: `m_SerializationMode: 2`). У каждого ассета есть `.meta` с GUID (32 hex), у папок `.meta` нет. Скрипты — сборка `Game.Runtime` (`Assets/Scripts`), тесты — `Game.Tests.EditMode`. 64 файла (половина — `.meta`), 22 эталона.

Что лежит в базе:
- `Assets/Scenes/Main.unity` — Canvas с `MainMenu`, две кнопки uGUI с `m_OnClick` (`OnPlayClicked` у `MainMenu` и у `Tutorial`), `Spawner` с `EnemySpawner` (поле Addressables `m_AssetGUID`), экземпляр `Player.prefab` с переопределениями (`_speed`, старое имя `jumpForce`, UnityEvent `Health.OnDied` → `MainMenu.ShowGameOver` через `m_Modifications`), экземпляр `CoinCluster.prefab`; `SceneRoots` формата 2022.3.
- `Assets/Scenes/Bonus.unity` — только экземпляр варианта `MagnetCoin.prefab`.
- Префабы: `Player` (два компонента, у `Health` отрицательный fileID), `Enemy` (`Animator` → `Enemy.controller`, `[SerializeReference]` с `PatrolBehaviour`), варианты `Enemy` — `Boss` (Addressable) и `Resources/Enemies/Slime`; одноимённый `Prefabs/Enemies/Slime.prefab` вне `Resources` — ловушка; `Coin`; вложенный `CoinCluster` (два экземпляра `Coin` со stripped-заглушками, fileID по формуле XOR); вариант `MagnetCoin` с `m_RemovedComponents` (`Coin`) и `m_AddedComponents` (`Magnet`).
- `Assets/Animations/Enemy_Attack.anim` (Animation Event `functionName: OnAttackHit`) и `Enemy.controller`; `Assets/Data/Waves.asset` — ScriptableObject с `[SerializeReference]`, одна запись с устаревшим `ns: Game.AI`; группа Addressables `Default Local Group.asset` (адрес `Enemies/Boss`); `ProjectSettings/EditorBuildSettings.asset` со сценами.
- Скрипты пакетов (кнопка uGUI `4e29b1a8…`, группа Addressables `bbb281ee…`) ссылаются на GUID, которых в базе нет — как в проекте без `Library/PackageCache`: это не ошибка, такие компоненты должны уходить в неразрешённые.

| Эталон | Что проверяет |
|---|---|
| `unity-mg-01-coin-placement` | `m_Script` → `.meta` → класс; вложенный префаб и сцена через него; вариант с удалённым компонентом и сцена с ним — ловушки |
| `unity-mg-02-magnet-added-component` | Компонент из `m_AddedComponents` на stripped-объекте; база варианта — ловушка |
| `unity-mg-03-playercontroller-placement` | Префаб и сцена с его экземпляром |
| `unity-mg-04-enemy-prefab-consumers` | Варианты префаба, ссылка `{fileID, guid}` из ScriptableObject; одноимённый несвязанный префаб |
| `unity-mg-05-coin-prefab-nesting` | `prefab_instance` и `prefab_variant`; транзитивные сцены — `may` |
| `unity-mg-06-mainmenu-onplayclicked-unityevent` | `m_PersistentCalls` → метод; тот же `m_MethodName` у другого класса — ловушка |
| `unity-mg-07-mainmenu-showgameover-override` | UnityEvent, заданный переопределением `m_Modifications` в сцене |
| `unity-mg-08-ontriggerenter-message` | Сообщение движка; то же сообщение другого класса — ловушка |
| `unity-mg-09-awake-message` | `Awake`; `Health.Awake` — ловушка |
| `unity-mg-10-patrol-update-not-message` | `Update` не-MonoBehaviour-класса — не сообщение, а вызов через интерфейс |
| `unity-mg-11-lowercase-ontriggerexit` | `onTriggerExit` со строчной — не сообщение (эталон без `must`) |
| `unity-mg-12-playercontroller-speed-rename` | Данные поля в префабе и в `propertyPath` сцены; одноимённое поле `Enemy._speed` |
| `unity-mg-13-jumpheight-formerlyserializedas` | `[FormerlySerializedAs("jumpForce")]`: старый ключ в сцене относится к `PlayerController`, тот же ключ в `Enemy` — ловушка |
| `unity-mg-14-chasebehaviour-serializereference` | `references.RefIds[].type`; запись со старым пространством имён — ловушка |
| `unity-mg-15-patrolbehaviour-serializereference` | `[SerializeReference]` в префабе и инициализатор поля |
| `unity-mg-16-boss-addressables` | `m_AssetGUID` и адрес `Addressables.LoadAssetAsync("…")`; `Resources.Load` того же пути — ловушка |
| `unity-mg-17-resources-slime-consumers` | `Resources.Load("литерал")` — must; не литерал — `may` |
| `unity-mg-18-spawnslime-callees` | Outgoing: `Resources.Load` ищет только в `Resources/` |
| `unity-mg-19-enemy-onattackhit-anim-event` | Animation Event `functionName`; вызов одноимённого метода внутри — не рекурсия |
| `unity-mg-20-combatstats-onattackhit` | Animation Event не вызывает методы обычного класса |
| `unity-mg-21-health-takedamage-tests` | `[Test]` в `Tests/EditMode`; соседний тест — ловушка |
| `unity-mg-22-bonus-scene-loadscene` | `SceneManager.LoadScene("Имя")` → сцена из `EditorBuildSettings` |

Связей: `must` 36, `may` 6, `must_not` 23; ловушек — 17 эталонов из 22.

## Итого по C# и Unity

61 эталон; связей `must` 108, `may` 22, `must_not` 72; ловушек — 51 эталон.

## TS/JS — `ts/browser-game`

Браузерная игра на самописном движке без фреймворков: `src/engine` (сущности, события, рендер, математика), `src/game` (актёры, HUD, уровни, звук), `src/util` (TS и два JS-модуля), тесты vitest в `tests/`. Алиасы: `tsconfig.json` расширяет (`extends`) `tsconfig.base.json` с `baseUrl` и `paths` (`@engine`, `@engine/*`, `@game/*`, `@util`). 30 файлов, 18 эталонов.

| Эталон | Что проверяет |
|---|---|
| `01-util-format-trap` | Две функции `format` в разных модулях; алиас `@util` без звёздочки, `export { a } from`, импорт TS из JS без расширения |
| `02-clamp-renamed-reexport` | `export { clamp as clampValue }`, импорт пространства имён `* as math`, локальная одноимённая `clamp` |
| `03-entity-onupdate-overrides` | Абстрактный метод, виртуальный вызов через `this`, переопределения на двух уровнях; `Hud.onUpdate` вне иерархии |
| `04-drawable-implementations` | `implements`; класс с `draw` другой сигнатуры не реализует интерфейс |
| `05-drawall-callees` | Outgoing: вызов через интерфейс → реализации |
| `06-entity-inherit` | `extends` через алиас и `export *` |
| `07-onenemykilled-events` | Эмиттер событий: `on`/`off` метода-стрелки, `emit` по строковому каналу; соседние каналы и обработчики |
| `08-playhit-lambda-handler` | Вызов внутри лямбды-обработчика относится к объемлющему методу |
| `09-playgameover-method-as-handler` | Метод передан значением как обработчик |
| `10-starttimer-js-module` | Импорт реального `.js` из TS (`../util/timer.js`) |
| `11-tick-in-timer-callback` | Колбэк во внешний API браузера (`external: browser:event-loop`) |
| `12-onkeydown-dom-callback` | `addEventListener` на верхнем уровне модуля |
| `13-dynamic-import-level1` | `import("литерал")` — must; шаблонный ``import(`…${x}`)`` и `mod.default()` — may |
| `14-engine-math-module-imports` | Модуль-папка с `index.ts` через `paths` `@engine/*` и `export * from "./math"` |
| `15-util-index-alias-imports` | Кто импортирует `index.ts` по алиасу; импорты соседних `./format` — ловушки |
| `16-vector-add-trap` | `Vector.add` против `Set.add` |
| `17-player-jump-tests` | `test` из vitest-файла + вызов через поле объекта |
| `18-computescore-tests` | Тест через алиас `@game/*` |

Связей: `must` 34, `may` 35, `must_not` 23; ловушек — 13 эталонов из 18.

## Go — `go/arena`

Модуль `example.com/arena` (`go.mod`), пакеты `geom`, `entity`, `render`, `ui`, `events`, `game`, `cmd/arena`. 17 файлов, 13 эталонов.

| Эталон | Что проверяет |
|---|---|
| `01-entity-implicit-implementations` | Неявная реализация интерфейса по набору методов, в том числе с методом, продвинутым из встроенного указателя; тестовая подделка во внешнем пакете `game_test`; «почти» реализации (нет `ID`, нет `Update`) |
| `02-base-embedding` | Встраивание структуры по значению и указателем |
| `03-drawer-implementations` | Получатели значение/указатель; алиас импорта `r`; встраивание интерфейса из другого пакета; `Draw` с лишним параметром и `draw` со строчной — ловушки |
| `04-drawall-callees` | Outgoing через интерфейс; тип, который реализует интерфейс, но никогда к нему не приводится (`ui.Tooltip`) — `may` |
| `05-entity-update-dispatch` | Вызов метода интерфейса и реализации; одноимённый `HUD.Update` |
| `06-vec-add-trap` | `geom.Vec.Add` среди `World.Add` и `Score.Add` |
| `07-base-move-promoted` | Вызов продвинутого метода через встраивание; `Tooltip.Move` — ловушка |
| `08-vec-scale-pointer-receiver` | Метод с получателем-указателем на адресуемом поле |
| `09-score-callback-trap` | Значение-метод как обработчик шины; одноимённая функция `ui.OnMonsterKilled` в другом пакете |
| `10-goroutine-spawnloop` | `go w.spawnLoop(ch)` — вызов в горутине |
| `11-func-through-channel` | Функция передаётся через `chan func()` и вызывается в другой горутине |
| `12-world-tick-tests` | `_test.go` во внешнем тестовом пакете |
| `13-render-package-imports` | Импорт пакета, в том числе под алиасом (`match: file`) |

Связей: `must` 33, `may` 10, `must_not` 17; ловушек — 7 эталонов из 13.

## Rust — `rust/ecs-lite`

Пакет `ecs-lite` (`Cargo.toml`): библиотека `ecs_lite` + бинарник `src/main.rs` + интеграционный тест `tests/render_test.rs`. 16 файлов, 15 эталонов.

| Эталон | Что проверяет |
|---|---|
| `01-draw-trait-imports` | `pub use` в корне крейта, `super::`, импорт из бинарника и интеграционного теста через реэкспорт |
| `02-clamp-vs-f32-clamp` | `pub use self::ops::clamp` из приватного модуля; стандартный `f32::clamp` — ловушка |
| `03-draw-implementations` | `impl Trait for Type`; inherent-методы `draw` у `Label` и `World` — ловушки |
| `04-describe-blanket-impl` | Blanket impl `impl<T: Draw> Describe for T`; реализации через подстановку — `may` |
| `05-describe-method-callers` | Вызов через ограничение `where T: Describe`; `sprite.describe()` через blanket impl — `may` |
| `06-draw-layer-default-method` | Метод трейта с телом по умолчанию: вызов через ограничения, переопределение, вызов на типе без переопределения; поле `layer` — ловушка |
| `07-draw-all-dyn-callees` | Outgoing через `dyn Draw`: метод трейта — must, реализации — may |
| `08-sprite-draw-callers` | Прямой вызов — must; `impl Trait` (аргумент и возврат), дженерик, `dyn` — may |
| `09-component-update-default` | Метод по умолчанию через `Box<dyn Component>`; `Entity::update`/`World::update`/`Label::update` |
| `10-vec2-new-trap` | `Vec2::new` среди шести `new`; вызовы из раскрытия `vec2!` — may |
| `11-vec2-macro` | `macro_rules!` с `#[macro_export]`: вызовы `vec2!` (через `use` и textual scope `#[macro_use]`); `vec!` и модуль `vec2` — ловушки |
| `12-vec2-length-trap` | `Vec2::length` против `Text::length`; `#[test]` в `mod tests` того же файла |
| `13-world-imports-super` | `use super::World`, `pub use` |
| `14-component-impls-generic-bound` | Реализации трейта; ограничение `C: Component + 'static` |
| `15-renderer-push-trap` | `Renderer::push` против `Vec::push` |

Что ожидаем от макросов: вызов `vec2!(…)` — ребро на узел `macro` (`must`); тело `macro_rules!` не раскрывается, поэтому `Vec2::new` из раскрытия — `may`, а тип выражения `vec2!(…)` неизвестен (вызов `.length()` на нём — `may`).

Связей: `must` 42, `may` 32, `must_not` 31; ловушек — 12 эталонов из 15.

## Итого по TS, Go, Rust

46 эталонов; связей `must` 109, `may` 77, `must_not` 71.
