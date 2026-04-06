with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "        let intent_str = format!(\"{} {}\", context.last_message, context.keywords.join(\" \"));" in lines[i]:
        lines[i] = "        let intent_str = format!(\"{} {}\", context.last_message, context.keywords.join(\" \"));\n        if !intent_str.trim().is_empty() && self.intelligence.is_relevant(&intent_str, line, 0.7) {\n            // Don't short-circuit if it's a Cargo line that we want to summarize\n            if command.starts_with(\"cargo\") && (line.contains(\"Checking\") || line.contains(\"Compiling\")) {\n                // Let it continue to synthesis\n            } else {\n                return PipelineAction::ShortCircuit(line.to_string());\n            }\n        }\n"
        lines[i+1] = ""
        lines[i+2] = ""
        lines[i+3] = ""
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
