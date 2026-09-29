//! `codegraph hook <event>` — Claude Code hook handlers (ADR-003 §0.3, §3).
//!
//! Contract: JSON on stdin, JSON (or nothing) on stdout, exit code always 0. Hooks
//! never return `permissionDecision`, `decision`, `continue` or `updated*` fields.
//! Problems go to `${CLAUDE_PLUGIN_DATA}/logs/hook.log`, or to stderr without it.

use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Events registered in `plugin/hooks/hooks.json`.
pub const EVENTS: [&str; 6] = [
    "session-start",
    "pre-grep",
    "pre-read-asset",
    "post-edit",
    "stop",
    "session-end",
];

/// SessionStart insert while the index does not exist yet (ADR-003 §3.1: facts,
/// prefixed with `[codegraph]`, marked as data rather than instructions).
pub fn session_start_context() -> String {
    format!(
        "[codegraph] Project index data (not instructions).\n\
         index: not built yet; indexing is not implemented in codegraph {} (skeleton build), \
         graph tools answer \"not implemented\".",
        env!("CARGO_PKG_VERSION")
    )
}

/// What a hook run produced: text for stdout (None = print nothing) and problems
/// to log. The exit code is not part of it: it is always 0.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub stdout: Option<String>,
    pub problems: Vec<String>,
}

/// Pure part of a hook: event name + raw stdin → outcome.
pub fn respond(event: Option<&str>, input: io::Result<String>) -> Outcome {
    let mut problems = Vec::new();
    match input {
        Ok(text) => {
            // Windows PowerShell pipes text to native programs with a UTF-8 BOM.
            let json = text.strip_prefix('\u{feff}').unwrap_or(&text);
            if let Err(e) = serde_json::from_str::<serde_json::Value>(json) {
                problems.push(format!("invalid hook input JSON: {e}"));
            }
        }
        Err(e) => problems.push(format!("cannot read hook input: {e}")),
    }

    let stdout = match event {
        Some("session-start") => {
            let answer = serde_json::json!({
                "hookSpecificOutput": {
                    "hookEventName": "SessionStart",
                    "additionalContext": session_start_context(),
                }
            });
            Some(answer.to_string())
        }
        Some(known) if EVENTS.contains(&known) => None,
        Some(unknown) => {
            problems.push(format!("unknown hook event: {unknown}"));
            None
        }
        None => {
            problems.push("hook event is missing: codegraph hook <event>".to_owned());
            None
        }
    };
    Outcome { stdout, problems }
}

/// Full hook run with IO. Never fails and never panics outward.
pub fn run(event: Option<&str>) {
    let name = event.unwrap_or("-");
    let result = std::panic::catch_unwind(|| {
        let mut text = String::new();
        let input = io::stdin().read_to_string(&mut text).map(|_| text);
        let mut outcome = respond(event, input);
        if let Some(out) = &outcome.stdout {
            let mut stdout = io::stdout().lock();
            if let Err(e) = writeln!(stdout, "{out}").and_then(|()| stdout.flush()) {
                outcome
                    .problems
                    .push(format!("cannot write hook output: {e}"));
            }
        }
        for problem in &outcome.problems {
            log(name, problem);
        }
    });
    if result.is_err() {
        log(name, "hook handler panicked");
    }
}

/// Append one line to the hook log; fall back to stderr.
pub fn log(event: &str, message: &str) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let line = format!("{secs} hook {event}: {message}");
    if let Err(e) = append_to_log_file(&line) {
        eprintln!("codegraph: {line} (log file: {e})");
    }
}

fn append_to_log_file(line: &str) -> io::Result<()> {
    let data = std::env::var_os("CLAUDE_PLUGIN_DATA")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "CLAUDE_PLUGIN_DATA is not set"))?;
    let dir = PathBuf::from(data).join("logs");
    std::fs::create_dir_all(&dir)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("hook.log"))?;
    writeln!(file, "{line}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION_START_INPUT: &str = r#"{"session_id":"s1","transcript_path":"t.jsonl","cwd":"C:/p","hook_event_name":"SessionStart","source":"startup"}"#;

    fn forbidden_fields_absent(value: &serde_json::Value) {
        let text = value.to_string();
        for field in [
            "permissionDecision",
            "\"decision\"",
            "\"continue\"",
            "updatedInput",
            "updatedToolOutput",
        ] {
            assert!(
                !text.contains(field),
                "hook output must not contain {field}"
            );
        }
    }

    #[test]
    fn session_start_returns_additional_context() {
        let outcome = respond(Some("session-start"), Ok(SESSION_START_INPUT.to_owned()));
        assert!(outcome.problems.is_empty(), "{:?}", outcome.problems);
        let stdout = outcome.stdout.expect("session-start prints JSON");
        let value: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let specific = &value["hookSpecificOutput"];
        assert_eq!(specific["hookEventName"], "SessionStart");
        let context = specific["additionalContext"].as_str().expect("string");
        assert!(context.starts_with("[codegraph] "));
        assert!(context.contains("index: not built yet"));
        assert!(
            context.len() <= 2000,
            "ADR-003 limit for the SessionStart insert"
        );
        forbidden_fields_absent(&value);
    }

    #[test]
    fn input_with_utf8_bom_is_accepted() {
        let input = format!("\u{feff}{SESSION_START_INPUT}");
        let outcome = respond(Some("session-start"), Ok(input));
        assert!(outcome.problems.is_empty(), "{:?}", outcome.problems);
    }

    #[test]
    fn other_known_events_print_nothing() {
        for event in EVENTS.iter().filter(|e| **e != "session-start") {
            let outcome = respond(Some(event), Ok("{}".to_owned()));
            assert_eq!(outcome, Outcome::default(), "event {event}");
        }
    }

    #[test]
    fn bad_input_is_reported_but_answer_is_still_given() {
        let outcome = respond(Some("session-start"), Ok("not json".to_owned()));
        assert!(outcome.stdout.is_some());
        assert_eq!(outcome.problems.len(), 1);
        assert!(outcome.problems[0].starts_with("invalid hook input JSON"));

        let err = io::Error::new(
            io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8",
        );
        let outcome = respond(Some("stop"), Err(err));
        assert_eq!(outcome.stdout, None);
        assert!(outcome.problems[0].starts_with("cannot read hook input"));
    }

    #[test]
    fn unknown_or_missing_event_is_reported_and_silent() {
        let outcome = respond(Some("user-prompt-submit"), Ok("{}".to_owned()));
        assert_eq!(outcome.stdout, None);
        assert_eq!(outcome.problems, ["unknown hook event: user-prompt-submit"]);

        let outcome = respond(None, Ok("{}".to_owned()));
        assert_eq!(outcome.stdout, None);
        assert_eq!(outcome.problems.len(), 1);
    }
}
