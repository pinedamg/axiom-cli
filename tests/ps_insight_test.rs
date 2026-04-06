mod common;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;
use std::fs;

#[test]
fn test_ps_high_cpu_insight() {
    let mut session = common::setup_session();
    let command = "ps aux";
    let context = IntentContext {
        last_message: "check processes".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = fs::read_to_string("tests/fixtures/ps_aux_raw.txt")
        .expect("Failed to load ps aux fixture");

    for line in raw_output.lines() {
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }

    let summaries = session.engine.flush_summaries();
    
    let mut found_high_cpu_insight = false;
    for summary in summaries {
        if summary.contains("High CPU load detected") && summary.contains("rustc") {
            found_high_cpu_insight = true;
            break;
        }
    }
    
    assert!(found_high_cpu_insight, "Should detect high CPU for rustc");
}

#[test]
fn test_ps_kernel_cleanup() {
    let mut session = common::setup_session();
    let command = "ps aux";
    let context = IntentContext {
        last_message: "list processes".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = fs::read_to_string("tests/fixtures/ps_aux_raw.txt")
        .expect("Failed to load ps aux fixture");

    for line in raw_output.lines() {
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }

    let summaries = session.engine.flush_summaries();
    
    let mut found_kernel_summary = false;
    for summary in summaries {
        if summary.contains("Kernel Workers: [kworker]") && summary.contains("count: 2") {
            found_kernel_summary = true;
            break;
        }
    }
    
    assert!(found_kernel_summary, "Should cleanup and group kernel worker names");
}
