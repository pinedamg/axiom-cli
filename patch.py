with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "match manager.transform" in line:
        lines[i] = "            let processed = manager.transform(&line.unwrap_or_default());\n            if processed.is_empty() {\n                None\n            } else {\n                Some(processed)\n            }\n"
        lines[i+1] = ""
        lines[i+2] = ""
        lines[i+3] = ""

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
