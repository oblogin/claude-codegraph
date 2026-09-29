# Bench — набор проверок графа кода

Здесь лежат эталоны («правильные ответы»), по которым меряем полноту и точность графа связей: нашего плагина, готовых инструментов (Graft, CodeGraph и др.) и базовой линии `grep`. Задача — F02-01 «Эталонные связи в наших проектах» (`Docs/Tasks/E-01 …/F-02 …/`), скрипт прогона — F02-02, синтетические базы — F02-03.

```
Bench/
  README.md          ← этот файл: формат эталона, проекты, как добавить свой
  projects.yaml      ← реестр проектов: URL, коммит, способ клонирования
  projects/<name>/   ← клоны (в git НЕ коммитятся, см. .gitignore)
  cases/<name>/*.yaml← эталоны, один файл = один эталон
```

## Проекты

| Ключ | Что это | Язык | URL | Коммит | Размер |
|---|---|---|---|---|---|
| `chopchop` | Unity Open Project #1 «Chop Chop», Unity 2020.3, текстовые сцены и префабы | C# + Unity YAML | https://github.com/UnityTechnologies/open-project-1 | `608eac98df29cd97821a6115cd52dfb9027345b1` | 311 `.cs` (~22 тыс. строк), 50 сцен, 483 префаба, 656 `.asset` |
| `excalibur` | Excalibur — 2D-движок для браузерных игр на TypeScript, без редактора сцен | TS | https://github.com/excaliburjs/Excalibur | `760a28a8544d477a496cbb72a14ac68ff9583354` | `src/engine`: 294 файла (~60 тыс. строк), тесты `src/spec`: 160 файлов |
| `nano` | nano — лёгкий фреймворк игровых серверов на Go (сессии, группы, кластер, обработчики через reflection) | Go | https://github.com/lonng/nano | `dbf22c701d271c5dcdb647df375320a9986b83d5` | 77 `.go` (~9,6 тыс. строк) |
| `ggez` | ggez — игровой фреймворк на Rust (трейты `EventHandler`, `Drawable`, `Has<T>`, дженерики) | Rust | https://github.com/ggez/ggez | `9c865474504084b60204874bc028ba91c5f013b9` | 73 `.rs` (~21 тыс. строк) |

Проекты пользователя пока не добавлены: первая версия набора собрана на открытых проектах, похожих на наш стек (решение лида от 2026-09-29).

### Как клонировать (воспроизвести `Bench/projects/`)

Во всех командах ниже в Git Bash на Windows нужно `export MSYS_NO_PATHCONV=1`, иначе пути вида `/site/` в шаблонах sparse-checkout превращаются в `C:/Program Files/Git/site/`.

```bash
cd Bench/projects

# Unity: репозиторий огромный из-за бинарных ассетов — берём только текстовые файлы.
git clone --depth 1 --filter=blob:none --no-checkout https://github.com/UnityTechnologies/open-project-1.git chopchop
cd chopchop
git sparse-checkout set --no-cone '/LICENSE*' '/README*' '/UOP1_Project/ProjectSettings/' '/UOP1_Project/Packages/' \
  '/UOP1_Project/Assets/Scripts/' '*.cs' '*.meta' '*.unity' '*.prefab' '*.asset' '*.asmdef' '*.anim' '*.controller' \
  '*.overrideController' '*.inputactions' '*.mat' '*.playable' '*.signal' '*.mixer' '*.json' '*.shadergraph' '*.shadersubgraph'
git checkout 608eac98df29cd97821a6115cd52dfb9027345b1
cd ..

# TS: без папки site/ (сайт документации ~400 МБ).
git clone --depth 1 https://github.com/excaliburjs/Excalibur.git excalibur
git -C excalibur sparse-checkout set --no-cone '/*' '!/site/'

git clone --depth 1 https://github.com/lonng/nano.git nano
git clone --depth 1 https://github.com/ggez/ggez.git ggez
```

Если `main`/`master` ушёл вперёд, эталон относится к коммиту из таблицы: `git fetch --depth 1 origin <commit> && git checkout <commit>`. Код проектов не собираем и не запускаем — эталоны составлены чтением.

## Формат эталона

Один YAML-файл — один эталон: `Bench/cases/<project>/<id>.yaml`. Все пути — относительно корня клона (`Bench/projects/<project>/`), строки — с 1. Формат одинаков для реальных проектов и для синтетических баз F02-03.

```yaml
schema: 1
id: chopchop-01-protagonist-placement   # уникальный; совпадает с именем файла
project: chopchop                       # ключ из projects.yaml
commit: 608eac98df29cd97821a6115cd52dfb9027345b1
languages: [csharp, unity-yaml]         # csharp | unity-yaml | ts | js | go | rust
kind: unity_placement                   # тип эталона, см. ниже
trap: true                              # есть ложные совпадения (must_not), которые нельзя выдавать
question: >                             # вопрос, как его задал бы разработчик агенту
  На каких объектах в сценах и префабах висит компонент Protagonist?
target:                                 # о чём спрашиваем
  file: UOP1_Project/Assets/Scripts/Characters/Protagonist.cs
  symbol: Protagonist
  line: 7
direction: incoming                     # incoming: кто ссылается на target; outgoing: на что ссылается target
change: rename                          # только для kind: impact — rename | signature | delete | move
match: file                             # как сравнивать ответ с эталоном: file | symbol | line
scope: [UOP1_Project/Assets/]           # где искать; находки вне scope не считаются
edges:
  - rel: script_on_object               # тип связи, словарь ниже
    expect: must                        # must | may | must_not
    from: {file: UOP1_Project/Assets/Prefabs/Characters/PigChef.prefab, line: 340, object: PigChef, fileID: "5557640735889932260"}
    to: target                          # «target» = сам target; можно не писать
    via: []                             # цепочка файлов для транзитивных связей (необязательно)
    confidence: high                    # high | medium | low
    how: "m_Script guid 45f6… = Protagonist.cs.meta; документ не stripped"
notes: >
  Свободный текст: спорные места, что сознательно не включено.
```

### Поля эталона

| Поле | Обяз. | Смысл |
|---|---|---|
| `schema` | да | Версия формата, сейчас `1`. |
| `id` | да | Уникальный идентификатор, совпадает с именем файла без `.yaml`. |
| `project`, `commit` | да | Проект из `projects.yaml` и коммит, для которого эталон верен. Для синтетики `commit` можно не писать. |
| `languages` | да | Какие языки/форматы затрагивает эталон. |
| `kind` | да | `consumers` — все потребители символа; `implementations` — кто реализует/переопределяет; `callees` — что реально вызывается из точки (outgoing); `unity_placement` — где висит скрипт; `unity_event` — какие UnityEvent вызывают метод; `engine_message` — метод вызывает движок по имени; `impact` — что затронет правка `change`. |
| `trap` | нет | `true`, если в эталоне есть `must_not` (ловушка для grep и «наивного» графа). |
| `question` | да | Формулировка вопроса. Адаптер инструмента может отдать её агенту как есть. |
| `target` | да | Узел запроса (формат узла — ниже). |
| `direction` | да | `incoming` (по умолчанию) или `outgoing`. |
| `change` | для `impact` | Какая правка: `rename`, `signature`, `delete`, `move`. |
| `match` | да | Минимальная точность, с которой находка засчитывается: `file` — совпал файл (для ассетов Unity ещё и `fileID`, если он указан); `symbol` — файл + объемлющий символ; `line` — файл + строка с допуском `line_tolerance`. |
| `line_tolerance` | нет | Допуск по строке для `match: line`, по умолчанию 2. |
| `scope` | нет | Префиксы путей, в которых ищем. Находки вне scope игнорируются (не штрафуются и не засчитываются). Нет поля — весь проект. |
| `edges` | да | Список ожидаемых связей. Может не содержать ни одного `must` — тогда эталон проверяет только точность (полнота не считается). |
| `notes` | нет | Спорные места, что сознательно не учтено. |

### Узел (конец связи)

| Поле | Смысл |
|---|---|
| `file` | Путь к файлу от корня клона. Обязателен, кроме внешних узлов. |
| `symbol` | Полное имя символа в принятой для языка записи: `Class.Method` (C#, TS), `pkg.(*Type).Method` / `pkg.Func` (Go), `Type::method` / `<Type as Trait>::method` (Rust). Для связей из кода — объемлющий символ, где стоит ссылка. |
| `line` | Строка, где стоит ссылка (вызов, подписка, `m_Script`, `m_MethodName`…), с 1. |
| `object` | Unity: имя или путь GameObject (`Player/Body`), если известно. |
| `fileID` | Unity: fileID документа компонента/объекта внутри файла, **строкой** (64-битные числа). |
| `external` | Внешний вызывающий, которого нет в коде проекта: `unity:message:OnTriggerEnter`, `go:reflection`, `browser:event-loop` и т. п. |

`to: target` (или отсутствие `to`) означает «связь идёт в target»; для `direction: outgoing` наоборот, `from` можно не писать, а в `to` — узел, куда ведёт связь.

### Словарь связей `rel`

| `rel` | Когда |
|---|---|
| `call` | Прямой вызов по статическому типу. |
| `call_virtual` | Вызов через базовый класс/абстрактный метод, реальная цель — переопределение (C#, TS). |
| `call_interface` | Вызов через интерфейс/трейт/дженерик-ограничение (C#, TS, Go interface, Rust `impl Trait`/`dyn`). |
| `override` | Метод переопределяет/реализует метод базы или интерфейса. |
| `inherit` | Класс наследует класс (`extends`, `: Base`), Go-встраивание (embedding). |
| `implement` | Тип реализует интерфейс/трейт (явно `implements`/`impl Trait for` или неявно, как в Go). |
| `import` | Импорт модуля/символа. |
| `reference` | Упоминание символа без вызова: тип поля, аргумент, `typeof`, чтение/запись поля. |
| `event_subscribe` / `event_unsubscribe` | Подписка/отписка на событие или делегат (`+=`, `-=`, `.on(`, `.off(`). |
| `event_emit` | Вызов события (`Invoke`, `emit`, `RaiseEvent`). |
| `callback_register` | Функция/метод передан как значение (колбэк, делегат, обработчик) — сам вызов будет позже. |
| `callback_invoke` | Место, где вызывается функция-значение, в которую попадает target. |
| `reflection` | Связь через reflection или регистрацию по имени (Go `reflect`, обработчик по строке маршрута). |
| `string_ref` | Ссылка строковым литералом (маршрут, имя события, имя метода в `SendMessage`). |
| `test` | Тест, который проверяет target (вызов из тестового файла). |
| `script_on_object` | Unity: компонент со `m_Script` = target прямо в этом файле. |
| `script_via_prefab` | Unity: компонент попадает в файл через вложенный префаб или вариант (`PrefabInstance.m_SourcePrefab`), цепочка — в `via`. |
| `prefab_instance` / `prefab_variant` | Unity: файл содержит экземпляр префаба / является вариантом префаба. |
| `unity_event` | Unity: `m_PersistentCalls.m_Calls[]` с `m_MethodName` = метод target (в теле документа или через `m_Modifications`). |
| `serialized_field` | Unity: в ассете лежат данные сериализованного поля target (ключ YAML или `propertyPath`). |
| `asset_ref` | Ссылка на ассет по GUID (`{fileID, guid}`, `m_AssetGUID`). |
| `engine_message` | Движок вызывает метод по имени (Unity `Awake`, `OnTriggerEnter`…, Animation Event `functionName`); конец связи — `external`. |
| `instantiate` | Создание экземпляра типа (`new T()`, `T{}`, `T::new()` как конструктор). |

Новый тип связи добавляем сюда же, прежде чем использовать в эталоне.

### Соответствие модели графа (ADR-006)

Решение лида (2026-09-29): словарь `rel` — язык тестов, модель графа ADR-006 — внутренний язык ядра. Переводит адаптер `to_bench()` в тест-раннере ([[F02-02]]); эталоны под модель не переписываются.

| `rel` в Bench | Как получается из графа ADR-006 |
|---|---|
| `call` | ребро `calls` на цель, у которой нет переопределений среди кандидатов |
| `call_virtual`, `call_interface` | ребро `calls` на базовый метод или метод интерфейса + переход по входящим `overrides`/`implements` к реализациям |
| `override` / `inherit` / `implement` / `import` | `overrides` / `inherits` / `implements` / `imports` |
| `reference` | `uses_type`, `reads`, `writes` (любое из трёх) |
| `instantiate` | `instantiates` |
| `event_subscribe`, `event_unsubscribe` | `handles` (обработчик → событие) + место подписки из доказательства ребра; отписка — по токену `-=`/`off` в доказательстве |
| `event_emit` | `raises` |
| `callback_register` | `references` (метод как значение) |
| `callback_invoke` | не строится в MVP — эталоны с этим видом считаются `may` |
| `reflection`, `string_ref` | `calls` или `unity_ref` с источником `heuristic` |
| `test` | любое ребро из узла с флагом `test` |
| `script_on_object` | `runs_script` |
| `script_via_prefab` | запрос `where_used` (цепочки `instance_of`/`variant_of` за вычетом удалённых компонентов) |
| `prefab_instance` / `prefab_variant` | `instance_of` / `variant_of` |
| `unity_event` | ребро UnityEvent → метод из слоя Unity |
| `serialized_field` | `serializes` |
| `asset_ref` | `unity_ref` (GUID и литералы `Resources.Load`, `LoadScene`, адреса Addressables) |
| `engine_message` | флаг узла `entry=unity_message` (для Animation Event — `anim_event`) |

Узлы: короткая запись `Class.Method` из эталона сопоставляется с ключом ADR-006 по имени типа и метода; перегрузки — списком типов `Format(int,int)`, явная реализация — `Type.IFace.M`.

### `expect`: как считать

- **`must`** — связь обязана быть в ответе. **Полнота** = найденные `must` / все `must`.
- **`may`** — допустимо, но не обязательно (транзитивные связи, спорные случаи, упоминание в комментарии документации). Не влияет ни на полноту, ни на точность.
- **`must_not`** — ложное совпадение (обычно то, что находит `grep` по имени). Попадание в ответ — **ошибка точности**.
- Находка, которой нет в эталоне и которая попала в `scope`, — «лишняя»: отчёт показывает её отдельно, на ручной разбор (может оказаться пропуском эталона).

Совпадение находки с записью эталона определяется по `match` эталона. Если в одном файле несколько записей и `match: file`, файл засчитывается один раз, а `must_not` в этом же файле — только если в файле нет ни одной `must`/`may` записи.

### `confidence` и `how`

`how` — как связь проверена человеком/агентом при составлении (что открыто и прочитано). `confidence: high` — проверено глазами по коду/YAML; `medium` — вывод по косвенным признакам (например, тип переменной выведен из контекста, runtime-регистрация); `low` — предположение, стоит перепроверить.

## Эталоны (версия 1, 27 шт.)

| Проект | Эталоны | По типам (`kind`) |
|---|---|---|
| `chopchop` (C# + Unity YAML) | 10 | unity_placement 2, unity_event 3, consumers 2 (C#-событие, колбэк через интерфейс), engine_message 1, impact 2 |
| `excalibur` (TS) | 5 | consumers 4 (прямые вызовы, хук через цепочку событий, 2 ловушки с одноимёнными методами), impact 1 |
| `nano` (Go) | 6 | consumers 3 (прямые, reflection + строковый маршрут, колбэк), callees 1, implementations 1, impact 1 |
| `ggez` (Rust) | 6 | consumers 3, implementations 1, impact 2 |

Итого: consumers 12, impact 6, unity_event 3, unity_placement 2, implementations 2, callees 1, engine_message 1.
Ловушки (`trap: true`, есть `must_not`) — 24 из 27. Связей в эталонах: `must` 186, `may` 73, `must_not` 93.

| Эталон | О чём |
|---|---|
| `chopchop-01-protagonist-placement` | Где висит Protagonist; ловушка — компонент удалён в экземплярах (`m_RemovedComponents`) |
| `chopchop-02-collectableitem-variants` | Где висит CollectableItem; 9 вариантов префаба без GUID скрипта |
| `chopchop-03-critter-alert-unityevent` | UnityEvent задан переопределением во вложенном префабе |
| `chopchop-04-genericbutton-click-trap` | UnityEvent с устаревшим `m_TargetAssemblyTypeName`; одноимённые Click |
| `chopchop-05-cutscene-loadscene-signal` | Метод вызывается из Timeline SignalReceiver; ловушки LoadScene* |
| `chopchop-06-inputreader-moveevent` | Подписчики C#-события; ловушки CameraMoveEvent/MenuMouseMoveEvent |
| `chopchop-07-inputreader-onmove-interface` | Вызов через интерфейс Input System в сгенерированном коде; три OnMove |
| `chopchop-08-stateaction-onupdate-impact` | Смена сигнатуры абстрактного метода: 30 переопределений; параллельная иерархия |
| `chopchop-09-zonetrigger-enterzone-rename` | Переименование сериализованного поля: данные в 5 префабах |
| `chopchop-10-zonetrigger-ontriggerenter-message` | Сообщение движка OnTriggerEnter |
| `excalibur-01-imagesource-load-trap` | ImageSource.load среди ~20 вызовов load() |
| `excalibur-02-actor-oncollisionstart-hook` | Хук, вызываемый из обработчика события после pipe(); тесты |
| `excalibur-03-entity-oninitialize-trap` | onInitialize у 6 несвязанных иерархий классов |
| `excalibur-04-loadable-load-rename-impact` | Переименование метода интерфейса: 9 файлов движка + 22 файла тестов |
| `excalibur-05-eventemitter-pipe-callers` | Прямые вызовы + тесты; ловушка unpipe |
| `nano-01-session-push-callers` | Session.Push среди 4 типов с Push; параметр с именем пакета |
| `nano-02-roommanager-join-reflection` | Обработчик через reflection и строковый маршрут в JS-клиенте |
| `nano-03-component-afterinit-callees` | Куда уходит вызов через интерфейс; незарегистрированный компонент |
| `nano-04-master-onsessionclosed-callback` | Колбэк-функция; одноимённые функции в трёх пакетах |
| `nano-05-networkentity-implementations` | Неявная реализация интерфейса; Session «почти» реализует |
| `nano-06-group-broadcast-signature-impact` | Смена сигнатуры; вызовы через встраивание (embedding) |
| `ggez-01-resize-event-signature-impact` | Метод трейта с реализацией по умолчанию |
| `ggez-02-mesh-draw-callers-trap` | `<Mesh as Drawable>::draw` среди десятка draw |
| `ggez-03-drawable-draw-signature-impact` | Смена сигнатуры трейта; параллельный трейт Drawable3d |
| `ggez-04-has-graphicscontext-impls` | Реализации `Has<GraphicsContext>`, включая blanket impl |
| `ggez-05-check-update-time-trap` | Метод и устаревшая свободная функция с тем же именем |
| `ggez-06-request-quit-callers` | Документация утверждает вызов, которого в коде нет |

Все пути и строки эталонов проверены автоматически (файл существует, строка в пределах файла и содержит ожидаемый текст — сверено глазами по распечатке).

## Как добавить проект пользователя

1. Добавить запись в `projects.yaml`: `name`, `source` (`git` с `url` и `commit` или `local` с `path`), `languages`, `root`.
2. Для Unity проверить `ProjectSettings/EditorSettings.asset` → `m_SerializationMode: 2` (Force Text) и что `.meta` лежат рядом с ассетами. Бинарные сцены эталонами не покрываем.
3. Клонировать в `Bench/projects/<name>/` (для больших Unity-проектов — как `chopchop`, только текстовые файлы). Если проект закрытый — не коммитить ни клон, ни фрагменты кода в эталоны: в эталоне только пути, строки и имена.
4. Выбрать 5–10 символов: по одному на каждый `kind`, хотя бы одну ловушку (одноимённые методы, строковые ссылки, удалённый в экземпляре компонент).
5. Для каждого составить эталон вручную: кандидатов можно искать `grep`, но каждый проверить глазами; ложные совпадения grep записать как `must_not`. Для Unity цепочки префабов удобно найти скриптом по `.meta`/`m_SourcePrefab`, но записи в эталоне проверяются чтением YAML.
6. Положить файлы в `Bench/cases/<name>/`, прогнать скрипт F02-02 и разобрать «лишние» находки: либо это ошибка инструмента, либо эталон неполон — тогда дописать эталон.

## Синтетические базы (F02-03)

Синтетическая база — маленький проект, граф которого известен заранее. Лежит в `Tests/fixtures/<язык>/<база>/` (не в `Bench/`), ожидаемый граф — `expected.yaml` рядом с базой: поток YAML-документов через `---`, каждый документ — эталон в формате выше; `project: fixture-<язык>-<база>`, `commit` не нужен, `confidence` — `high`, для соседних строк `match: line` с `line_tolerance: 0`. Список баз и что каждая проверяет — `Tests/fixtures/README.md`. При ручной правке базы номера строк в `expected.yaml` пересчитываются проверкой из раннера F02-02.
