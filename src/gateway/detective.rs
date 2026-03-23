use sysinfo::System;

pub struct ProcessDetective;

impl ProcessDetective {
    /// Checks if any process in the hierarchy (up to 3 levels) is a known AI agent.
    pub fn is_called_by_ai() -> bool {
        let mut sys = System::new_all();
        sys.refresh_all();

        let pid = sysinfo::get_current_pid().ok();
        if pid.is_none() { return false; }
        
        let mut current_pid = pid.unwrap();
        let known_agents = ["gemini", "claude", "cursor", "node", "windsurf", "idx"];

        // Traverse up to 4 parents
        for _ in 0..4 {
            if let Some(process) = sys.process(current_pid) {
                let name = process.name().to_lowercase();
                
                // If we find an agent name in the process string
                for agent in &known_agents {
                    if name.contains(agent) {
                        return true;
                    }
                }

                // Move to parent
                if let Some(parent_pid) = process.parent() {
                    current_pid = parent_pid;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        false
    }

    /// Returns the name of the detected parent process for debugging
    pub fn get_parent_name() -> String {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        if let Some(current) = sys.process(sysinfo::get_current_pid().unwrap()) {
            if let Some(parent_pid) = current.parent() {
                if let Some(parent) = sys.process(parent_pid) {
                    return parent.name().to_string();
                }
            }
        }
        "unknown".to_string()
    }
}
