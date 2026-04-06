mod common;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;

#[test]
fn test_cargo_noise_reduction() {
    let mut session = common::setup_session();
    let command = "cargo build";
    let context = IntentContext {
        last_message: "compile the project".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
   Downloading crates...
   Compiling libc v0.2.150
   Compiling serde v1.0.193
   Compiling axiom v0.1.0 (/home/user/axiom)
warning: unused variable: `x`
  --> src/main.rs:10:9
    Finished dev [unoptimized + debuginfo] target(s) in 1.2s
    ";

    let mut lines_printed = 0;
    let mut warning_shown = false;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context) {
            lines_printed += 1;
            if processed.contains("warning: unused variable") {
                warning_shown = true;
            }
        }
    }

    assert!(warning_shown, "Local warnings must always be shown");
    assert!(lines_printed < 5, "Most dependency logs should be collapsed");
}

#[test]
fn test_npm_deprecation_collapse() {
    let mut session = common::setup_session();
    let command = "npm install";
    let context = IntentContext {
        last_message: "install dependencies".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
npm WARN deprecated inflight@1.0.6: This module is not supported, please use lru-cache instead.
npm WARN deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported.
+ lodash@4.17.21
added 1 package in 2s
    ";

    let mut deprecation_shown = false;
    let mut summary_shown = false;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context) {
            if processed.contains("deprecated") {
                deprecation_shown = true;
            }
            if processed.contains("added 1 package") {
                summary_shown = true;
            }
        }
    }

    assert!(!deprecation_shown, "Deprecation warnings should be collapsed by default");
    assert!(summary_shown, "Final install summary must be shown");
}
