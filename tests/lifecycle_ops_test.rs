use axiom::engine::AxiomEngine;
use axiom::IntentContext;
use axiom::privacy::PrivacyRedactor;
use axiom::engine::intelligence::FuzzyIntelligence;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_aggressive_deduplication() {
    let redactor = PrivacyRedactor::default();
    let intelligence = Box::new(FuzzyIntelligence);
    let mut engine = AxiomEngine::new(redactor, vec![], intelligence);
    
    let context = IntentContext {
        last_message: "Testing".to_string(),
        command: "test".to_string(),
        keywords: vec![],
    };

    // First line should be processed normally
    let line1 = "Downloading package...";
    let res1 = engine.process_line(line1, "test", &context);
    assert_eq!(res1, Some(line1.to_string()));

    // Second identical line should be swallowed
    let res2 = engine.process_line(line1, "test", &context);
    assert_eq!(res2, None);

    // Third identical line should be swallowed
    let res3 = engine.process_line(line1, "test", &context);
    assert_eq!(res3, None);

    // A different line should trigger the "repeated" message
    let line2 = "Extracting...";
    let res4 = engine.process_line(line2, "test", &context);
    assert!(res4.unwrap().contains("previous line repeated 2 more times"));
}

#[tokio::test]
async fn test_raw_backup_tee_system() {
    // We need to ensure we don't pollute /tmp/axiom during tests if possible, 
    // but the current implementation is hardcoded. 
    // For now, we'll verify it writes SOMETHING to a log.
    let log_path = std::path::Path::new("/tmp/axiom/last_run.log");
    
    // Clear previous log if it exists to have a clean slate
    let _ = fs::remove_file(log_path);

    let mut engine = AxiomEngine::new(PrivacyRedactor::default(), vec![], Box::new(FuzzyIntelligence));
    let context = IntentContext {
        last_message: "Testing".to_string(),
        command: "test".to_string(),
        keywords: vec![],
    };

    let test_line = "RAW_BACKUP_TEST_LINE_12345";
    engine.process_line(test_line, "test", &context);

    assert!(log_path.exists());
    let contents = fs::read_to_string(log_path).unwrap();
    assert!(contents.contains(test_line));
}

#[tokio::test]
async fn test_installer_shell_integration() {
    let dir = tempdir().unwrap();
    let zshrc = dir.path().join(".zshrc");
    
    // 1. Initial creation
    axiom::engine::installer::AxiomInstaller::install_shell_integration(&zshrc, true).unwrap();
    let content = fs::read_to_string(&zshrc).unwrap();
    assert!(content.contains("axiom initialize"));
    assert!(content.contains("alias git='axiom git'"));
    assert!(content.contains("export PATH"));

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
    assert!(content.contains("BEGIN AXIOM INSTRUCTIONS"));
    assert!(content.ends_with("# Original Content"));

    // Inject again (should update existing block)
    axiom::engine::installer::AxiomInstaller::inject_ai_context(&agents_md, true).unwrap();
    let content2 = fs::read_to_string(&agents_md).unwrap();
    let count = content2.matches("BEGIN AXIOM INSTRUCTIONS").count();
    assert_eq!(count, 1);
}
