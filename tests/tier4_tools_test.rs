use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;

fn setup_session() -> AxiomSession {
    let config = AxiomConfig::default();
    AxiomSession::new(config).expect("Failed to setup session for testing")
}

#[test]
fn test_jq_array_collapsing() {
    let mut session = setup_session();
    let command = "jq '.' large.json";
    let context = IntentContext {
        last_message: "analyze the data".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
[
  { \"id\": 1, \"status\": \"active\" },
  { \"id\": 2, \"status\": \"active\" },
  { \"id\": 3, \"status\": \"active\" },
  { \"id\": 4, \"status\": \"active\" },
  { \"id\": 5, \"status\": \"active\" },
  { \"id\": 6, \"status\": \"active\" },
  { \"id\": 7, \"status\": \"active\" }
]
    ";

    let mut lines_printed = 0;
    for line in raw_output.lines() {
        if let Some(_) = session.engine.process_line(line, command, &context) {
            lines_printed += 1;
        }
    }

    assert!(lines_printed <= 8, "Repetitive JSON objects should be aggregated");
    let summaries = session.engine.flush_summaries();
    assert!(!summaries.is_empty());
}

#[test]
fn test_journalctl_noise_reduction() {
    let mut session = setup_session();
    let command = "journalctl -u ssh";
    let context = IntentContext {
        last_message: "show logs".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
Mar 22 10:00:01 host systemd[1]: Starting OpenBSD Secure Shell server...
Mar 22 10:00:02 host sshd[123]: Server listening on 0.0.0.0 port 22.
Mar 22 10:00:05 host systemd[1]: Started OpenBSD Secure Shell server.
Mar 22 10:00:10 host sshd[124]: Accepted password for user from 192.168.1.1
Mar 22 10:00:11 host sshd[124]: pam_unix(sshd:session): session opened for user mpineda
    ";

    let mut lines_printed = 0;
    for line in raw_output.lines() {
        if let Some(_) = session.engine.process_line(line, command, &context) {
            lines_printed += 1;
        }
    }

    assert!(lines_printed < 5, "Routine systemd and ssh logs should be filtered out");
}

#[test]
fn test_markdown_table_conversion_integration() {
    let mut session = setup_session();
    session.engine.set_markdown_mode(true);
    
    let command = "docker ps";
    let context = IntentContext {
        last_message: "list containers".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    // Note: Using 3 spaces to ensure table detection
    let table_header = "CONTAINER ID   IMAGE   COMMAND   STATUS";
    let table_row = "1234567890ab   nginx   \"nginx\"   Up 2 hours";

    let header_result = session.engine.process_line(table_header, command, &context).expect("Header should be returned");
    let row_result = session.engine.process_line(table_row, command, &context).expect("Row should be returned");

    // Check for Markdown pipe separators
    assert!(header_result.contains("|"), "Header should contain pipes. Result: {}", header_result);
    assert!(header_result.contains("CONTAINER"), "Header should contain the column name");
    assert!(row_result.contains("|"), "Row should contain pipes. Result: {}", row_result);
}
