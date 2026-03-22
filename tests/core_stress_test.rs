use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;

#[test]
fn test_core_generic_scenarios() {
    let config = AxiomConfig::default();
    let mut session = AxiomSession::new(config).expect("Failed to create session");
    
    let command = "complex-logger-tool";
    
    // Test Case 1: Docker-like logs (UUIDs and Hex)
    let context_generic = IntentContext {
        last_message: "just run".to_string(),
        command: command.to_string(),
        keywords: session.config.intent_keywords.clone(),
    };

    let docker_line = "Container 550e8400-e29b-41d4-a716-446655440000 started. Image: 0xdeadbeef";
    let (template, vars) = session.engine.discovery.extract_parts(docker_line);
    
    assert!(template.contains("<UUID>"));
    assert!(template.contains("<HEX>"));
    assert_eq!(vars.len(), 2);

    // Test Case 2: Python Traceback (Paths and Keywords)
    let traceback_line = "  File \"/usr/lib/python3.9/site-packages/requests/api.py\", line 64, in get";
    let (path_template, _) = session.engine.discovery.extract_parts(traceback_line);
    assert!(path_template.contains("<PATH>"));

    // Test Case 3: Intent Overriding for Errors
    let error_line = "[CRITICAL] System failure at /var/log/syslog";
    let context_error = IntentContext {
        last_message: "Why did it crash?".to_string(),
        command: command.to_string(),
        keywords: session.config.intent_keywords.clone(),
    };
    
    // Even if it was noise, it should be shown because of "crash" keyword
    let result = session.engine.process_line(error_line, command, &context_error);
    assert!(result.is_some());
    assert!(result.unwrap().contains("failure"));

    // Test Case 4: Privacy Redaction consistency (Using higher entropy secret)
    let secret_line = "The token is AKIA5G4H3J2K1L0M9N8P7Q6R5S4T3U2V1W0X and email is dev@test.com";
    let result_privacy = session.engine.process_line(secret_line, command, &context_generic);
    let output = result_privacy.unwrap();
    assert!(output.contains("[REDACTED_PII]"));
    assert!(output.contains("[REDACTED_SECRET]"));
}

#[test]
fn test_aggregator_no_information_loss() {
    let config = AxiomConfig::default();
    let mut session = AxiomSession::new(config).expect("Failed to create session");
    let command = "batch-processor";
    
    // Scenario: User doesn't care about the sequence, so aggregation should happen
    let context = IntentContext {
        last_message: "wait until done".to_string(),
        command: command.to_string(),
        keywords: session.config.intent_keywords.clone(),
    };

    // Simulate 15 repetitive lines to pass threshold
    for i in 0..15 {
        let line = format!("Log entry sequence #{}", i);
        session.engine.process_line(&line, command, &context);
    }

    let summaries = session.engine.flush_summaries();
    assert!(!summaries.is_empty(), "Aggregator should have captured noise lines");
    assert!(summaries[0].contains("matched 10 more times"));
}
