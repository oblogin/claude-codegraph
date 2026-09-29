//! `codegraph` — one binary for the CLI, the MCP server and the Claude Code hooks
//! (ADR-004, ADR-005): `index`, `query`, `mcp`, `hook <event>`.

mod hook;

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "codegraph",
    version,
    about = "Local code graph and memory for Claude Code agents"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Build or update the index of the current project
    Index {
        /// Rebuild everything instead of updating changed files
        #[arg(long)]
        full: bool,
        /// Update only these files
        #[arg(long, num_args = 1.., value_name = "PATH")]
        paths: Vec<PathBuf>,
    },
    /// Query the graph from the command line
    Query {
        /// Query text
        #[arg(required = true, num_args = 1..)]
        query: Vec<String>,
    },
    /// Serve MCP over stdio (started by Claude Code from plugin/.mcp.json)
    Mcp,
    /// Handle a Claude Code hook event: JSON on stdin and stdout, always exits 0
    Hook {
        /// session-start | pre-grep | pre-read-asset | post-edit | stop | session-end
        #[arg(
            value_name = "EVENT",
            trailing_var_arg = true,
            allow_hyphen_values = true
        )]
        args: Vec<String>,
    },
}

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().collect();
    let cli = match Cli::try_parse_from(&args) {
        Ok(cli) => cli,
        // A hook must never fail: exit code 2 would block a tool call (ADR-003 §0.3).
        // Help and version requests (exit code 0) are printed as usual.
        Err(e) if is_hook_invocation(&args) && e.exit_code() != 0 => {
            hook::log("-", &format!("cannot parse hook arguments: {e}"));
            return ExitCode::SUCCESS;
        }
        Err(e) => e.exit(),
    };
    match cli.command {
        Command::Index { full, paths } => {
            let scope = if full {
                "full rebuild".to_owned()
            } else if paths.is_empty() {
                "incremental update".to_owned()
            } else {
                format!("update of {} path(s)", paths.len())
            };
            not_implemented(&format!("index ({scope})"))
        }
        Command::Query { query } => not_implemented(&format!("query \"{}\"", query.join(" "))),
        Command::Mcp => match codegraph_mcp::run_stdio() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("codegraph mcp: {e:#}");
                ExitCode::FAILURE
            }
        },
        Command::Hook { args } => {
            hook::run(args.first().map(String::as_str));
            ExitCode::SUCCESS
        }
    }
}

fn is_hook_invocation(args: &[OsString]) -> bool {
    args.get(1).is_some_and(|a| a == "hook")
}

fn not_implemented(what: &str) -> ExitCode {
    eprintln!(
        "codegraph {what}: not implemented yet in codegraph {} (skeleton build)",
        env!("CARGO_PKG_VERSION")
    );
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Command, clap::Error> {
        Cli::try_parse_from(args).map(|cli| cli.command)
    }

    #[test]
    fn parses_index() {
        assert!(matches!(
            parse(&["codegraph", "index"]).unwrap(),
            Command::Index { full: false, ref paths } if paths.is_empty()
        ));
        match parse(&["codegraph", "index", "--full", "--paths", "a.cs", "b.cs"]).unwrap() {
            Command::Index { full, paths } => {
                assert!(full);
                assert_eq!(paths, [PathBuf::from("a.cs"), PathBuf::from("b.cs")]);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn parses_query_and_requires_text() {
        match parse(&["codegraph", "query", "Player", "Jump"]).unwrap() {
            Command::Query { query } => assert_eq!(query, ["Player", "Jump"]),
            other => panic!("unexpected {other:?}"),
        }
        assert!(parse(&["codegraph", "query"]).is_err());
    }

    #[test]
    fn parses_mcp() {
        assert!(matches!(
            parse(&["codegraph", "mcp"]).unwrap(),
            Command::Mcp
        ));
    }

    #[test]
    fn parses_hook_with_any_arguments() {
        for (argv, event) in [
            (
                vec!["codegraph", "hook", "session-start"],
                Some("session-start"),
            ),
            (vec!["codegraph", "hook"], None),
            (
                vec!["codegraph", "hook", "stop", "--extra", "x"],
                Some("stop"),
            ),
            (vec!["codegraph", "hook", "--weird"], Some("--weird")),
        ] {
            match parse(&argv).unwrap() {
                Command::Hook { args } => {
                    assert_eq!(args.first().map(String::as_str), event, "{argv:?}")
                }
                other => panic!("unexpected {other:?}"),
            }
        }
    }

    #[test]
    fn version_and_unknown_commands() {
        let e = parse(&["codegraph", "--version"]).unwrap_err();
        assert_eq!(e.kind(), clap::error::ErrorKind::DisplayVersion);
        assert!(parse(&["codegraph", "frobnicate"]).is_err());
        assert!(parse(&["codegraph"]).is_err());
    }

    #[test]
    fn detects_hook_invocation() {
        let args = |v: &[&str]| v.iter().map(OsString::from).collect::<Vec<_>>();
        assert!(is_hook_invocation(&args(&["codegraph", "hook", "x"])));
        assert!(!is_hook_invocation(&args(&["codegraph", "mcp"])));
        assert!(!is_hook_invocation(&args(&["codegraph"])));
    }

    #[test]
    fn cli_definition_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
