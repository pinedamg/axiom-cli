mod common;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;
use std::fs;

#[test]
fn test_docker_ps_insight() {
    let mut session = common::setup_session();
    let command = "docker ps -a";
    let context = IntentContext {
        last_message: "check containers".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = fs::read_to_string("tests/fixtures/docker_ps_raw.txt")
        .expect("Failed to load docker ps fixture");

    for line in raw_output.lines() {
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }

    let summaries = session.engine.flush_summaries();
    
    let mut found_prune_suggestion = false;
    
    for summary in summaries {
        if summary.contains("docker system prune") {
            found_prune_suggestion = true;
        }
    }
    
    // In our fixture, we have 6 stopped containers. The threshold in DockerHandler is > 5.
    assert!(found_prune_suggestion, "Should suggest 'docker system prune' for 6 stopped containers");
}
