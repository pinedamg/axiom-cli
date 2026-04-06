with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "if self.intelligence.is_relevant(&context.last_message, line, 0.5) || self.intelligence.is_relevant(&context.keywords.join(\" \"), line, 0.5) {" in lines[i]:
        lines[i] = "        if self.intelligence.is_relevant(&_context.last_message, line, 0.7) || self.intelligence.is_relevant(&_context.keywords.join(\" \"), line, 0.7) {\n            return PipelineAction::ShortCircuit(line.to_string());\n        }\n"
        lines[i+1] = ""
        lines[i+2] = ""
        break

for i in range(len(lines)):
    if "let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());" in lines[i]:
        lines.insert(i, lines[i-1])
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
