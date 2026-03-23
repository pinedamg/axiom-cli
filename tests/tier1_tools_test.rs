use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;

fn setup_session() -> AxiomSession {
    let config = AxiomConfig::default();
    AxiomSession::new(config).expect("Failed to setup session for testing")
}

#[test]
fn test_ls_efficiency() {
    let mut session = setup_session();
    let command = "ls -la";
    let context = IntentContext {
        last_message: "show files".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
total 1234
drwxr-xr-x  2 user group  4096 Mar 22 10:00 .
drwxr-xr-x 20 user group  4096 Mar 22 10:00 ..
-rw-r--r--  1 user group   123 Mar 22 10:00 file1.txt
-rw-r--r--  1 user group   456 Mar 22 10:00 file2.txt
-rw-r--r--  1 user group   789 Mar 22 10:00 .hidden_config
    ";

    let mut compressed_output = String::new();
    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(line, command, &context) {
            compressed_output.push_str(&processed);
            compressed_output.push('\n');
        }
    }

    let original_size = raw_output.len();
    let compressed_size = compressed_output.len();
    let savings = (original_size - compressed_size) as f64 / original_size as f64 * 100.0;

    println!("LS Savings: {:.1}%", savings);
    assert!(!compressed_output.contains(".hidden_config"));
    assert!(!compressed_output.contains("total 1234"));
    assert!(savings > 30.0, "LS reduction should be significant");
}

#[test]
fn test_ripgrep_aggregation() {
    let mut session = setup_session();
    let command = "rg 'search'";
    let context = IntentContext {
        last_message: "just searching".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let mut raw_output = String::new();
    for i in 0..20 {
        raw_output.push_str(&format!("src/file_{}.rs:10: content\n", i));
    }

    for line in raw_output.lines() {
        session.engine.process_line(line, command, &context);
    }
    
    let summaries = session.engine.flush_summaries();
    assert!(!summaries.is_empty(), "Summaries should be generated for repetitive rg output");
}

#[test]
fn test_cat_guardian_mode() {
    let mut session = setup_session();
    let command = "cat giant_file.log";
    let context = IntentContext {
        last_message: "show logs".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let mut raw_output = String::new();
    for i in 0..150 {
        raw_output.push_str(&format!("Log line number {}\n", i));
    }

    let mut lines_printed = 0;
    let mut guardian_triggered = false;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(line, command, &context) {
            lines_printed += 1;
            if processed.contains("Guardian Mode") {
                guardian_triggered = true;
            }
        }
    }

    assert!(guardian_triggered, "Guardian mode should trigger after 100 lines");
    assert!(lines_printed <= 101, "Should not print more than 101 lines including warning");
}
