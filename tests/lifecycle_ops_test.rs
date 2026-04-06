use axiom::engine::AxiomEngine;
use axiom::IntentContext;
use axiom::privacy::PrivacyRedactor;
use axiom::engine::intelligence::FuzzyIntelligence;
use axiom::gateway::core::TerminalEvent;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_aggressive_deduplication() {
    let redactor = PrivacyRedactor::default();
    let intelligence = Box::new(FuzzyIntelligence);
    let mut engine = AxiomEngine::new(redactor, vec![], intelligence, 1); // Threshold 1
    
    let context = IntentContext::default();

    // First line should be processed normally
    let line1 = "Downloading package...";
    let res1 = engine.process_line(TerminalEvent::StaticLine(line1.to_string()), "test", &context);
    assert_eq!(res1, Some(line1.to_string()));

    // Second identical line should be swallowed (since threshold is 1)
    let res2 = engine.process_line(TerminalEvent::StaticLine(line1.to_string()), "test", &context);
    assert_eq!(res2, None);
}

#[tokio::test]
async fn test_installer_shell_integration() {
    let dir = tempdir().unwrap();
    let zshrc = dir.path().join(".zshrc");
    
    // 1. Initial creation
    axiom::engine::installer::AxiomInstaller::install_shell_integration(&zshrc, true).unwrap();
    let content = fs::read_to_string(&zshrc).unwrap();
    assert!(content.contains("axiom initialize"));
    assert!(content.contains("axiom() {"));
    assert!(content.contains("alias git='axiom git'"));

    // 2. Idempotency (run again, shouldn't duplicate)
    axiom::engine::installer::AxiomInstaller::install_shell_integration(&zshrc, true).unwrap();
    let content2 = fs::read_to_string(&zshrc).unwrap();
    let count = content2.matches("axiom initialize").count();
    assert_eq!(count, 2); // Start and End delimiters
}

#[tokio::test]
async fn test_installer_ai_context_injection() {
    let dir = tempdir().unwrap();
    let agents_md = dir.path().join("AGENTS.md");
    fs::write(&agents_md, "# Original Content").unwrap();

    // Inject as prefix
    axiom::engine::installer::AxiomInstaller::inject_ai_context(&agents_md, true).unwrap();
    let content = fs::read_to_string(&agents_md).unwrap();
    assert!(content.contains("### 🤖 Axiom: Agent Execution Protocol"));
    assert!(content.ends_with("# Original Content"));
}
