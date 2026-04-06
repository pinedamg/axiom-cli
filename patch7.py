with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "        if self.intelligence.is_relevant(&_context.last_message, line, 0.5) || self.intelligence.is_relevant(&_context.keywords.join(\" \"), line, 0.5) {" in lines[i]:
        lines[i] = ""
        lines[i+1] = ""
        lines[i+2] = ""
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
