use axiom::engine::AxiomEngine;
use axiom::privacy::PrivacyRedactor;
use axiom::engine::intelligence::FuzzyIntelligence;
use axiom::IntentContext;
use axiom::engine::commands::get_all_handlers;

#[test]
fn test_recipe_pipeline_full_flow() {
    // Arrange
    let redactor = PrivacyRedactor::default();
    let intelligence = Box::new(FuzzyIntelligence);
    let mut engine = AxiomEngine::new(redactor, vec![], intelligence, 3);
    engine.handlers = get_all_handlers();
    
    // Use an intent that clearly overlaps with the line
    let context = IntentContext {
        last_message: "I want to see main.rs source code".to_string(),
        command: "ls".to_string(),
        keywords: vec!["main.rs".to_string()],
    };

    // Act
    let important_line = "main.rs";
    let out = engine.process_line(important_line, "ls", &context);
    
    // Assert
    assert!(out.is_some(), "Stage 5 (Semantic) should let pass lines relevant to the message");
    assert_eq!(out.unwrap(), "main.rs");
}

#[test]
fn test_recipe_pipeline_deduplication() {
    let redactor = PrivacyRedactor::default();
    let intelligence = Box::new(FuzzyIntelligence);
    let mut engine = AxiomEngine::new(redactor, vec![], intelligence, 3);
    
    let context = IntentContext {
        last_message: "List files".to_string(),
        command: "ls".to_string(),
        keywords: vec![],
    };
    let line = "file.txt";

    let _ = engine.process_line(line, "ls", &context);
    let out2 = engine.process_line(line, "ls", &context);
    
    assert!(out2.is_none(), "Stage 1 (Dedup) should swallow repeated lines");
}
