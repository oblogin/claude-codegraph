//! The real `codegraph` binary: hook exit codes and output, `--version`, stubs, and
//! an MCP session over stdio exactly as Claude Code starts it.

use std::io::Write;
use std::process::{Command, Output, Stdio};

use serde_json::{Value, json};

const BIN: &str = env!("CARGO_BIN_EXE_codegraph");

fn run(args: &[&str], stdin: &[u8], plugin_data: Option<&std::path::Path>) -> Output {
    let mut cmd = Command::new(BIN);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_remove("CLAUDE_PLUGIN_DATA");
    if let Some(dir) = plugin_data {
        cmd.env("CLAUDE_PLUGIN_DATA", dir);
    }
    let mut child = cmd.spawn().expect("spawn codegraph");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait codegraph")
}

const SESSION_START: &str = r#"{"session_id":"abc","transcript_path":"C:/t.jsonl","cwd":"C:/p","hook_event_name":"SessionStart","source":"startup"}"#;

#[test]
fn hook_session_start_prints_context_and_exits_0() {
    let out = run(&["hook", "session-start"], SESSION_START.as_bytes(), None);
    assert_eq!(out.status.code(), Some(0));
    let value: Value = serde_json::from_slice(&out.stdout).expect("stdout is JSON");
    assert_eq!(value["hookSpecificOutput"]["hookEventName"], "SessionStart");
    let context = value["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext");
    assert!(context.starts_with("[codegraph] "));
    assert!(context.contains("not built yet"));
    for forbidden in ["permissionDecision", "\"decision\"", "\"continue\""] {
        assert!(!value.to_string().contains(forbidden));
    }
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn other_hooks_are_silent_and_exit_0() {
    let input = json!({"session_id": "abc", "hook_event_name": "PreToolUse",
        "tool_name": "Grep", "tool_input": {"pattern": "Jump"}})
    .to_string();
    for event in [
        "pre-grep",
        "pre-read-asset",
        "post-edit",
        "stop",
        "session-end",
    ] {
        let out = run(&["hook", event], input.as_bytes(), None);
        assert_eq!(out.status.code(), Some(0), "{event}");
        assert!(out.stdout.is_empty(), "{event}: no output");
        assert!(out.stderr.is_empty(), "{event}: no problems");
    }
}

#[test]
fn hook_never_fails_on_bad_input_or_arguments() {
    let cases: [(&[&str], &[u8]); 6] = [
        (&["hook", "session-start"], b"not json"),
        (&["hook", "session-start"], b""),
        (&["hook", "stop"], &[0xff, 0xfe, 0x00]),
        (&["hook", "no-such-event"], b"{}"),
        (&["hook"], b"{}"),
        (&["hook", "--bogus", "--flags", "x"], b"{}"),
    ];
    for (args, stdin) in cases {
        let out = run(args, stdin, None);
        assert_eq!(out.status.code(), Some(0), "{args:?}");
        // Problems are reported (stderr without CLAUDE_PLUGIN_DATA), not swallowed.
        assert!(!out.stderr.is_empty(), "{args:?}: problem is reported");
    }
    // session-start still answers on broken input.
    let out = run(&["hook", "session-start"], b"not json", None);
    let value: Value = serde_json::from_slice(&out.stdout).expect("JSON");
    assert_eq!(value["hookSpecificOutput"]["hookEventName"], "SessionStart");
}

#[test]
fn hook_problems_go_to_plugin_data_log() {
    let dir = tempfile::tempdir().expect("temp dir");
    let out = run(&["hook", "no-such-event"], b"{}", Some(dir.path()));
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "logged to file, not stderr");
    let log = std::fs::read_to_string(dir.path().join("logs").join("hook.log")).expect("log");
    assert!(log.contains("unknown hook event: no-such-event"), "{log}");
}

#[test]
fn version_flag() {
    let out = run(&["--version"], b"", None);
    assert_eq!(out.status.code(), Some(0));
    let text = String::from_utf8(out.stdout).expect("utf-8");
    assert_eq!(
        text.trim(),
        format!("codegraph {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn hook_help_is_printed_with_exit_0() {
    let out = run(&["hook", "--help"], b"", None);
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("Usage: codegraph"));
}

#[test]
fn index_and_query_are_honest_stubs() {
    for args in [&["index"][..], &["index", "--full"], &["query", "Player"]] {
        let out = run(args, b"", None);
        assert_eq!(out.status.code(), Some(1), "{args:?}");
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(err.contains("not implemented yet"), "{args:?}: {err}");
    }
    let out = run(&["frobnicate"], b"", None);
    assert_eq!(out.status.code(), Some(2), "clap usage error");
}

#[test]
fn mcp_over_stdio() {
    let requests = [
        json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "test", "version": "0"}}}),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}),
        json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
            "params": {"name": "impact", "arguments": {"diff": "working"}}}),
    ];
    let stdin: String = requests.iter().map(|r| format!("{r}\n")).collect();
    let out = run(&["mcp"], stdin.as_bytes(), None);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let answers: Vec<Value> = String::from_utf8(out.stdout)
        .expect("utf-8")
        .lines()
        .map(|l| serde_json::from_str(l).expect("JSON-RPC line"))
        .collect();
    let by_id = |id: i64| {
        answers
            .iter()
            .find(|a| a["id"] == id)
            .unwrap_or_else(|| panic!("answer {id} in {answers:?}"))
    };
    let init = &by_id(1)["result"];
    assert_eq!(init["serverInfo"]["name"], "codegraph");
    assert!(
        init["instructions"]
            .as_str()
            .expect("instructions")
            .starts_with("codegraph: local code graph + memory")
    );
    let mut names: Vec<&str> = by_id(2)["result"]["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    names.sort_unstable();
    assert_eq!(
        names,
        ["explore", "impact", "inspect", "recall", "refs", "remember"]
    );
    let call = &by_id(3)["result"];
    assert_eq!(call["isError"], true);
    assert!(
        call["content"][0]["text"]
            .as_str()
            .expect("text")
            .contains("impact: not implemented yet")
    );
}
