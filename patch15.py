with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "fn stage_analyze(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {" in lines[i]:
        lines.insert(i, "    fn stage_redact(&self, line: &str) -> String {\n        self.redactor.redact(line)\n    }\n\n")
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
