//! MCP server `cg`: handshake `instructions` and the six agent tools of ADR-003.
//! Tool bodies are stubs until the query layer lands (F-04, F-05, MVP-2).
//!
//! Structure follows ue-xray `crates/xray-mcp/src/server.rs` (see THIRD_PARTY.md).

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, Implementation, Meta, ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, ServiceExt, tool, tool_handler, tool_router};

use crate::params::{
    ExploreParams, ImpactParams, InspectParams, RecallParams, RefsParams, RememberParams,
};

/// Text of `instructions` in the `initialize` answer — ADR-003 §2, verbatim.
/// Claude Code truncates it at 2048 characters.
pub const INSTRUCTIONS: &str = "\
codegraph: local code graph + memory for this repo (C# with Unity scenes/prefabs, JS/TS, Go, Rust, Markdown/PDF). The index refreshes before each query; every answer starts with an index status line.
- Where/what/how questions about code: call explore first, instead of grep+read chains.
- Before changing a public method, signature, serialized field or anything Unity calls: call impact. It sees prefab/scene/UnityEvent links grep misses.
- refs: who uses X / what X uses. inspect: outline of one symbol, file or asset.
- Before re-solving a problem: recall (journal, project knowledge, global bases). Index first, then mode=get by id.
- Decisions, gotchas, dead ends: remember. Project knowledge becomes an uncommitted draft.
Load all at once: ToolSearch \"select:mcp__plugin_codegraph_cg__inspect,mcp__plugin_codegraph_cg__refs,mcp__plugin_codegraph_cg__impact,mcp__plugin_codegraph_cg__recall,mcp__plugin_codegraph_cg__remember\".
Labels: E extracted, I inferred, A ambiguous (candidates, not facts). Empty result: read blind spots, then grep.";

/// Tool names in the order ADR-003 lists them.
pub const TOOL_NAMES: [&str; 6] = ["explore", "inspect", "refs", "impact", "recall", "remember"];

/// `_meta` key that makes Claude Code load a tool schema at session start (ADR-003).
pub const ALWAYS_LOAD_META_KEY: &str = "anthropic/alwaysLoad";

fn always_load() -> Meta {
    let mut meta = Meta::new();
    meta.0.insert(
        ALWAYS_LOAD_META_KEY.to_owned(),
        serde_json::Value::Bool(true),
    );
    meta
}

/// Stub answer: the first line is the index status (ADR-003 §1, common rules), the
/// last one says why the answer is empty. Marked as a tool error so the agent does
/// not mistake it for an empty result and falls back to grep and read.
fn not_implemented(tool: &str) -> CallToolResult {
    let version = env!("CARGO_PKG_VERSION");
    CallToolResult::error(vec![Content::text(format!(
        "index: not built\n\
         {tool}: not implemented yet in codegraph {version} (skeleton build).\n\
         blind spots: no code graph yet; use grep and read instead."
    ))])
}

#[derive(Clone)]
pub struct CodegraphServer {
    tool_router: ToolRouter<Self>,
}

impl Default for CodegraphServer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodegraphServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Tools as announced in `tools/list`.
    pub fn tools(&self) -> Vec<rmcp::model::Tool> {
        self.tool_router.list_all()
    }
}

#[tool_router]
impl CodegraphServer {
    #[tool(
        name = "explore",
        description = "Entry point for where/what/how questions about this repo: code symbols, Unity assets, docs. Use it instead of grep+read chains. Without query returns a ranked map of the repo. Compact text; the first line is the index status.",
        annotations(read_only_hint = true, open_world_hint = false),
        meta = always_load()
    )]
    fn explore(&self, Parameters(_p): Parameters<ExploreParams>) -> CallToolResult {
        not_implemented("explore")
    }

    #[tool(
        name = "inspect",
        description = "One symbol, file, Unity asset or doc section: signature or outline without bodies, or the body; plus first-circle links (callers, callees, overrides, tests, Unity usages, notes).",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn inspect(&self, Parameters(_p): Parameters<InspectParams>) -> CallToolResult {
        not_implemented("inspect")
    }

    #[tool(
        name = "refs",
        description = "Who uses X (direction=in) or what X uses (out), including Unity scenes, prefabs, UnityEvents and docs. Every link carries a label: [E] extracted, [I] inferred, [A] ambiguous (candidates only).",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn refs(&self, Parameters(_p): Parameters<RefsParams>) -> CallToolResult {
        not_implemented("refs")
    }

    #[tool(
        name = "impact",
        description = "Before changing a public symbol, signature, serialized field or a method Unity calls, or after edits (git diff): what breaks (depth 1), what is likely affected (2), Unity assets, tests, co-changed files, risk, blind spots.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn impact(&self, Parameters(_p): Parameters<ImpactParams>) -> CallToolResult {
        not_implemented("impact")
    }

    #[tool(
        name = "recall",
        description = "Past experience before re-solving a problem: agent journal, project knowledge, global bases. mode=index lists matches (id, date, type, title); then mode=get with ids for full records; mode=timeline around an anchor.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn recall(&self, Parameters(_p): Parameters<RecallParams>) -> CallToolResult {
        not_implemented("recall")
    }

    #[tool(
        name = "remember",
        description = "Record a decision, gotcha, dead end or solution (level journal, or project: an uncommitted draft), or rate a past answer or record with feedback. Never writes to global bases.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    fn remember(&self, Parameters(_p): Parameters<RememberParams>) -> CallToolResult {
        not_implemented("remember")
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for CodegraphServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("codegraph", env!("CARGO_PKG_VERSION")))
            .with_instructions(INSTRUCTIONS)
    }
}

/// Serve MCP over stdin/stdout until the client disconnects (`codegraph mcp`).
pub fn run_stdio() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        let service = CodegraphServer::new()
            .serve(rmcp::transport::stdio())
            .await?;
        service.waiting().await?;
        Ok(())
    })
}
