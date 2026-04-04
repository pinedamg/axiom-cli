use axiom::IntentContext;
mod common;

#[test]
fn test_ls_v1_long_listing_synthesis() {
    let mut session = common::setup_session();
    let command = "ls -l";
    let context = IntentContext {
        last_message: "list files".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
-rw-r--r-- 1 user group 123 Mar 24 10:00 file1.rs
-rw-r--r-- 1 user group 456 Mar 24 10:01 file2.rs
-rw-r--r-- 1 user group 789 Mar 24 10:02 file3.rs
";

    for line in raw_output.lines().filter(|l| !l.is_empty()) {
        session.engine.process_line(line, command, &context);
    }

    let summaries = session.engine.flush_summaries();
    for s in &summaries { println!("DEBUG: {}", s); }
    assert!(summaries.iter().any(|s| s.contains("FILE [rw-]")), "Should synthesize files by permissions");
    assert!(summaries.iter().any(|s| s.contains(" (3) | file1.rs, file2.rs, file3.rs")), "Should list all files in summary");
}

#[test]
fn test_ls_v1_standard_extension_synthesis() {
    let mut session = common::setup_session();
    let command = "ls";
    let context = IntentContext {
        last_message: "show directory".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    // Standard ls output with multiple columns
    let raw_output = "Cargo.toml  README.md  src  target  test.rs  another.rs";

    session.engine.process_line(raw_output, command, &context);

    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("Grouped 2 files by extension [RS]")), "Should synthesize standard ls by extension");
}

#[test]
fn test_ls_v2_semantic_insight() {
    let mut session = common::setup_session();
    let command = "ls";
    let context = IntentContext {
        last_message: "is this a rust project?".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    // Process a line that includes Cargo.toml
    session.engine.process_line("Cargo.toml README.md src", command, &context);

    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("Insight: Detected Rust Project Workspace")), "Should provide semantic insight for Rust project");
}

#[test]
fn test_ls_v3_privacy_redaction() {
    let mut session = common::setup_session();
    let command = "ls -la";
    let context = IntentContext {
        last_message: "show me all files".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let hidden_line = ".env secret_key=12345";
    let result = session.engine.process_line(hidden_line, command, &context);
    
    // Result should be None because it's synthesized/redacted OR it should contain REDACTED_BY_SCHEMA
    if let Some(processed) = result {
        assert!(processed.contains("[REDACTED_BY_SCHEMA]"), "Hidden files should be redacted by default");
    }
}
