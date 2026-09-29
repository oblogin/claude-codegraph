//! Contract of the MCP server `cg` against ADR-003: handshake instructions, the six
//! tools, their annotations, description limits and the stub answers.

use std::collections::BTreeSet;

use codegraph_mcp::{ALWAYS_LOAD_META_KEY, CodegraphServer, INSTRUCTIONS, TOOL_NAMES};
use rmcp::ServerHandler;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// The ```text block of ADR-003 §2 — the source of truth for `instructions`.
fn adr_instructions() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../Docs/Tasks/Adr/ADR-003 Интерфейс для агента.md"
    );
    let adr = std::fs::read_to_string(path).expect("ADR-003 is readable");
    let adr = adr.replace("\r\n", "\n");
    let section = adr
        .split("### 2. Текст `instructions`")
        .nth(1)
        .expect("ADR-003 has section 2");
    let block = section
        .split("```text\n")
        .nth(1)
        .and_then(|rest| rest.split("\n```").next())
        .expect("section 2 has a text block");
    block.to_owned()
}

#[test]
fn instructions_match_adr_003_verbatim() {
    assert_eq!(INSTRUCTIONS, adr_instructions());
    let info = CodegraphServer::new().get_info();
    assert_eq!(info.instructions.as_deref(), Some(INSTRUCTIONS));
    assert_eq!(info.server_info.name, "codegraph");
    assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
    assert!(info.capabilities.tools.is_some());
    assert!(
        INSTRUCTIONS.chars().count() <= 2048,
        "Claude Code truncates at 2048"
    );
}

#[test]
fn instructions_name_only_existing_tools() {
    let names: BTreeSet<String> = CodegraphServer::new()
        .tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    let select = INSTRUCTIONS
        .split("select:")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("select: list");
    for full in select.split(',') {
        let tool = full
            .strip_prefix("mcp__plugin_codegraph_cg__")
            .unwrap_or_else(|| panic!("{full}: plugin `codegraph`, server `cg`"));
        assert!(names.contains(tool), "{tool} is not served");
    }
}

#[test]
fn serves_the_six_tools_of_adr_003() {
    let tools = CodegraphServer::new().tools();
    let names: BTreeSet<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert_eq!(names, TOOL_NAMES.into_iter().collect());
}

#[test]
fn tool_annotations_and_always_load() {
    for tool in CodegraphServer::new().tools() {
        let ann = tool.annotations.as_ref().expect("annotations");
        let read_only = tool.name != "remember";
        assert_eq!(ann.read_only_hint, Some(read_only), "{}", tool.name);
        assert_eq!(ann.open_world_hint, Some(false), "{}", tool.name);

        let always_load = tool
            .meta
            .as_ref()
            .and_then(|m| m.0.get(ALWAYS_LOAD_META_KEY))
            .cloned();
        if tool.name == "explore" {
            assert_eq!(always_load, Some(Value::Bool(true)));
        } else {
            assert_eq!(always_load, None, "{} loads lazily", tool.name);
        }
    }
}

#[test]
fn descriptions_fit_adr_003_limits() {
    for tool in CodegraphServer::new().tools() {
        let description = tool.description.as_deref().expect("description");
        assert!(
            description.chars().count() <= 350,
            "{}: description is {} chars",
            tool.name,
            description.chars().count()
        );
        assert!(description.is_ascii(), "{}: English only", tool.name);

        let schema = Value::Object((*tool.input_schema).clone());
        let props = schema["properties"].as_object().expect("properties");
        assert!(!props.is_empty(), "{}: parameters", tool.name);
        for (param, spec) in props {
            let d = spec["description"]
                .as_str()
                .unwrap_or_else(|| panic!("{}.{param}: description", tool.name));
            assert!(
                d.chars().count() <= 80,
                "{}.{param}: {} chars",
                tool.name,
                d.chars().count()
            );
            assert!(d.is_ascii(), "{}.{param}: English only", tool.name);
        }
        assert!(
            schema.get("$defs").is_none(),
            "{}: schema must be self-contained",
            tool.name
        );
    }
}

#[test]
fn schemas_carry_the_main_parameters() {
    let expect: [(&str, &[&str], &[&str]); 6] = [
        (
            "explore",
            &["query", "scope", "kinds", "detail", "budget"],
            &[],
        ),
        (
            "inspect",
            &["target", "view", "neighbors", "budget"],
            &["target"],
        ),
        (
            "refs",
            &[
                "target",
                "direction",
                "depth",
                "edges",
                "include_ambiguous",
                "budget",
            ],
            &["target"],
        ),
        (
            "impact",
            &[
                "symbols",
                "files",
                "diff",
                "depth",
                "tests",
                "summary_only",
                "budget",
            ],
            &[],
        ),
        (
            "recall",
            &[
                "query", "mode", "ids", "anchor", "window", "scope", "filters", "limit",
            ],
            &[],
        ),
        (
            "remember",
            &[
                "type",
                "title",
                "body",
                "level",
                "bind",
                "tags",
                "supersedes",
                "feedback",
            ],
            &[],
        ),
    ];
    let tools = CodegraphServer::new().tools();
    for (name, params, required) in expect {
        let tool = tools.iter().find(|t| t.name == name).expect(name);
        let schema = Value::Object((*tool.input_schema).clone());
        let props: BTreeSet<&str> = schema["properties"]
            .as_object()
            .expect("properties")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(props, params.iter().copied().collect(), "{name}");
        let req: BTreeSet<&str> = schema
            .get("required")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        assert_eq!(req, required.iter().copied().collect(), "{name} required");
    }
}

/// One JSON-RPC exchange over an in-memory stdio-like transport.
struct Client {
    write: tokio::io::DuplexStream,
    read: BufReader<tokio::io::DuplexStream>,
}

impl Client {
    async fn send(&mut self, message: Value) {
        let mut line = message.to_string();
        line.push('\n');
        self.write.write_all(line.as_bytes()).await.expect("write");
    }

    async fn receive(&mut self) -> Value {
        let mut line = String::new();
        self.read.read_line(&mut line).await.expect("read");
        serde_json::from_str(&line).unwrap_or_else(|e| panic!("bad JSON {line:?}: {e}"))
    }
}

#[tokio::test]
async fn protocol_initialize_list_and_call() {
    use rmcp::ServiceExt;

    let (client_write, server_read) = tokio::io::duplex(64 * 1024);
    let (server_write, client_read) = tokio::io::duplex(64 * 1024);
    let server = tokio::spawn(async move {
        let running = CodegraphServer::new()
            .serve((server_read, server_write))
            .await
            .expect("server starts");
        running.waiting().await.expect("server stops cleanly");
    });
    let mut client = Client {
        write: client_write,
        read: BufReader::new(client_read),
    };

    client
        .send(
            json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "test", "version": "0"}
            }}),
        )
        .await;
    let init = client.receive().await;
    assert_eq!(init["id"], 1);
    assert_eq!(init["result"]["instructions"], INSTRUCTIONS);
    assert_eq!(init["result"]["serverInfo"]["name"], "codegraph");

    client
        .send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))
        .await;
    client
        .send(json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}))
        .await;
    let list = client.receive().await;
    let tools = list["result"]["tools"].as_array().expect("tools");
    assert_eq!(tools.len(), 6);
    let explore = tools
        .iter()
        .find(|t| t["name"] == "explore")
        .expect("explore");
    assert_eq!(explore["_meta"][ALWAYS_LOAD_META_KEY], true);

    for (id, name, args) in [
        (3, "explore", json!({})),
        (
            4,
            "refs",
            json!({"target": "Player", "depth": 2, "edges": ["calls"]}),
        ),
        (
            5,
            "remember",
            json!({"type": "gotcha", "title": "t", "body": "b"}),
        ),
    ] {
        client
            .send(json!({"jsonrpc": "2.0", "id": id, "method": "tools/call",
                "params": {"name": name, "arguments": args}}))
            .await;
        let answer = client.receive().await;
        assert_eq!(answer["id"], id);
        assert_eq!(
            answer["result"]["isError"], true,
            "{name}: stub is an error"
        );
        let text = answer["result"]["content"][0]["text"]
            .as_str()
            .expect("text");
        assert!(text.starts_with("index: "), "{name}: status line first");
        assert!(
            text.contains(&format!("{name}: not implemented yet")),
            "{text}"
        );
    }

    // Wrong parameters are rejected by the schema, not silently accepted.
    client
        .send(json!({"jsonrpc": "2.0", "id": 6, "method": "tools/call",
            "params": {"name": "refs", "arguments": {"depth": 2}}}))
        .await;
    let answer = client.receive().await;
    let text = answer.to_string();
    assert!(
        answer["result"]["isError"] == true || answer.get("error").is_some(),
        "{text}"
    );
    assert!(text.contains("target"), "{text}");

    drop(client);
    server.await.expect("server task");
}
