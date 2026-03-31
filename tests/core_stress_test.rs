use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;

#[test]
fn test_core_generic_scenarios() {
    let config = AxiomConfig::default();
    let mut session = AxiomSession::new(config).expect("Failed to create session");
    
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
    
    let result = session.engine.process_line(error_line, command, &context_error);
    assert!(result.is_some());
    assert!(result.unwrap().contains("failure"));

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
    
    let context = IntentContext {
        last_message: "wait until done".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    for i in 0..15 {
        let line = format!("Log entry sequence #{}", i);
        session.engine.process_line(&line, command, &context);
    }

    let summaries = session.engine.flush_summaries();
    assert!(!summaries.is_empty(), "Aggregator should have captured noise lines");
    assert!(summaries[0].contains("matched 10 more times"));
}

#[test]
fn test_adversarial_secrets_and_false_positives() {
    let config = AxiomConfig::default();
    let mut session = AxiomSession::new(config).expect("Failed to create session");

    let command = "complex-logger-tool";

    let context_generic = IntentContext {
        last_message: "just run".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    // Test new secrets (Anthropic, Groq, Google Cloud, Stripe)
    let secret_line1 = "Anthropic: sk-ant-DUMMY_REDACTED_KEYS-test and Groq: gsk_DUMMYREDACTEDKEYS";
    let output1 = session.engine.process_line(secret_line1, command, &context_generic).unwrap();
    assert!(output1.contains("Anthropic: [REDACTED_SECRET]"));
    assert!(output1.contains("Groq: [REDACTED_SECRET]"));

    let secret_line2 = "GCP: AIzaSyDUMMYREDACTEDKEYS-test and Stripe: sk_live_DUMMYREDACTEDKEYS";
    let output2 = session.engine.process_line(secret_line2, command, &context_generic).unwrap();
    assert!(output2.contains("GCP: [REDACTED_SECRET]"));
    assert!(output2.contains("Stripe: [REDACTED_SECRET]"));

    // Test false positives bypass (Git SHA 40 chars and Docker ID 64 chars)
    let git_sha = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"; // 40 chars valid hex
    let docker_id = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0a1b2c3d4e5f6a7b8c9d0e1f2"; // 64 chars valid hex
    let fp_line = format!("Git {} and Docker {}", git_sha, docker_id);
    let output3 = session.engine.process_line(&fp_line, command, &context_generic).unwrap();

    assert!(output3.contains(git_sha));
    assert!(output3.contains(docker_id));
    assert!(!output3.contains("[REDACTED_SECRET]"));
}
