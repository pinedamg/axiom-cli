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
    
    // Check that we see the crates (some of them)
    let joined = summaries.join("\n");
    assert!(joined.contains("serde"));
    assert!(joined.contains("tokio"));
    assert!(joined.contains("axiom"));
}
