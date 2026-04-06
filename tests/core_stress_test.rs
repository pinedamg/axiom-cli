mod common;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;

#[test]
fn test_core_generic_scenarios() {
    let mut session = common::setup_session();
    
    let command = "complex-logger-tool";
    
    let context_generic = IntentContext {
        last_message: "just run".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let docker_line = "Container 550e8400-e29b-41d4-a716-446655440000 started. Image: 0xdeadbeef";
    let (template, vars) = session.engine.discovery.extract_parts(docker_line);
    
    assert!(template.contains("<UUID>"));
    assert!(template.contains("<HEX>"));
    assert_eq!(vars.len(), 2);

    let traceback_line = "  File \"/usr/lib/python3.9/site-packages/requests/api.py\", line 64, in get";
    let (path_template, _) = session.engine.discovery.extract_parts(traceback_line);
    assert!(path_template.contains("<PATH>"));

    let error_line = "[CRITICAL] System failure at /var/log/syslog";
    let context_error = IntentContext {
        last_message: "Why did it crash?".to_string(),
        command: command.to_string(),
        keywords: vec!["crash".to_string()],
    };
    
    let result = session.engine.process_line(TerminalEvent::StaticLine(error_line.to_string()), command, &context_error);
    assert!(result.is_some());
    assert!(result.unwrap().contains("failure"));

    let secret_line = "The token is AWS_ACCESS_KEY_EXAMPLE_123456789 and email is dev@test.com";
    let result_privacy = session.engine.process_line(TerminalEvent::StaticLine(secret_line.to_string()), command, &context_generic);
    let output = result_privacy.unwrap();
    assert!(output.contains("[REDACTED_PII]"));
    assert!(output.contains("[REDACTED_SECRET]"));
}

#[test]
fn test_aggregator_no_information_loss() {
    let (mut session, _tmp) = common::setup_test_session();
    session.engine.discovery.threshold = 5; // Match old default for this test
    let command = "batch-processor";

    
    let context = IntentContext {
        last_message: "wait until done".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    for i in 0..15 {
        let line = format!("Log entry sequence #{}", i);
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }

    let summaries = session.engine.flush_summaries();
    assert!(!summaries.is_empty(), "Aggregator should have captured noise lines");
    assert!(summaries[0].contains("matched 10 more times"));
}
