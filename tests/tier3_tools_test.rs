use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;

fn setup_session() -> AxiomSession {
    let config = AxiomConfig::default();
    AxiomSession::new(config).expect("Failed to setup session for testing")
}

#[test]
fn test_kubectl_pod_summary() {
    let mut session = setup_session();
    let command = "kubectl get pods";
    let context = IntentContext {
        last_message: "check system status".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
NAME                     READY   STATUS    RESTARTS   AGE
auth-service-xyz         1/1     Running   0          5d
api-gateway-abc          1/1     Running   0          5d
database-0               1/1     Running   0          10d
failing-worker           0/1     Error     5          2m
    ";

    let mut lines_printed = 0;
    let mut error_shown = false;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(line, command, &context) {
            lines_printed += 1;
            if processed.contains("failing-worker") {
                error_shown = true;
            }
        }
    }

    assert!(error_shown, "Pods with errors must be visible");
    assert!(lines_printed <= 3, "Healthy pods should be collapsed into a summary");
}

#[test]
fn test_terraform_plan_clean() {
    let mut session = setup_session();
    let command = "terraform plan";
    let context = IntentContext {
        last_message: "show deployment plan".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
aws_instance.web: Refreshing state... [id=i-1234567890]
aws_db_instance.db: Refreshing state... [id=db-987654321]
Plan: 1 to add, 0 to change, 0 to destroy.
    ";

    let mut refresh_shown = false;
    let mut plan_shown = false;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(line, command, &context) {
            if processed.contains("Refreshing state") {
                refresh_shown = true;
            }
            if processed.contains("Plan:") {
                plan_shown = true;
            }
        }
    }

    assert!(!refresh_shown, "Terraform refreshing noise should be hidden");
    assert!(plan_shown, "The final Terraform plan summary must be visible");
}
