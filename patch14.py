with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "        // 5a. Schema Check" in lines[i]:
        lines.insert(i, "    fn stage_analyze(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {\n        let handler_idx = self.handlers.iter().position(|h| h.matches(command));\n")
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
