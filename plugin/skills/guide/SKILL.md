---
name: guide
description: How to use codegraph for multi-file refactors, renames touching Unity assets, bug hunts with past context, and recording or promoting knowledge
---

# codegraph guide

codegraph keeps a local graph of this repository (code, Unity scenes and prefabs, docs) and a memory of past work. Tools of the MCP server `cg`: `explore`, `inspect`, `refs`, `impact`, `recall`, `remember`.

> Status: skeleton build. The tools answer "not implemented yet"; fall back to grep and read.

## Workflows

- **Find your way:** `explore` first (a query, or none for a repo map), then `inspect` the symbol, file or asset you need.
- **Rename or refactor:** `impact` with the symbols before editing. Depth 1 breaks, depth 2 is likely affected. Unity links (UnityEvent, serialized fields, prefabs) break silently: keep method names referenced by UnityEvents, use `[FormerlySerializedAs]` for renamed serialized fields. After the edits run `impact` with `diff`.
- **Bug hunt:** `recall` before re-solving (journal, project knowledge, global bases): index first, then `mode=get` by id. `refs` shows who calls the suspect code, including scenes and prefabs.
- **Record:** `remember` decisions, gotchas and dead ends. `level: project` creates an uncommitted draft; a human commits it. Global bases are written only by `/codegraph:promote`.

## Reading answers

- The first line is the index status and the answer id (`ans:…`).
- Link labels: `[E]` extracted, `[I]` inferred, `[A]` ambiguous (candidates, not facts); a source such as `[E unity]` or `[I git]` when it is not the parser.
- An empty answer lists `blind spots:` (file not indexed, dynamic string call, known language gap). Read them, then grep.
