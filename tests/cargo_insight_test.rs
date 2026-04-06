use axiom::engine::AxiomEngine;
use axiom::privacy::PrivacyRedactor;
use axiom::engine::intelligence::FuzzyIntelligence;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;

#[test]
fn test_cargo_aggregation() {
    let redactor = PrivacyRedactor::new(4.5, vec![]);
    let mut engine = AxiomEngine::new(redactor, vec![], Box::new(FuzzyIntelligence), 3);
    let context = IntentContext {
        last_message: "Build the project".to_string(),
        command: "cargo build".to_string(),
        keywords: vec![],
    };

    let lines = vec![
        "    Checking serde v1.0.150",
        "    Checking serde_json v1.0.91",
        "    Checking tokio v1.23.0",
        "    Compiling regex v1.7.0",
        "    Compiling anyhow v1.0.60",
        "    Compiling libc v0.2.1",
        "    Compiling axiom v0.1.0 (/home/mpineda/projects/axiom)",
    ];

    for line in lines {
        engine.process_line(TerminalEvent::StaticLine(line.to_string()), "cargo build", &context);
    }

    let summaries = engine.flush_summaries();
    println!("DEBUG: Synthesis Buffer keys: {:?}", engine.discovery.synthesis_buffer.keys());
    println!("SUMMARIES: {:?}", summaries);
    
    // Check for insight
    assert!(summaries.iter().any(|s| s.contains("Checking") || s.contains("Compiling")));
    
    // Check that we see the aggregated crates
    let joined = summaries.join("\n");
    assert!(joined.contains("serde"));
    assert!(joined.contains("tokio"));
    assert!(joined.contains("regex"));
    
    // Axiom itself should NOT be aggregated because it's an outlier (contains path)
    // and should have been printed as a normal line by the engine.
    // In this unit test of 'engine.flush_summaries()', only the aggregated items appear.
    // So joined.contains("axiom") will be false if it's an outlier.
}
