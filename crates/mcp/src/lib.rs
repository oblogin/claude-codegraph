//! `codegraph-mcp` — the MCP server `cg` over stdio on top of `codegraph-query`:
//! the six tools of ADR-003, handshake `instructions`, answer budgets.

pub mod params;
mod server;

pub use server::{ALWAYS_LOAD_META_KEY, CodegraphServer, INSTRUCTIONS, TOOL_NAMES, run_stdio};
