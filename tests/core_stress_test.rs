mod common;
use axiom::IntentContext;

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
        session.engine.process_line(&line, command, &context);
    }

    let summaries = session.engine.flush_summaries();
    assert!(!summaries.is_empty(), "Aggregator should have captured noise lines");
    assert!(summaries[0].contains("matched 10 more times"));
}

#[test]
fn test_adversarial_secrets() {
    let config = AxiomConfig::default();
    let session = AxiomSession::new(config).expect("Failed to create session");

    let fake_keys = vec![
        "AKIAIOSFODNN7EXAMPLE",                                // AWS
        "sk-ant-api03-abcdefghijklmnopqrstuvwxyz1234567890",   // Anthropic
        "gsk_ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",            // Groq
        "sk_live_51MzhABCDEF1234567890",                       // Stripe
        "ya29.a0AfB_byCDEF1234567890",                         // Google OAuth
        "sk-proj-abcDEFghiJKLmnoPQRstuVWXyz123456",            // OpenAI Project
        "sk-123456789012345678901234567890123456789012345678", // OpenAI Legacy
    ];

    for key in fake_keys {
        let line = format!("Found key: {}", key);
        let result = session.engine.redactor.redact(&line);
        assert!(
            result.contains("[REDACTED_SECRET]"),
            "Secret not redacted: {}", key
        );
        assert!(!result.contains(key), "Secret leaked in output: {}", key);
    }

    // Test false positives (Git SHA and Docker IDs should NOT be redacted)
    let git_sha = "9b4662d55d3e020e98031e405a415053e1a0678d"; // 40 chars
    let docker_id = "65239e235a9f6e14a1f68153eb268df1d02c81729ecf6168e36fa33c7f1a3028"; // 64 chars

    let result_git = session.engine.redactor.redact(&format!("commit {}", git_sha));
    assert!(result_git.contains(git_sha), "Git SHA falsely redacted");

    let result_docker = session.engine.redactor.redact(&format!("container {}", docker_id));
    assert!(result_docker.contains(docker_id), "Docker ID falsely redacted");
}
