with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "        if self.intelligence.is_relevant(&context.last_message, line, 0.7) || self.intelligence.is_relevant(&context.keywords.join(\" \"), line, 0.7) {" in lines[i]:
        lines[i] = "        let intent_str = format!(\"{} {}\", context.last_message, context.keywords.join(\" \"));\n        if !intent_str.trim().is_empty() && self.intelligence.is_relevant(&intent_str, line, 0.7) {\n            return PipelineAction::ShortCircuit(line.to_string());\n        }\n"
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
