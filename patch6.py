with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i in range(len(lines)):
    if "        if self.intelligence.is_relevant(&_context.last_message, line, 0.7) || self.intelligence.is_relevant(&_context.keywords.join(\" \"), line, 0.7) {" in lines[i]:
        lines[i] = "        if self.intelligence.is_relevant(&_context.last_message, line, 0.5) || self.intelligence.is_relevant(&_context.keywords.join(\" \"), line, 0.5) {\n"
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
