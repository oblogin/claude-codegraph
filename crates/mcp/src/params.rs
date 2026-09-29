//! Input parameters of the six agent tools (ADR-003 §1). The field doc comments are
//! the parameter descriptions the agent sees: English, at most 80 characters each.
//! Enums are inlined into the schema so every tool schema stays self-contained.

use schemars::JsonSchema;
use serde::Deserialize;

// `explore` (ADR-003 §1.1).
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ExploreParams {
    /// Text, name, path#Symbol or path; omit for a ranked map of the repo
    pub query: Option<String>,
    /// Folder or module to search in, e.g. Assets/Scripts/UI
    pub scope: Option<String>,
    /// Restrict results to these layers
    pub kinds: Option<Vec<ExploreKind>>,
    /// map | signatures (default) | snippets (up to 8 code lines for the top 3)
    pub detail: Option<ExploreDetail>,
    /// Answer budget in tokens (default 1500, max 6000)
    pub budget: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ExploreKind {
    Code,
    Asset,
    Doc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ExploreDetail {
    Map,
    Signatures,
    Snippets,
}

// `inspect` (ADR-003 §1.2).
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct InspectParams {
    /// Symbol id, file path, asset path (.unity/.prefab/.asset), GUID or doc.md#heading
    pub target: String,
    /// outline (files, assets) | signature (symbols) | body | full
    pub view: Option<InspectView>,
    /// Include first-circle links grouped by kind (default true)
    pub neighbors: Option<bool>,
    /// Answer budget in tokens (default 1200, max 8000)
    pub budget: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum InspectView {
    Outline,
    Signature,
    Body,
    Full,
}

// `refs` (ADR-003 §1.3; edge groups follow ADR-006 §1).
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct RefsParams {
    /// Symbol id, file path, asset path or GUID
    pub target: String,
    /// in: who uses target (default) | out: what target uses
    pub direction: Option<RefsDirection>,
    /// Hops to follow, 1-3 (default 1)
    #[schemars(range(min = 1, max = 3))]
    pub depth: Option<u8>,
    /// Link groups to follow; default all except co_change
    pub edges: Option<Vec<EdgeGroup>>,
    /// List ambiguous candidates [A] instead of only counting them (default false)
    pub include_ambiguous: Option<bool>,
    /// Answer budget in tokens (default 1500, max 6000)
    pub budget: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RefsDirection {
    In,
    Out,
}

// Groups of edge kinds accepted by `refs.edges` (ADR-006 §1, last column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum EdgeGroup {
    Calls,
    Imports,
    Inherits,
    References,
    UnityScript,
    UnityEvent,
    PrefabNesting,
    AssetRef,
    SerializedField,
    DocMention,
    CoChange,
}

// `impact` (ADR-003 §1.4): one of `symbols`, `files`, `diff`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ImpactParams {
    /// Symbols about to change (ids or path#Symbol)
    pub symbols: Option<Vec<String>>,
    /// Files about to change or already changed
    pub files: Option<Vec<String>>,
    /// working (default) | staged | <base>..<head>
    pub diff: Option<String>,
    /// 1 breaks, 2 likely affected, 3 may affect (default 2)
    #[schemars(range(min = 1, max = 3))]
    pub depth: Option<u8>,
    /// Report tests: changed / stale / none (default true)
    pub tests: Option<bool>,
    /// Short summary only, about 300 tokens
    pub summary_only: Option<bool>,
    /// Answer budget in tokens (default 2000, max 8000)
    pub budget: Option<u32>,
}

// `recall` (ADR-003 §1.5).
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct RecallParams {
    /// Search text (mode=index)
    pub query: Option<String>,
    /// index (default): list matches | get: full records by ids | timeline
    pub mode: Option<RecallMode>,
    /// Record ids for mode=get, e.g. K-3f9a, J-0412
    pub ids: Option<Vec<String>>,
    /// Record id to centre the timeline on (mode=timeline)
    pub anchor: Option<String>,
    /// Records before and after the anchor (mode=timeline)
    pub window: Option<u32>,
    /// journal | project | global | all (default)
    pub scope: Option<RecallScope>,
    /// Narrow by type, tags, bound symbol or file, date, attached bases
    pub filters: Option<RecallFilters>,
    /// Max records (default 20)
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RecallMode {
    Index,
    Get,
    Timeline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RecallScope {
    Journal,
    Project,
    Global,
    All,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[schemars(inline)]
pub struct RecallFilters {
    /// Record types, e.g. decision, gotcha, dead_end
    #[serde(rename = "type")]
    pub types: Option<Vec<String>>,
    /// Technology tags
    pub tags: Option<Vec<String>>,
    /// Only records bound to this symbol or file
    pub bound: Option<String>,
    /// Only records since this date (YYYY-MM-DD)
    pub since: Option<String>,
    /// Global bases to search (scope=global), e.g. me, ivan
    pub bases: Option<Vec<String>>,
}

// `remember` (ADR-003 §1.6): write mode (`type`, `title`, `body`, …) or `feedback`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct RememberParams {
    /// Record type (write mode)
    #[serde(rename = "type")]
    pub kind: Option<RecordType>,
    /// Short title (write mode)
    pub title: Option<String>,
    /// Record text, up to 2000 characters (write mode)
    pub body: Option<String>,
    /// journal (default) | project: uncommitted draft of project knowledge
    pub level: Option<RecordLevel>,
    /// Symbol ids and files the record is about
    pub bind: Option<Vec<String>>,
    /// Technology tags
    pub tags: Option<Vec<String>>,
    /// Id of the record this one replaces
    pub supersedes: Option<String>,
    /// Rate a past answer (ans:A-7c4) or record instead of writing one
    pub feedback: Option<Feedback>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RecordType {
    Decision,
    Gotcha,
    DeadEnd,
    ProblemSolution,
    TradeOff,
    Fact,
    Note,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RecordLevel {
    Journal,
    Project,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(inline)]
pub struct Feedback {
    /// Answer id (ans:A-7c4) or record id (K-3f9a)
    pub target: String,
    /// useful | dead_end | corrected
    pub verdict: Verdict,
    /// Optional comment
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum Verdict {
    Useful,
    DeadEnd,
    Corrected,
}
