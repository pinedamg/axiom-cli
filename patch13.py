with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "fn stage_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {" in lines[i]:
        lines[i] = """    fn stage_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());

        let intent_str = format!("{} {}", context.last_message, context.keywords.join(" "));
        if !intent_str.trim().is_empty() && self.intelligence.is_relevant(&intent_str, line, 0.7) {
            // Don't short-circuit if it's a Cargo line that we want to summarize
            let is_cargo_aggregate = command.starts_with("cargo") && (line.contains("Checking") || line.contains("Compiling"));
            let is_docker_aggregate = command.starts_with("docker") && (line.contains("Pulling") || line.contains("Waiting") || line.contains("Download") || line.contains("Extracting"));
            if is_cargo_aggregate || is_docker_aggregate {
                // Let it continue to synthesis
            } else {
                return PipelineAction::ShortCircuit(line.to_string());
            }
        }

        let is_outlier = handler.map_or(false, |h| {
            h.parse_line(line).map_or(false, |m| h.is_outlier(line, &m))
        });

        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return PipelineAction::ShortCircuit("[AXIOM] High noise detected. Activating Guardian Mode...".to_string());
            }
            if self.line_counter > 100 && !is_outlier {
                return PipelineAction::Swallow;
            }
        }
        PipelineAction::Continue(line.to_string())
    }
"""
        del lines[i+1:i+35] # Clean up the rest of the original stage_guard
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
