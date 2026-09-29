# 05. Unity и браузерные движки: откуда брать связи

> Задача: F01-05 «Разбор Unity и браузерных движков».
> Дата: 2026-09-29. Исследователь: Claude (субагент).
> Unreal не разбираем: он отложен.

**Как читать пометки.**
- **[проверено]** — видел своими глазами: в файлах открытых проектов, в исходниках или в дословном тексте документации.
- **[заявлено]** — так написано в документации, README или новостях. Сам не запускал и не проверял.
- **[по памяти]** — общеизвестный факт, первоисточник в этой задаче не открывал.
- **[не нашёл]** — искал и не нашёл.

Код инструментов не запускался и не устанавливался. Репозитории клонировались с `--depth 1` и читались.

Примеры YAML взяты из открытого проекта Unity **Chop Chop** (Unity Open Project #1, Apache-2.0, Unity 2020.3.17f1), коммит `608eac9`. Короткий корень ссылок:
`CC:` = `https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/`

---

## Главное коротко

1. **Почти все связи «скрипт ↔ сцена/префаб» можно достать из файлов, без редактора.** Для этого проект должен хранить ассеты текстом (режим *Force Text*, это умолчание) и держать видимые `.meta`. Нужны три вещи: карта `GUID → файл` из `.meta`, поле `m_Script` у компонентов и цепочки `m_SourcePrefab` у вложенных префабов и вариантов. **[проверено]** на Chop Chop.
2. **Новый Unity CLI существует и он официальный.** Это отдельная программа `unity` от Unity Technologies. Бета началась весной 2026 года, анонс был 20.07.2026, текущая версия — `1.0.0-beta.11` от 22.09.2026. Программа бесплатная, но с закрытым кодом. **[заявлено]**
3. **Сам по себе CLI не отдаёт граф связей в пригодном для нас виде.** У него есть близкие команды: `unity vcs affected` («какие префабы и сцены затрагивает изменённый ассет»), `unity vcs diff` и `unity vcs blame`. Работают ли они без открытого редактора, в документации не сказано. JSON-схемы ответа тоже нет. «Живые» команды (`unity command …`, `eval` на C#) работают только с запущенным редактором Unity 6+ и экспериментальным пакетом `com.unity.pipeline` (0.3.x-exp). **[заявлено]**
4. **Unity MCP-серверов много, но все они работают через открытый редактор и нужны в основном для правок.** Самые крупные: CoplayDev/unity-mcp (~14,6 тыс. звёзд) и IvanMurzak/Unity-MCP (~4,4 тыс.). Готового «обратного поиска ссылок» у них нет. Встроенный MCP в пакете Unity AI Assistant Unity объявила устаревшим и советует переходить на CLI. **[проверено по коду / заявлено]**
5. **Автономные парсеры YAML есть, но все маленькие и узкие.** Это `unityparser` (Python), `unity-yaml-bridge` (TS), `unity-prefab-parser-mcp` (TS, MCP), `unity-assetdb` (Rust, есть поиск использований по GUID) и `uymerge` (Rust, слияние). Код можно смотреть как образец, но брать целиком в зависимости нет смысла. Формат простой, свой индексатор выйдет дешевле.
6. **Юридический риск.** 30.06.2026 Unity обновила условия использования. Пункты 17.2 (ff) и (gg) запрещают доступ к «предложениям Unity» через ИИ-агентов, CLI и MCP без «Authorized Agentic Access», а также сторонние плагины без письменного разрешения. Поддержка Unity на Reddit ответила, что локальное использование допустимо, но в текст условий это не внесли. Один из инструментов, `unity-agentic-tools`, прямо закрылся «из-за изменений в условиях Unity». **Чтение файлов проекта с диска этих пунктов, по нашему пониманию, не касается. Мосты внутрь редактора — серая зона.** Это не юридическая консультация. **[проверено: текст ToS / заявлено: ответ Unity]**
7. **В браузерных движках связей вне кода меньше.** Phaser и PixiJS — это почти чистый код плюс строковые ключи ассетов. Babylon.js Editor, Cocos Creator, PlayCanvas и Phaser Editor хранят сцены в JSON и привязывают скрипты по UUID, ключу-пути или имени. У Cocos формат проверен на примере: тип компонента в сцене — это сжатый UUID из `.meta` скрипта.
8. **Рекомендация для MVP-1: свой офлайн-индексатор текстового YAML Unity.** Он работает без редактора и без Unity CLI, с инкрементальным обновлением по файлам. Unity CLI использовать позже: как эталон для сверки (`vcs affected`) и как необязательный «живой» режим. Для браузерных игр в MVP-1 — только код JS/TS, редакторные форматы — после MVP.

---

## 1. Формат сцен, префабов и ассетов Unity

### 1.1. Условия: текстовая сериализация и `.meta`

- Режим хранения задаётся в `ProjectSettings/EditorSettings.asset`. Значение `m_SerializationMode: 2` — это *Force Text*. В Chop Chop стоит именно оно (`CC:ProjectSettings/EditorSettings.asset`). **[проверено]**
- Рядом с каждым ассетом и папкой лежит `.meta` с постоянным `guid`. Все ссылки между файлами идут по этому GUID, а не по пути. Поэтому переименование и перенос в редакторе ссылки не ломают. А перенос файла без его `.meta` ломает. **[проверено]**

Пример `.meta` скрипта (`CC:Assets/Scripts/StateMachine/Core/StateMachine.cs.meta`):

```yaml
fileFormatVersion: 2
guid: 1eeda163c70b3cb4ebb88ba92f608fd8
MonoImporter:
  ...
  executionOrder: 0
```

Пример `.meta` префаба (`CC:Assets/Prefabs/Characters/NPCs_BasePrefabs/Townsfolk_M.prefab.meta`):

```yaml
fileFormatVersion: 2
guid: 661c491ac76af4b429f8a8c0e6e8c762
PrefabImporter:
  ...
```

### 1.2. Устройство файла `.unity` / `.prefab` / `.asset`

Файл — это набор YAML-документов, по одному на объект:

```yaml
%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!114 &-6233334297847596787      ← класс 114 = MonoBehaviour, &… = fileID внутри файла
MonoBehaviour:
  ...
```

- Число после `!u!` — **класс** Unity (1 = GameObject, 4 = Transform, 114 = MonoBehaviour, 1001 = PrefabInstance и т. д.). Число после `&` — **fileID**, уникальный внутри файла. Документация Unity 6.6 описывает это кратко (страница [FormatDescription](https://docs.unity3d.com/Manual/FormatDescription.html)). **[проверено]**
- Ссылка на объект имеет вид `{fileID: N, guid: G, type: T}`:
  - без `guid` — объект в этом же файле;
  - `guid` — другой ассет, `fileID` — объект внутри него;
  - `type: 3` встречается у импортируемых ассетов (скрипты, префабы, текстуры), `type: 2` — у «родных» ассетов Unity (`.asset`, `.mat`), `type: 0` — у встроенных ресурсов с GUID вида `0000000000000000e000000000000000`. **[проверено по примерам; смысл type — по памяти]**
- У префаба корневой объект в ссылках — `fileID: 100100000`. У ScriptableObject и материала — `fileID: 11400000` / `2100000`. **[проверено]**

### 1.3. Как компонент ссылается на скрипт: `m_Script`

Кусок `Townsfolk_M.prefab` (`CC:Assets/Prefabs/Characters/NPCs_BasePrefabs/Townsfolk_M.prefab`, стр. 57–69):

```yaml
--- !u!114 &-6233334297847596787
MonoBehaviour:
  m_GameObject: {fileID: 5054559148327267577}       ← на каком объекте висит
  m_Enabled: 1
  m_Script: {fileID: 11500000, guid: 1eeda163c70b3cb4ebb88ba92f608fd8, type: 3}
  m_Name:
  _transitionTableSO: {fileID: 11400000, guid: fbef10fb82542eb458caa7ac9574384b, type: 2}
```

- `guid: 1eeda163…` совпадает с `.meta` файла `StateMachine.cs`, где объявлен `public class StateMachine : MonoBehaviour`. Так получается ребро **«StateMachine.cs висит на объекте в Townsfolk_M.prefab»**. **[проверено]**
- `fileID: 11500000` означает «скрипт из `.cs`-файла проекта». Unity требует, чтобы имя класса совпадало с именем файла. Значит, по GUID мы сразу знаем класс. **[проверено: fileID; правило имени — по памяти]**
- Скрипты из **DLL** ссылаются иначе. GUID указывает на DLL, а `fileID` — на хеш от пространства имён и имени класса. Пример из открытых фикстур: `m_Script: {fileID: -765806418, guid: f70555f144d8491a825f0804e09c671c, type: 3}` — это компонент старого UnityEngine.UI.dll. Для DLL понадобится таблица «хеш → класс» (или её можно пропустить в MVP). **[проверено: вид ссылки; формула хеша — по памяти]**
- Скрипты из пакетов (UPM) лежат в `Packages/` или `Library/PackageCache/…`, и у них тоже есть `.meta` с GUID. Индексатор должен обходить и эти папки. **[по памяти]**

Остальные поля того же документа — это сериализованные поля класса (`_transitionTableSO` и т. п.). Каждая ссылка с `guid` — это ребро «компонент → ассет».

### 1.4. Поле-ссылка через Addressables

Кусок `CC:Assets/Scenes/Initialization.unity` (стр. 152–161). В коде `InitializationLoader.cs` объявлено `[SerializeField] private AssetReference _menuLoadChannel`:

```yaml
  m_Script: {fileID: 11500000, guid: 43bf3a70578af0044a7af79dd1c44ea6, type: 3}
  _managersScene: {fileID: 11400000, guid: c2119785ce96367428081d6eee0d932b, type: 2}
  _menuLoadChannel:
    m_AssetGUID: 33cec85652903d245b99985f9cec9841    ← ссылка Addressables: голый GUID, без {fileID…}
```

Это важно: «обычный» поиск по шаблону `{fileID…, guid…}` такие ссылки пропустит. **[проверено]**

### 1.5. Вложенные префабы: `PrefabInstance`, `m_SourcePrefab`, `stripped`

Если префаб A вставлен в сцену или в префаб B, в файле B **нет копии** объектов A. Там есть только:

1. документ `PrefabInstance` (класс 1001) со ссылкой на исходный префаб и списком изменённых свойств;
2. «заглушки» (`stripped`) — только для тех объектов A, на которые в B кто-то ссылается.

Кусок `CC:Assets/Prefabs/Characters/NPCs_Examples/Andre.prefab` (стр. 97–110, 160–171):

```yaml
--- !u!4 &2905015166066834466 stripped                 ← заглушка Transform из вложенного префаба
Transform:
  m_CorrespondingSourceObject: {fileID: 2142915689959929641, guid: 661c491ac76af4b429f8a8c0e6e8c762, type: 3}
  m_PrefabInstance: {fileID: 3885908913776036619}
--- !u!1001 &8194636062233296424
PrefabInstance:
  m_Modification:
    m_TransformParent: {fileID: 2905015166066834466}   ← вставлен внутрь объекта выше
    m_Modifications:
    - target: {fileID: 4313218885251033291, guid: c87f5ac2d2b1bee43a1afec712ca6de5, type: 3}
      propertyPath: m_Name
      value: Sombrero                                   ← переопределённое свойство
    m_RemovedComponents: []
  m_SourcePrefab: {fileID: 100100000, guid: c87f5ac2d2b1bee43a1afec712ca6de5, type: 3}   ← ЧТО вставлено
```

Что из этого следует для графа:

- Ребро «B содержит экземпляр A» даёт `m_SourcePrefab.guid`. **[проверено]**
- **Чтобы ответить «на каких объектах висит скрипт X», нужно пройти цепочку.** Скрипт X висит в префабе A. Префаб A вставлен в B, а B — в сцену S. Значит, X «присутствует» в S, хотя в файле S его GUID может ни разу не встретиться. Это главный источник пропусков у простого `grep guid`. **[проверено на структуре]**
- Переопределения (`m_Modifications`) — это пары «объект в исходнике (fileID+guid) + путь свойства + значение/ссылка». Там тоже бывают ссылки на ассеты (`objectReference: {… guid …}`). В сцене Chop Chop через переопределение передаётся ссылка на компонент другого экземпляра: `propertyPath: _cameraManager` и `objectReference: {fileID: 1016146176}`, где `1016146176` — заглушка MonoBehaviour из другого префаба (`CC:Assets/Scenes/Examples/InventoryScene.unity`, стр. 283–285, 1568–1573). **[проверено]**
- fileID заглушки в новых файлах считается как `(fileID экземпляра XOR fileID в исходнике) & 0x7FFF…`. Для `Andre.prefab` формула сошлась. Для сцены, сохранённой в старой версии Unity (маленькие fileID), не сошлась. **Опираться на формулу нельзя.** Правильно читать `m_PrefabInstance` и `m_CorrespondingSourceObject` у заглушки. **[проверено]**

### 1.6. Варианты префабов (Prefab Variant)

Вариант — это префаб, **весь** файл которого сводится к одному корневому `PrefabInstance` без родителя. `Andre.prefab` — вариант `Townsfolk_M.prefab`:

```yaml
--- !u!1001 &3885908913776036619
PrefabInstance:
  m_Modification:
    m_TransformParent: {fileID: 0}                ← нет родителя → это вариант
    m_Modifications:
    - target: {fileID: 5054559148327267577, guid: 661c491ac76af4b429f8a8c0e6e8c762, type: 3}
      propertyPath: m_Name
      value: Andre
    - target: {fileID: 3798993931151915321, guid: 661c491ac76af4b429f8a8c0e6e8c762, type: 3}
      propertyPath: m_Materials.Array.data[0]
      objectReference: {fileID: 2100000, guid: 47cbf2e085484df40807cae147b92ff2, type: 2}
  m_SourcePrefab: {fileID: 100100000, guid: 661c491ac76af4b429f8a8c0e6e8c762, type: 3}   ← база: Townsfolk_M
```

- Ребро «Andre — вариант Townsfolk_M». Все скрипты базы (StateMachine и др.) есть и в варианте, хотя в файле варианта их GUID нет. **[проверено]**
- В новых версиях Unity (примерно с 2022.2) в `m_Modification` появились `m_AddedGameObjects`, `m_AddedComponents` и `m_RemovedGameObjects` вдобавок к старому `m_RemovedComponents`. Компоненты, добавленные в вариант или экземпляр, лежат в файле как обычные документы, а ссылка на них — в `m_AddedComponents[].addedObject`. Это видно в фикстурах `unity-yaml-bridge` (`samples/prefabs/ArenaStart.prefab`, стр. 29325+). **[проверено: формат; версия Unity — по памяти]**
- `m_RemovedComponents` означает, что компонент базы в экземпляре **удалён**. Если его не учесть, граф даст ложное «X висит здесь». **[проверено: поле есть]**

### 1.7. UnityEvent: связь «объект → метод по имени»

Кнопка в `CC:Assets/Prefabs/UI/GameplayScene/Dialogue/ButtonChoice.prefab` (стр. 121–135):

```yaml
  m_OnClick:
    m_PersistentCalls:
      m_Calls:
      - m_Target: {fileID: 1448887160}                         ← компонент в этом же файле (m_Script b29347f9…)
        m_TargetAssemblyTypeName: UIDialogueChoiceFiller, Assembly-CSharp
        m_MethodName: ButtonClicked                              ← имя метода строкой
        m_Mode: 1
```

- Это ребро **«префаб ButtonChoice вызывает `UIDialogueChoiceFiller.ButtonClicked`»**. В коде у метода вызовов нет, поэтому поиск ссылок в IDE покажет «0 использований». Переименование метода молча ломает кнопку. Для «последствий правок» это одно из самых ценных рёбер. **[проверено]**
- `m_TargetAssemblyTypeName` появился в новых версиях. В старых файлах тип берут через `m_Target → m_Script → GUID`. **[по памяти]**

### 1.8. `[SerializeReference]`

Полиморфные поля хранятся в блоке `references` с типом, записанным **строкой** (из фикстур `uymerge`, `tests/fixtures/inputs/table-with-refs.asset`):

```yaml
  references:
    version: 2
    RefIds:
    - rid: 842043826615615503
      type: {class: SmartFormatTag, ns: UnityEngine.Localization.Metadata, asm: Unity.Localization}
      data: ...
```

Переименование класса, пространства имён или сборки ломает данные. Спасает только атрибут `[MovedFrom]`. Ребро: «ассет → класс C# по полному имени». **[проверено: формат; `[MovedFrom]` — по памяти]**

### 1.9. Другие текстовые файлы с полезными связями

| Файл | Что там | Пометка |
|---|---|---|
| `*.asset` (ScriptableObject) | Документ `MonoBehaviour` с `m_Script` → класс SO; поля-ссылки на другие ассеты | [проверено] |
| `AddressableAssetsData/AssetGroups/*.asset` | `m_SerializeEntries: - m_GUID, m_Address, m_SerializedLabels` — какой ассет под каким адресом-строкой | [проверено] (`CC:Assets/AddressableAssetsData/AssetGroups/DataScriptableObjects.asset`) |
| `*.asmdef` | JSON: имя сборки, `references` по имени или `"GUID:…"` (GUID другого `.asmdef`), платформы, `defineConstraints` | [проверено] (`CC:Assets/Scripts/StateMachine/UOP1.StateMachine.asmdef`; mcp-unity `Editor/McpUnity.Editor.asmdef`) |
| `ProjectSettings/EditorBuildSettings.asset` | Список сцен в сборке (путь + GUID) — нужен для `SceneManager.LoadScene("Имя")` и по индексу | [по памяти] |
| `*.anim` | Animation Events: `m_Events: - functionName: "…"` — ещё один вызов метода по имени строкой | [по памяти] |
| `*.controller` | Animator: состояния, клипы, `StateMachineBehaviour` через `m_Script` | [по памяти] |
| `*.inputactions` | JSON Input System; часто рядом лежит сгенерированный C#-класс | [по памяти] |
| `TagManager.asset` | Теги и слои, на которые код ссылается строками (`CompareTag("Player")`) | [по памяти] |

### 1.10. Подводные камни разбора

- **Это не совсем стандартный YAML.** Заголовки `--- !u!114 &123 stripped` и собственные теги ломают обычные парсеры. Все разобранные парсеры сначала делят файл на документы по регулярному выражению `^--- !u!(\d+) &(-?\d+)( stripped)?`, а тело разбирают отдельно (`unity-yaml-bridge/src/unity-yaml-parser.ts`, стр. 102–111). **[проверено]**
- **fileID — 64-битные числа.** В JavaScript `5054559148327267577` превращается в `5054559148327267000`: точность теряется, ссылки перестают совпадать. Оба TS-парсера держат fileID строками (`unity-prefab-parser-mcp/src/parser.ts:216`, `unity-yaml-bridge/src/unity-yaml-parser.ts:493–501`). Проверил в Node: `Number('5054559148327267577')` → `5054559148327267000`. **Если ядро будет на TS/JS — только строки или BigInt.** **[проверено]**
- **Файлы бывают огромными**: 30 589 строк в одном префабе UI (`unity-yaml-bridge/samples/prefabs/ArenaStart.prefab`). Для индекса ссылок лучше потоковый построчный разбор, а полный YAML — только по запросу. **[проверено: размер]**
- В файлах могут остаться маркеры конфликтов слияния. Часть ассетов бинарные (LightingData, TerrainData, модели): их пропускаем. Папка `Library/` — кэш редактора во внутреннем формате, для нас бесполезна. **[по памяти]**
- Числовой `fileID` объекта внутри файла стабилен, но ничего не говорит человеку. Чтобы показывать агенту «Player/Body/Hat», иерархию нужно собирать через `Transform.m_Father/m_Children` (или `SceneRoots` в новых версиях). **[проверено: поля есть]**

### 1.11. Какие рёбра графа получаются из файлов

| Ребро | Откуда | Точность |
|---|---|---|
| Скрипт → на каком объекте в сцене/префабе/SO висит | `m_Script.guid` + `.meta` | высокая |
| Префаб → где вставлен (вложенный) | `PrefabInstance.m_SourcePrefab` | высокая |
| Вариант → база | корневой `PrefabInstance` с `m_TransformParent: {fileID: 0}` | высокая |
| Скрипт → «присутствует» в сцене через цепочку префабов | транзитивно по двум строкам выше, минус `m_RemovedComponents` | высокая при аккуратном обходе |
| Компонент → ассет в поле | любые `{…guid…}` в теле и в `m_Modifications[].objectReference` | высокая |
| Поле C# → данные в ассетах | имя поля в YAML = имя `[SerializeField]`/`public` поля | средняя (наследование, `[FormerlySerializedAs]`) |
| Объект → метод C# (UnityEvent) | `m_PersistentCalls.m_Calls[].m_MethodName` + тип цели | высокая |
| Ассет → класс по строке (`[SerializeReference]`) | `references.RefIds[].type` | высокая |
| Addressables: адрес-строка → ассет | группы `AddressableAssetsData` | высокая |
| Сборка → сборка | `.asmdef` | высокая |

---

## 2. Новый Unity CLI

### 2.1. Что это

- **Официальный инструмент Unity Technologies**: отдельная программа `unity` для терминала. Не входит ни в Hub, ни в редактор. **[заявлено]**
  - Блог: [Meet the Unity CLI](https://unity.com/blog/meet-the-unity-cli), 20.07.2026, приурочен к Unite Seoul 2026.
  - Документация: [docs.unity.com/en-us/hub/unity-cli](https://docs.unity.com/en-us/hub/unity-cli) — там прямо написано, что CLI **экспериментальный**.
  - [Release notes](https://docs.unity.com/en-us/unity-cli/release-notes): последняя версия — `1.0.0-beta.11` от 22.09.2026, самая ранняя на странице — `1.0.0-beta.4` от 12.08.2026. По вторичному источнику (Medium), первая публичная бета вышла 04.04.2026.
  - [Ветка на форуме Unity](https://discussions.unity.com/t/announcing-the-unity-cli-a-new-way-to-connect-your-tools-and-agents/1731104).
- **Установка**: скрипт с CDN Unity (`install.ps1` / `install.sh`, канал `beta`), позже через winget и Homebrew. Linux требует glibc 2.34+. **[заявлено]**
- **Лицензия**: CLI и Unity MCP **бесплатны, без ограничений на число одновременных пользователей** (со слов сотрудников Unity на форуме). Исходников нет: в организации Unity-Technologies на GitHub репозиторий CLI не нашёл. Отдельной открытой лицензии нет, применяются общие условия Unity. Для части команд нужен вход в аккаунт (`unity auth login`, например перед `unity pipeline install`). **[заявлено / не нашёл исходников]**
- Старый CLI внутри Unity Hub объявлен устаревшим начиная с Hub 3.18.0. **[заявлено, вторичный источник]**
- **Путаница с именами.** Есть несколько сторонних проектов с тем же названием: `RageAgainstThePixel/unity-cli` (утилита для CI), `youngwoocho02/unity-cli`, `bigdra50/unity-cli`, `akiojin/unity-cli`, а также `yucchiy/UniCli` и `hatayama/unity-cli-loop`. Все сторонние работают **через мост в открытом редакторе**. Пользователь, скорее всего, имел в виду официальный. **[проверено: список на GitHub]**

### 2.2. Три слоя возможностей

| Слой | Что умеет | Нужен ли редактор | Пометка |
|---|---|---|---|
| **Сам CLI** | установка редакторов и модулей, `projects create/verify/clean`, `auth`, `doctor`, группа `vcs`, `skill`, `mcp configure`; вывод `--format json/tsv`, понятные коды выхода | нет (кроме отдельных команд) | [заявлено] |
| **Пакетный режим** | `unity run` (batch mode, `--command` — зарегистрированная команда редактора без окна), `unity build`, `unity test` | запускает редактор сам; нужна **лицензия Unity** и установленный редактор | [заявлено] |
| **Живой редактор** | `unity command <имя>`, `unity command eval "C#"`, `eval_file`, `unity recompile`, `unity mcp` | **да**: Unity 6.0+, в проекте пакет `com.unity.pipeline` (сейчас `0.3.1-exp.1`), связь HTTP+JSON на localhost с токеном | [заявлено] |

Встроенные команды пакета pipeline — это в основном **правки**: `create_prefab`, `instantiate_prefab`, `create_prefab_variant`, `set_component_properties` и т. п. Для чтения есть `get_scene_hierarchy`, `find_assets`, `get_serialized_fields`, `get_component_properties`. **Команды «кто ссылается на ассет / зависимости» среди встроенных нет.** Такую можно получить через `eval` (`AssetDatabase.GetDependencies`) или своей командой с атрибутом `[CliCommand]`. Источник — навык-справочник [menstood/unity-pipeline-commands-skill](https://github.com/menstood/unity-pipeline-commands-skill/tree/c5e762d26395160bcb86d1c4ba17112ba5322f9b), составленный по руководству пакета 0.3 и «сверенный с живым редактором». **[заявлено автором навыка]**

### 2.3. Команды, близкие к нашей задаче

Дословно из release notes `1.0.0-beta.8` (02.09.2026) и справочника команд **[заявлено]**:

- `unity vcs affected` — «reports the impact of a change in Unity terms instead of Git terms. For example, it reports which prefabs reference a changed texture and which scenes reference those prefabs». В справочнике: «Reports which assets, assemblies, and tests a change affects. Reports a lower bound when applicable».
- `unity test --affected --since <ref>` запускает только тесты, которых касается изменение, «based on the impact graph from `unity vcs affected`».
- `unity vcs diff <path>` показывает изменения сцены или префаба по именам GameObject и компонентов, а не по fileID.
- `unity vcs blame <scene|prefab>` показывает, кто последним менял каждый GameObject и компонент (`--object`, `--field`).
- `unity vcs summarize --since <ref>` делает сводку ветки для PR.
- `unity projects verify` ищет потерянные `.meta`, дубли GUID и маркеры конфликтов. Про неё прямо сказано: «**It needs no Editor, license, or network access**».

### 2.4. Можно ли получать связи сцен и префабов без открытого редактора?

- **Живой слой — нет.** Все команды pipeline требуют запущенного редактора Unity 6+. Проекты на 2022 LTS не поддерживаются вообще. **[заявлено]**
- **Пакетный слой — да, но дорого.** Через `unity run --command` можно вызвать свою команду, которая выгрузит `AssetDatabase.GetDependencies` в JSON. Но для этого нужны установленный редактор нужной версии, лицензия и полный импорт проекта (папка `Library/`). На больших проектах это минуты и десятки минут. **[заявлено: `run --command`; оценка времени — по опыту]**
- **`vcs affected` / `diff` / `blame` — неизвестно.** Судя по описанию, эти команды читают YAML сами, без редактора: `diff` «по именам вместо fileID», `affected` даёт «нижнюю границу». Для `projects verify` отсутствие редактора указано явно, для этих команд — **нет**. JSON-схемы ответа в документации нет. **[не нашёл]**

### 2.5. Вывод: годится ли Unity CLI

- **Как основа MVP-1 — не годится.** Код закрыт, это бета, результат не описан схемой, живой режим требует Unity 6 и открытого редактора. Плюс мы не контролируем, какие связи он видит (UnityEvent? Addressables?). Внешняя зависимость от бинарника Unity в плагине Claude Code — лишний риск.
- **Как эталон для сверки — годится, и это ценно.** `unity vcs affected` — ровно наш сценарий «последствия правки ассета», сделанный самим вендором. Имеет смысл прогнать его на наших проектах из [[F-02 Набор проверок на наших проектах]] и сравнить с нашим графом.
- **Как необязательный «живой» режим после MVP — годится.** Если у пользователя открыт редактор Unity 6 с pipeline, можно спрашивать точные данные (`eval` → `AssetDatabase.GetDependencies`, иерархия сцены с учётом всех переопределений) и проверять правки агента (`recompile`, `test --affected`).
- **Что проверить руками (задача-кандидат):** поставить CLI на машину с нашим Unity-проектом, закрыть редактор и выполнить `unity vcs affected --format json`, `unity vcs diff`, `unity vcs blame`. Посмотреть: работают ли без редактора и лицензии, сколько идут, есть ли JSON, видят ли UnityEvent и Addressables.

### 2.6. Встроенный MCP Unity (AI Assistant)

MCP-сервер из пакета `com.unity.ai.assistant` Unity объявила устаревшим в пользу CLI. Пишут, что CLI быстрее, тратит меньше токенов и не требует подписки Unity AI. Режим `unity mcp` при этом остаётся поддерживаемым для агентов, которые не умеют вызывать shell. Страница: [Replace MCP server with Unity CLI](https://docs.unity.com/en-us/unity-cli/replace-mcp-server-unity-cli). **[заявлено]**

---

## 3. Существующие инструменты и MCP-серверы для Unity

Данные по GitHub сняты 2026-09-29.

### 3.1. Автономные парсеры (работают без редактора)

| Инструмент | Язык, лицензия | Что делает | Годится нам? |
|---|---|---|---|
| [socialpoint-labs/unity-yaml-parser](https://github.com/socialpoint-labs/unity-yaml-parser) (`unityparser`, ~180★, коммит `157b6a6`, 2025-02) | Python, MIT | Загружает и сохраняет YAML Unity «как это делает Unity», фильтрует документы по классу и атрибутам | Как образец. Графа ссылок нет. [проверено: README/код] |
| [yulcat/unity-yaml-bridge](https://github.com/yulcat/unity-yaml-bridge/tree/64919cf76b3c2bd3502e5d81227dc2e06c6f71c9) (`ubridge`, 9★, 2026-08) | TS/Node 20+, MIT | Сжимает префабы в компактный текст для ИИ («−92–96% токенов» [заявлено]) и собирает обратно; есть варианты, вложенные префабы и разрешение GUID через `--project` | Хороший образец разбора на TS (fileID строками, заглушки, варианты). В npm, по README, не опубликован. [проверено: код] |
| [luckynee/unity-prefab-parser-mcp](https://github.com/luckynee/unity-prefab-parser-mcp/tree/6cd4ecda76c90abf0c593fba5fcfcecb2fdaf773) (3★, 2026-05) | TS, MIT, MCP | Инструменты `init_unity_project` (кэш GUID из `.meta`), `browse_unity_project`, `list_unity_assets`, `parse_unity_file`: вывод «как в инспекторе», варианты | Показывает, **как агенту удобно отдавать** префаб. Обратных ссылок нет. [проверено: README/код] |
| [studio-boxcat/unity-assetdb](https://github.com/studio-boxcat/unity-assetdb/tree/34b191f30ceb99a01d9e3a6de2f48543f085b77b) (2026-06, перенесён в `boxcat-rust-tools`) | Rust, MIT/Apache-2.0 | Индекс `GUID ↔ путь ↔ имя` с подобъектами; `usage <guid>` — поиск файлов, ссылающихся на GUID; инкрементальное обновление через Watchman; на проекте в 20 тыс. `.meta`: холодный старт ~1 с, повтор ~120 мс | **Самый близкий по идее к нашему индексу.** Цифры скорости — ориентир. [проверено: docs; цифры — заявлено] |
| [lgarczyn/uymerge](https://github.com/lgarczyn/uymerge) (2026-07) | Rust, MIT | Структурное трёхстороннее слияние YAML Unity и форматтер «как редактор» | Для графа не нужен, но код чтения и записи полезен как образец. [проверено: README] |
| [AliMurato/UnityScriptScanner](https://github.com/AliMurato/UnityScriptScanner) (2★) | C#/.NET 8 | Roslyn + YAML: скрипты, не используемые в сценах; дамп иерархии сцен | Учебный пример связки «Roslyn + YAML». Префабы не учитывает. [проверено: README] |
| **Unity Insight** ([arXiv 2609.27585](https://arxiv.org/abs/2609.27585v1), 23.09.2026) | статья, кода нет | Постоянный индекс «код ↔ ассеты» для LLM-агентов: GUID↔путь, компонент↔скрипт, цепочки вложенных префабов, ссылки ассет→ассет; 5 инструментов `vfs_ls/glob/grep/read/refs`; работает с июля 2026 в Tuanjie Codely (китайская ветка Unity). На 28 вопросах: **−53% токенов и −52% времени** против обычного агента-исследователя | **Прямое подтверждение нашей идеи.** Слепые зоны, например `Resources.Load` по строке, авторы честно показывают, а не скрывают — перенять. [заявлено] |

Ещё один показательный случай — [taconotsandwich/unity-agentic-tools](https://github.com/taconotsandwich/unity-agentic-tools/tree/693e95ef19616dc09b8b832d2fd7504c48a509f4) (Apache-2.0). Раньше это был «CLI для чтения и правки сцен и префабов» с нативным модулем на Rust. В версии 0.7.0 модуль Rust удалили («Dropped the Rust native module»), и инструмент стал работать только через мост в редакторе. В README теперь стоит: «no longer maintained due to recent changes to Unity's Terms of Service». **[проверено]**

### 3.2. MCP-серверы и CLI-мосты в редактор

Все требуют **открытого редактора** и пакета-моста в проекте. Задуманы в первую очередь для **правок** («создай объект», «добавь компонент»), а не для анализа связей.

| Инструмент | ★ | Лицензия | Особенности | Обратные ссылки? |
|---|---|---|---|---|
| [CoplayDev/unity-mcp](https://github.com/CoplayDev/unity-mcp) (v10.0.0, 2026-06-30) | ~14,6 тыс. | MIT | Unity 2021.3–6.x, Python-сервер, 47 инструментов (`prefab_hierarchy`, `prefab_info`, `gameobject_components`, …) | Нет: в C# нет `GetDependencies` [проверено: grep] |
| [IvanMurzak/Unity-MCP](https://github.com/IvanMurzak/Unity-MCP) | ~4,4 тыс. | Apache-2.0 | Editor и Runtime, свой `unity-mcp-cli`, инструменты `assets-find`, `scene-get-data`, `gameobject-component-get`, `script-execute`, … | Нет [проверено: grep] |
| [CoderGamester/mcp-unity](https://github.com/CoderGamester/mcp-unity) | ~1,9 тыс. | MIT | Node-сервер + пакет в редакторе | Нет [проверено: grep] |
| [hatayama/unity-cli-loop](https://github.com/hatayama/unity-cli-loop), [yucchiy/UniCli](https://github.com/yucchiy/UniCli), [youngwoocho02/unity-cli](https://github.com/youngwoocho02/unity-cli) | 250–570 | MIT | CLI-мосты: компиляция, тесты, Play Mode, логи | Нет [заявлено: README] |
| Официальный `com.unity.pipeline` + Unity CLI | — | закрытый | см. раздел 2 | Нет среди встроенных, можно через `eval` |

Плагины внутри редактора для поиска ссылок тоже есть: [Unity-AssetDependencyGraph](https://github.com/Unity-Harry/Unity-AssetDependencyGraph), [Unity-MissingReferences-Hunter](https://github.com/AlexeyPerov/Unity-MissingReferences-Hunter) и многие другие. Но они рассчитаны на человека в редакторе.

### 3.3. Условия использования Unity (30.06.2026)

- Пункт 17.2 (ff) запрещает «access, invoke, query, instruct or otherwise interact with any Offering» через «AI agent … large language model, command line interface, model-context-protocol (MCP) client or server …», если это не «Authorized Agentic Access». Определения этого термина в тексте условий нет. **[проверено: текст ToS]**
- Пункт 17.2 (gg) запрещает «develop, distribute, or use any plugin, extension … that interfaces with … any Offering without Unity's prior written authorization». **[проверено: текст ToS]**
- Поддержка Unity на Reddit ответила: «Using AI tools to help you build your game is fine», а ограничения касаются облака, Asset Store и онлайн-API. В сами условия это не внесли ([Roboin, 02.07.2026](https://roboin.io/article/en/2026/07/02/unity-denies-concerns-over-ban-on-third-party-ai/)). **[заявлено]**
- **Что это значит для нас** (наше понимание, не юридическая консультация). Офлайн-разбор файлов проекта не обращается к «Offering» Unity: это файлы пользователя. Это самый безопасный путь. Официальный CLI, по логике, и есть «разрешённый» доступ. Собственный мост в редактор (свой UPM-пакет) попадает под (gg) буквально. Решение по живому режиму стоит отложить, пока Unity не уточнит формулировки.

---

## 4. Особенности C# в Unity, важные для связей

| Механизм | Где связь | Как находить | Что ломается при правке |
|---|---|---|---|
| `[SerializeField]` / `public` поля MonoBehaviour и SO | Имя поля = ключ в YAML всех сцен, префабов и SO с этим скриптом | Roslyn / tree-sitter по коду + имена ключей в документах с нужным `m_Script` | **Переименование поля молча обнуляет данные во всех ассетах**, если не добавить `[FormerlySerializedAs("старое")]`. Удаление — то же. Смена типа — частичная потеря |
| Имя класса и файла MonoBehaviour | Класс ↔ `.cs` ↔ GUID в `.meta` | правило «имя класса = имя файла» | Переименование класса без файла или перенос `.cs` без `.meta` → «Missing Script» во всех сценах |
| `UnityEvent` (кнопки, триггеры) | `m_PersistentCalls` в YAML: цель + `m_MethodName` строкой | разбор YAML → ребро «объект → метод» | Переименование или смена сигнатуры метода ломает вызов без ошибки компиляции |
| Сообщения движка: `Awake`, `Start`, `Update`, `FixedUpdate`, `LateUpdate`, `OnEnable/OnDisable`, `OnDestroy`, `OnTrigger*`/`OnCollision*` (2D и 3D), `OnMouse*`, `OnValidate`, `OnGUI`, `OnApplicationPause` и др. | Вызываются движком по имени, в коде вызовов нет | Список имён + класс наследует `MonoBehaviour` → неявное ребро «движок → метод» | Опечатка в имени = метод перестаёт вызываться. Анализатор «мёртвого кода» без этого списка даст ложные срабатывания |
| `SendMessage("Имя")`, `BroadcastMessage`, `Invoke("Имя")`, `StartCoroutine("Имя")` | Строка в коде | Строковые литералы в этих вызовах → ребро на метод по имени | Переименование метода |
| Animation Events | `.anim`: `functionName` | разбор YAML клипов | То же |
| `GetComponent<T>()`, `[RequireComponent(typeof(T))]` | Код ↔ ожидание, что на объекте есть компонент T | код + данные «какие компоненты вместе на объекте» из YAML | Удаление компонента из префаба ломает `GetComponent` в рантайме |
| `Resources.Load("путь")` | Строка ↔ файл в любой папке `Resources/` | литералы → поиск `Resources/**/путь.*` | Перенос или переименование ассета. Динамические строки не разрешить — **честно помечать как слепую зону** (так делает Unity Insight) |
| Addressables | `AssetReference` в YAML (`m_AssetGUID`) — надёжно; `Addressables.LoadAssetAsync("адрес")` — строка ↔ `m_Address` в группах | YAML + литералы в коде | Смена адреса или метки в группе |
| `SceneManager.LoadScene("Имя" / индекс)` | Строка или индекс ↔ `EditorBuildSettings.asset` | литералы + список сцен | Переименование сцены, изменение порядка |
| Теги и слои: `CompareTag("Player")`, `LayerMask.NameToLayer` | Строка ↔ `TagManager.asset` и `m_TagString` объектов | литералы + YAML | Переименование тега |
| `[SerializeReference]` | Полное имя типа строкой в YAML | YAML `RefIds[].type` | Переименование или перенос класса без `[MovedFrom]` |
| `.asmdef` | Границы сборок, ссылки между ними, платформы (Editor/рантайм), `defineConstraints` | JSON | Ссылка из рантайм-кода на Editor-сборку = ошибка сборки игры. Это же задаёт «область видимости» символов для графа кода |
| Editor-код: `[CustomEditor(typeof(T))]`, `[MenuItem]`, `[InitializeOnLoad]`, `[RuntimeInitializeOnLoadMethod]`, `[ExecuteAlways]` | Атрибуты | код | Точки входа без вызовов в коде (как сообщения движка) |

Для C# нужен обычный граф кода: Roslyn или tree-sitter-c-sharp, это тема F01-01 / F-04. Здесь важно, что **Unity добавляет к нему неявные точки входа и строковые связи, которые обычный анализатор C# не видит**.

---

## 5. Браузерные движки

| Движок | Где связи вне кода | Формат | Как скрипт привязан к объекту | Инструменты для агента |
|---|---|---|---|---|
| **Phaser** (3/4) | Без редактора — почти нигде: сцены создаются кодом. Связи «код ↔ ассет» — **строковые ключи**: `load.image('key', …)` ↔ `add.sprite(x, y, 'key')`, `scene.start('Game')`. Карты — Tiled/LDtk (JSON) [по памяти] | JS/TS, JSON | — | — |
| **Phaser Editor v5** (релиз весной 2026: v5.0.0 15.04, v5.0.1 11.05 [заявлено]) | Файлы `.scene` (JSON), user components в `.components` (JSON), asset pack JSON. **Редактор компилирует сцену в обычный код JS/TS**, поэтому связи потом видны и в коде [заявлено: docs] | JSON → сгенерированный код | Префаб — это класс. Экземпляр в сцене ссылается полем `prefabId` [проверено: описание инструментов в `phaserjs/editor-mcp-server`] | Официальный [editor-mcp-server](https://github.com/phaserjs/editor-mcp-server) (ISC), 40+ инструментов, работает **через запущенный Phaser Editor** [проверено: README] |
| **PixiJS** (v8) | Редактора нет. Связи — строковые алиасы в `Assets.add` / манифесте бандлов, JSON спрайт-листов [по памяти] | JS/TS, JSON | — | — |
| **Babylon.js** | Сцены `.babylon` (JSON) и glTF. Код ищет узлы **по имени строкой** (`scene.getMeshByName`) [по памяти] | JSON | — | — |
| **Babylon.js Editor v5** | Скрипты хранятся в метаданных узла: `node.metadata.scripts[] = {key, enabled, …}`, где `key` — путь к файлу скрипта. Декораторы связывают поля **по имени**: `@nodeFromScene("name")`, `@nodeFromDescendants`, `@soundFromScene`, `@visibleAsNumber/Entity/Asset…` [проверено: `tools/src/loading/script/apply.ts`, `tools/src/decorators/*` в BabylonJS/Editor `f1d0293`] | JSON + TS | по пути скрипта; поля — по имени узла | [не нашёл] официального MCP |
| **PlayCanvas** | Проект и сцены живут **в облачном редакторе**. Локально — через экспорт ZIP (REST `POST /api/projects/{id}/export`) [заявлено]. В сцене компонент `script` со списком скриптов по **имени** и атрибутами; ссылки на сущности — по `resource_id` (GUID) [по памяти, на реальном экспорте не проверял]. ESM-скрипты `.mjs`, атрибуты через JSDoc `@attribute` [заявлено] | JSON (облако) | по имени скрипта | Официальный [editor-mcp-server](https://github.com/playcanvas/editor-mcp-server) (MIT), работает **через открытый редактор в браузере**; сущности адресуются `resource_id` [проверено: код] |
| **Cocos Creator 3.x / COCOS 4** | `.scene` и `.prefab` — JSON-массив объектов, связь внутри файла через `{"__id__": N}`, с ассетами — через `{"__uuid__": "…"}`; у каждого ассета `.meta` с `uuid` [проверено] | JSON | **Тип компонента = сжатый UUID скрипта** (см. пример ниже) [проверено] | 04.01.2026 COCOS 4 открыт под MIT, редактор отделён (новая IDE «PinK»), курс на «MCP и агентов» [заявлено]; есть [cocos/cocos-cli](https://github.com/cocos/cocos-cli) (MIT): создание, импорт, сборка [проверено: README]; сторонние cocos-mcp-server |

**Пример Cocos** (учебный проект [cocos/cocos-tutorial-mind-your-step](https://github.com/cocos/cocos-tutorial-mind-your-step/tree/2213c0dbf9c11b4ba1458caaa25f633ff9357659), Creator 3.0). Здесь `.meta` скрипта `assets/Scipts/GameManager.ts.meta`:

```json
{ "importer": "typescript", "uuid": "326d60ed-0e36-4fc4-9e12-c13315b9801e", ... }
```

`assets/Scenes/Main.scene` — компонент этого скрипта и кнопка, которая вызывает его метод:

```json
{ "__type__": "326d6DtDjZPxJ4SwTMVuYAe",          ← первые 5 hex UUID + остаток в base64
  "node": {"__id__": 25},
  "cubePrfb": {"__uuid__": "eb7138f8-eb1f-433e-ae2c-77ff499c8e14"},   ← поле @property → префаб
  "playerCtrl": {"__id__": 24} }
{ "__type__": "cc.ClickEvent", "target": {"__id__": 25},
  "_componentId": "326d6DtDjZPxJ4SwTMVuYAe", "handler": "onStartButtonClicked" }   ← метод строкой
{ "__type__": "cc.PrefabInfo", "asset": {"__uuid__": "6cd4e8c1-b63f-4bbb-b3db-b323fa08b435"}, ... }
```

По устройству это прямой аналог Unity: GUID в `.meta`, скрипт в сцене по UUID, `ClickEvent` ≈ UnityEvent, `PrefabInstance` с `propertyOverrides`. Индексатор Unity переносится на Cocos сравнительно дёшево. **[проверено]**

**Вывод по вебу.** В Phaser и PixiJS без редактора главный тип «невидимых» связей — **строковые ключи** (ассеты, сцены, события). Их можно ловить в самом графе JS/TS по литералам. Редакторные форматы (Phaser Editor, Babylon Editor, Cocos, PlayCanvas) имеет смысл разбирать, только если команда ими пользуется. Сейчас это не подтверждено: надо спросить у пользователя.

---

## 6. Рекомендация: что делать в MVP-1, что позже

### MVP-1 (фича [[F-10 Сцены и префабы Unity]])

**Свой офлайн-индексатор текстового YAML Unity внутри нашего ядра.** Без редактора, без Unity CLI, без сторонних парсеров в зависимостях.

1. **Карта GUID.** Обойти `Assets/`, `Packages/` и `Library/PackageCache/` (только `.meta`), построить `guid → путь, тип импортёра`. Для `.cs` добавить класс: имя файла плюс пространство имён из графа кода.
2. **Документы.** Разрезать `.unity`, `.prefab`, `.asset`, `.controller`, `.anim`, `.overrideController` на документы по заголовку `--- !u!N &ID [stripped]`. fileID хранить **строками**.
3. **Рёбра первого круга** (всё из п. 1.11):
   - `m_Script` → скрипт на объекте (плюс имя GameObject через `m_GameObject → m_Name`);
   - `PrefabInstance.m_SourcePrefab` → вложенность, вариант (если `m_TransformParent: {fileID: 0}` и это корень);
   - `m_RemovedComponents`, `m_AddedComponents` — поправки к наследованию;
   - `m_PersistentCalls.m_Calls` → «объект → метод»;
   - все `{… guid …}` и `m_AssetGUID` → «ассет использует ассет»;
   - группы Addressables, `.asmdef`, `EditorBuildSettings`.
4. **Транзитивный обход.** Выдавать ответ «где висит компонент X» по цепочкам префаб → вариант → сцена.
5. **Неявные точки входа.** Список сообщений Unity и Editor-атрибутов, чтобы граф кода не считал `Update()` мёртвым кодом и показывал, «кто его вызывает» (движок).
6. **Запросы для агента** (пересекается с F-05):
   - «где используется скрипт/класс X» → сцены, префабы, SO, с путём объекта;
   - «что сломается, если переименовать поле / метод / класс / перенести файл». Поле → ассеты с данными (совет `[FormerlySerializedAs]`), метод → UnityEvent и Animation Events, класс → `[SerializeReference]` и Missing Script;
   - «что ссылается на ассет Y» (обратный индекс по GUID).
7. **Инкрементальность.** Пересчёт по изменённым файлам (время изменения / хеш). Ориентир скорости — `unity-assetdb`: ~1 с холодный старт на 20 тыс. ассетов.
8. **Честные слепые зоны.** Строки в `Resources.Load`, `Addressables.Load("адрес")`, `SendMessage` и т. п. разрешать, только если аргумент — литерал. Иначе помечать «не удалось определить».

Для браузерных игр в MVP-1: **только граф JS/TS**. Плюс, если дёшево, строковые ключи Phaser (`load.*` ↔ `add.*`/`scene.start`) как обычные строковые рёбра.

### После MVP

- **Unity CLI как эталон**: прогон `unity vcs affected` на наборе проверок F-02 и сравнение с нашим графом (см. п. 2.5, что проверить руками).
- **Живой режим через Unity CLI + pipeline** (Unity 6+): точная иерархия с переопределениями, `AssetDatabase.GetDependencies`, проверка правок агента через `recompile` / `test --affected`. Только после ясности с условиями Unity (п. 3.3).
- Полное применение переопределений (`m_Modifications`) — чтобы показывать итоговые значения полей в экземпляре, а не только сам факт связи.
- Скрипты из DLL (таблица «хеш fileID → класс»), Timeline, Input System, Shader Graph.
- Редакторные форматы веба: Cocos (формат почти как у Unity), затем Phaser Editor / Babylon Editor / PlayCanvas — по спросу команды.

### Что добавить в набор проверок F-02 для Unity

- Компонент, который висит **только** через вариант или вложенный префаб (ловит ошибку «простого grep по GUID»).
- Метод, вызываемый только из UnityEvent кнопки.
- Поле `[SerializeField]` с данными в нескольких префабах (сценарий переименования).
- Ассет, загружаемый через `AssetReference`, и ассет через `Resources.Load("…")`.
- Компонент, удалённый в экземпляре (`m_RemovedComponents`), — не должен считаться «висящим».

---

## 7. Открытые вопросы

1. Работают ли `unity vcs affected / diff / blame` без открытого редактора и лицензии? Есть ли у них JSON? Проверить руками (п. 2.5).
2. Какие версии Unity у наших проектов? Pipeline поддерживает только 6.0+.
3. Пользуется ли команда редакторами для веба (Phaser Editor, Cocos, Babylon Editor, PlayCanvas) или всё пишется кодом?
4. Нужна ли позиция юриста по пунктам 17.2 (ff)/(gg) условий Unity до работы над живым режимом?

---

## Соответствие критериям задачи

| Критерий | Раздел | Статус |
|---|---|---|
| Формат сцен, префабов, ассетов: GUID, `m_Script`, `fileID`, вложенные префабы, варианты | 1 | выполнен (на реальных файлах Chop Chop) |
| Новый Unity CLI: что умеет, связи без редактора, лицензия | 2 | выполнен частично: описан по документации. Вопрос «работает ли `vcs affected` без редактора» остаётся открытым, нужен ручной прогон (запуск запрещён условиями задачи) |
| Существующие инструменты и MCP-серверы | 3 | выполнен |
| Особенности C# в Unity | 4 | выполнен |
| Браузерные движки | 5 | выполнен. Cocos и Babylon Editor проверены по файлам и коду, PlayCanvas-сцена — по памяти |
| Рекомендация MVP-1 / позже | 6 | выполнен |
| Отчёт в `Docs/Research/05-unity-and-web-engines.md` | — | выполнен |

---

## Источники

**Unity CLI и условия Unity**
- [Meet the Unity CLI (блог, 20.07.2026)](https://unity.com/blog/meet-the-unity-cli)
- [Unity Pipeline and Unity CLI — Technical Walkthrough](https://unity.com/resources/unity-pipeline-cli-technical-walkthrough)
- [Документация Unity CLI](https://docs.unity.com/en-us/hub/unity-cli), [Use Unity CLI](https://docs.unity.com/en-us/unity-cli/use-unity-cli), [Справочник команд](https://docs.unity.com/en-us/unity-cli/unity-cli-reference), [Release notes](https://docs.unity.com/en-us/unity-cli/release-notes), [Replace MCP server](https://docs.unity.com/en-us/unity-cli/replace-mcp-server-unity-cli), [Pipeline package](https://docs.unity.com/en-us/unity-cli/unity-pipeline/unity-pipeline-package)
- [Руководство com.unity.pipeline 0.3](https://docs.unity3d.com/Packages/com.unity.pipeline@0.3/manual/index.html)
- [Анонс на форуме Unity](https://discussions.unity.com/t/announcing-the-unity-cli-a-new-way-to-connect-your-tools-and-agents/1731104)
- [Unity CLI: What Is Unity CLI? (Medium, вторичный)](https://medium.com/@lemapp09/unity-cli-what-is-unity-cli-fce004aab1c4)
- [Unity Terms of Service (ред. 30.06.2026)](https://unity.com/legal/terms-of-service), [Roboin: реакция Unity](https://roboin.io/article/en/2026/07/02/unity-denies-concerns-over-ban-on-third-party-ai/)

**Формат Unity**
- [Unity Manual: FormatDescription](https://docs.unity3d.com/Manual/FormatDescription.html)
- Chop Chop, коммит `608eac9`: [Andre.prefab](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/Prefabs/Characters/NPCs_Examples/Andre.prefab), [Townsfolk_M.prefab](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/Prefabs/Characters/NPCs_BasePrefabs/Townsfolk_M.prefab), [StateMachine.cs.meta](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/Scripts/StateMachine/Core/StateMachine.cs.meta), [ButtonChoice.prefab](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/Prefabs/UI/GameplayScene/Dialogue/ButtonChoice.prefab), [InventoryScene.unity](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/Scenes/Examples/InventoryScene.unity), [Initialization.unity](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/Scenes/Initialization.unity), [DataScriptableObjects.asset](https://github.com/UnityTechnologies/open-project-1/blob/608eac98df29cd97821a6115cd52dfb9027345b1/UOP1_Project/Assets/AddressableAssetsData/AssetGroups/DataScriptableObjects.asset)

**Инструменты**
- [Unity Insight (arXiv 2609.27585)](https://arxiv.org/abs/2609.27585v1)
- [socialpoint-labs/unity-yaml-parser](https://github.com/socialpoint-labs/unity-yaml-parser), [yulcat/unity-yaml-bridge](https://github.com/yulcat/unity-yaml-bridge), [luckynee/unity-prefab-parser-mcp](https://github.com/luckynee/unity-prefab-parser-mcp), [studio-boxcat/unity-assetdb](https://github.com/studio-boxcat/unity-assetdb), [lgarczyn/uymerge](https://github.com/lgarczyn/uymerge), [AliMurato/UnityScriptScanner](https://github.com/AliMurato/UnityScriptScanner), [taconotsandwich/unity-agentic-tools](https://github.com/taconotsandwich/unity-agentic-tools)
- [CoplayDev/unity-mcp](https://github.com/CoplayDev/unity-mcp), [IvanMurzak/Unity-MCP](https://github.com/IvanMurzak/Unity-MCP), [CoderGamester/mcp-unity](https://github.com/CoderGamester/mcp-unity), [hatayama/unity-cli-loop](https://github.com/hatayama/unity-cli-loop), [yucchiy/UniCli](https://github.com/yucchiy/UniCli), [youngwoocho02/unity-cli](https://github.com/youngwoocho02/unity-cli), [RageAgainstThePixel/unity-cli](https://github.com/RageAgainstThePixel/unity-cli), [menstood/unity-pipeline-commands-skill](https://github.com/menstood/unity-pipeline-commands-skill)

**Браузерные движки**
- Phaser: [Phaser Editor v5 Is Here](https://phaser.io/news/2026/04/phaser-editor-v5-release), [Scene compiler](https://docs.phaser.io/phaser-editor/v4/scene-editor/scene-compiler), [User Components](https://docs.phaser.io/phaser-editor/scene-editor/user-components/user-components-editor), [Prefabs](https://docs.phaser.io/phaser-editor/scene-editor/prefabs), [phaserjs/editor-mcp-server](https://github.com/phaserjs/editor-mcp-server)
- Babylon.js: [Editor — Adding scripts](https://editor.babylonjs.com/documentation/basics/adding-scripts), [BabylonJS/Editor `f1d0293`](https://github.com/BabylonJS/Editor/tree/f1d02938bb5ccb0b43d3a6d6e3809a37935fbec2/tools/src)
- PlayCanvas: [ESM Scripts](https://developer.playcanvas.com/user-manual/scripting/esm-scripts/), [Backup and Export](https://developer.playcanvas.com/user-manual/editor/projects/backup-and-export/), [REST: Export project](https://developer.playcanvas.com/user-manual/api/project-export/), [playcanvas/editor-mcp-server](https://github.com/playcanvas/editor-mcp-server)
- Cocos: [Meta files (3.8)](https://docs.cocos.com/creator/3.8/manual/en/asset/meta.html), [COCOS 4 Is Here, Fully Open Source](https://www.prnewswire.com/news-releases/cocos-4-is-here-fully-open-source-302652264.html), [cocos/cocos-cli](https://github.com/cocos/cocos-cli), [cocos-tutorial-mind-your-step](https://github.com/cocos/cocos-tutorial-mind-your-step/tree/2213c0dbf9c11b4ba1458caaa25f633ff9357659)
