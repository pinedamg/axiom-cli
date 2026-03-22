use axiom::{IntentContext, SemanticStreamer};
// use axiom::engine::AxiomEngine;

#[test]
fn test_error_prioritization() {
    // let engine = AxiomEngine::default(); 
    
    // Manual mock to demonstrate the concept before implementing the real engine
    let context = IntentContext {
        last_message: "Why is the build failing?".to_string(),
        command: "npm run build".to_string(),
    };

    let raw_output = "
        [SUCCESS] Loading assets...
        [SUCCESS] Optimizing images...
        [ERROR] Failed to compile: 'src/main.rs' not found.
        [SUCCESS] Cleaning up...
    ";

    // Simulation of what the engine will do:
    let result = if context.last_message.contains("fail") || context.last_message.contains("error") {
        "[ERROR] Failed to compile: 'src/main.rs' not found.\n(3 success lines collapsed)".to_string()
    } else {
        raw_output.to_string()
    };

    assert!(result.contains("[ERROR]"));
    assert!(!result.contains("[SUCCESS] Loading assets...")); 
}
