mod common;
use axiom::IntentContext;
use std::fs;

fn test_ls_insight_for_fixture(fixture_path: &str, expected_insight: &str) {
    let mut session = common::setup_session();
    let command = "ls -la";
    let context = IntentContext {
        last_message: "list files".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = fs::read_to_string(fixture_path)
        .expect(&format!("Failed to load fixture: {}", fixture_path));

    for line in raw_output.lines() {
        session.engine.process_line(line, command, &context);
    }

    let summaries = session.engine.flush_summaries();
    
    let mut found_insight = false;
    for summary in summaries {
        if summary.contains("Semantic Insight:") && summary.contains(expected_insight) {
            found_insight = true;
            break;
        }
    }
    
    assert!(found_insight, "Should detect '{}' project insight from fixture {}", expected_insight, fixture_path);
}

#[test]
fn test_ls_rust_insight() {
    test_ls_insight_for_fixture("tests/fixtures/ls_rust_project.txt", "Rust Project Workspace");
}

#[test]
fn test_ls_node_insight() {
    test_ls_insight_for_fixture("tests/fixtures/ls_node_project.txt", "Node.js Project");
}

#[test]
fn test_ls_go_insight() {
    test_ls_insight_for_fixture("tests/fixtures/ls_go_project.txt", "Go Module");
}
