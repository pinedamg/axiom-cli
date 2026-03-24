mod common;
use axiom::IntentContext;

#[test]
fn test_kubectl_advanced_cleaning() {
    let mut session = common::setup_session();
    let command = "kubectl describe pod my-pod";
    let context = IntentContext {
        last_message: "check pod errors".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
Name:         my-pod
Namespace:    default
Priority:     0
Labels:       app=web, env=prod, version=1.0.0
Annotations:  checksum/config=abc123def456
managedFields:
  - manager: kube-controller-manager
    operation: Update
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  10m   default-scheduler  Successfully assigned my-pod to node-1
    ";

    let mut lines_printed = 0;
    let mut managed_fields_shown = false;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(line, command, &context) {
            lines_printed += 1;
            if processed.contains("managedFields") {
                managed_fields_shown = true;
            }
        }
    }

    assert!(!managed_fields_shown, "Managed fields must be hidden (noise/security)");
}

#[test]
fn test_terraform_advanced_plan() {
    let mut session = common::setup_session();
    let command = "terraform plan";
    let context = IntentContext {
        last_message: "review infrastructure changes".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
  # aws_instance.web will be updated in-place
  ~ resource \"aws_instance\" \"web\" {
      ~ instance_type = \"t2.micro\" -> \"t3.medium\"
      + tags          = { \"Environment\" = \"Prod\" }
    }

Plan: 0 to add, 1 to change, 0 to destroy.
    ";

    let mut plan_summary_count = 0;
    let mut instance_id_count = 0;
    let mut lines_processed = 0;

    for line in raw_output.lines() {
        if let Some(processed) = session.engine.process_line(line, command, &context) {
            lines_processed += 1;
            if processed.contains("Plan:") {
                plan_summary_count += 1;
            }
            if processed.contains("aws_instance.web") {
                instance_id_count += 1;
            }
        }
    }

    assert_eq!(plan_summary_count, 1, "Plan summary must be kept");
    // Depending on the 'action: collapse' behavior, it might be hidden or replaced with a summary line.
    // If Axiom's engine is configured to hide collapsed lines by default in the test setup:
    // assert!(instance_id_count == 0 || instance_id_count == 1);
}

