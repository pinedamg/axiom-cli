mod common;
use axiom::IntentContext;

#[test]
fn test_jq_structural_synthesis() {
    let mut session = common::setup_session();
    let command = "jq .";
    let context = IntentContext {
        last_message: "read json".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
[
  {
    \"id\": 1,
    \"name\": \"item 1\",
    \"value\": \"data 1\"
  },
  {
    \"id\": 2,
    \"name\": \"item 2\",
    \"value\": \"data 2\"
  }
]
    ";

    for line in raw_output.lines() {
        session.engine.process_line(line, command, &context);
    }
    
    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("Synthesized 2 JSON/YAML objects")), "Should contain structural insight");
    assert!(summaries.iter().any(|s| s.contains("key [id]")), "Should mention id key");
    assert!(summaries.iter().any(|s| s.contains("key [name]")), "Should mention name key");
}

#[test]
fn test_journalctl_noise_reduction() {
    let mut session = common::setup_session();
    let command = "journalctl -u systemd-logind";
    let context = IntentContext {
        last_message: "check logs".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
Mar 27 18:50 machine systemd-logind[123]: New session 1 of user mpineda.
Mar 27 18:51 machine systemd-logind[123]: Session 1 logged out.
Mar 27 18:52 machine kernel: some kernel noise
Mar 27 18:53 machine systemd[1]: Started User Manager for UID 1000.
    ";

    for line in raw_output.lines() {
        session.engine.process_line(line, command, &context);
    }
    
    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("System Logs")), "Should contain system log insight");
    assert!(summaries.iter().any(|s| s.contains("noise lines from system service [systemd-logind]")), "Should mention logind noise");
}
