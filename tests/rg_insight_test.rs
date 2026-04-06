use axiom::engine::AxiomEngine;
use axiom::privacy::PrivacyRedactor;
use axiom::engine::intelligence::FuzzyIntelligence;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;

#[test]
fn test_rg_aggregation() {
    let redactor = PrivacyRedactor::new(4.5, vec![]);
    let mut engine = AxiomEngine::new(redactor, vec![], Box::new(FuzzyIntelligence), 3);
    let context = IntentContext {
        last_message: "Find something".to_string(),
        command: "rg Axiom".to_string(),
        keywords: vec![],
    };

    let lines = vec![
        "src/main.rs:10:fn main() {",
        "src/main.rs:15:  println!(\"Axiom\");",
        "src/lib.rs:5:pub fn lib_axiom() {}",
        "README.md:1: # Axiom Project",
    ];

    for line in lines {
        engine.process_line(TerminalEvent::StaticLine(line.to_string()), "rg Axiom", &context);
    }

    let summaries = engine.flush_summaries();
    
    // Check for insight
    assert!(summaries.iter().any(|s| s.contains("Search found 4 matches across 3 unique files")));
    
    // Check for file-specific summaries
    assert!(summaries.iter().any(|s| s.contains("src/main.rs: 2 matches")));
    assert!(summaries.iter().any(|s| s.contains("src/lib.rs: 1 matches")));
    assert!(summaries.iter().any(|s| s.contains("README.md: 1 matches")));
}
