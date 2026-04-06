mod common;
use axiom::IntentContext;
use axiom::gateway::core::TerminalEvent;

#[test]
fn test_docker_pull_synthesis() {
    let mut session = common::setup_session();
    let command = "docker pull alpine";
    let context = IntentContext {
        last_message: "pull image".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
e17133b79956: Pulling fs layer
717b09883515: Waiting
e17133b79956: Downloading [=========>                                         ]  1.5MB/5.8MB
717b09883515: Extracting [==================================================>]  15.2kB/15.2kB
    ";

    for line in raw_output.lines() {
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }
    
    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("Processing 4 image layers")), "Should contain transfer insight");
    assert!(summaries.iter().any(|s| s.contains("Hidden 4 layer progress updates")), "Should contain layer summary");
}

#[test]
fn test_kubectl_get_pods_synthesis() {
    let mut session = common::setup_session();
    let command = "kubectl get pods";
    let context = IntentContext {
        last_message: "check pods".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
NAME                     READY   STATUS             RESTARTS   AGE
auth-service-xyz         1/1     Running            0          24h
db-instance-123          1/1     Running            0          5d
api-gateway-abc          0/1     CrashLoopBackOff   15         2h
redis-master             1/1     Running            0          10d
    ";

    for line in raw_output.lines() {
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }
    
    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("Detected 1 unhealthy resources")), "Should contain health warning");
    assert!(summaries.iter().any(|s| s.contains("Running [3]")), "Should group running pods");
    assert!(summaries.iter().any(|s| s.contains("CrashLoopBackOff [1]")), "Should group failing pod");
}

#[test]
fn test_terraform_plan_synthesis() {
    let mut session = common::setup_session();
    let command = "terraform plan";
    let context = IntentContext {
        last_message: "plan infrastructure".to_string(),
        command: command.to_string(),
        keywords: vec![],
    };

    let raw_output = "
  # aws_instance.web will be created
  + resource \"aws_instance\" \"web\" {
      + ami                          = \"ami-0c55b159cbfafe1f0\"
      + instance_type                = \"t2.micro\"
    }

  # aws_db_instance.db will be destroyed
  - resource \"aws_db_instance\" \"db\" {
      - name = \"mydb\"
    }
    ";

    for line in raw_output.lines() {
        session.engine.process_line(TerminalEvent::StaticLine(line.to_string()), command, &context);
    }
    
    let summaries = session.engine.flush_summaries();
    assert!(summaries.iter().any(|s| s.contains("1 to add") && s.contains("1 to destroy")), "Should contain plan insight");
    assert!(summaries.iter().any(|s| s.contains("CREATE: 1 resources")), "Should summarize creates");
    assert!(summaries.iter().any(|s| s.contains("DESTROY: 1 resources")), "Should summarize destroys");
}
