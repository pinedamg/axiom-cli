with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "let is_outlier = handler.map_or(false, |h| {" in line:
        lines.insert(i, """        if self.intelligence.is_relevant(&context.last_message, line, 0.5) || self.intelligence.is_relevant(&context.keywords.join(" "), line, 0.5) {
            return PipelineAction::Continue(line.to_string());
        }

""")
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
