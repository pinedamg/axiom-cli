with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());" in line:
        lines.insert(i, """        if self.intelligence.is_relevant(&_context.last_message, line, 0.7) {
            return PipelineAction::ShortCircuit(line.to_string());
        }

""")
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
