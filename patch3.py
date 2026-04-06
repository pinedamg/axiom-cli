with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "pub struct SessionStats" in line:
        break
else:
    # Append SessionStats if not found
    with open('src/engine/mod.rs', 'a') as f:
        f.write("\n#[derive(Default, Clone, Debug)]\npub struct SessionStats {\n    pub raw_bytes: usize,\n    pub saved_bytes: usize,\n}\n")

    with open('src/engine/mod.rs', 'r') as f:
        lines = f.readlines()

    for i, line in enumerate(lines):
        if "pub last_command: String," in line:
            lines.insert(i+1, "    pub stats: SessionStats,\n")
            break

    for i, line in enumerate(lines):
        if "line_counter: 0," in line:
            lines.insert(i+1, "            stats: SessionStats::default(),\n")
            break

    for i, line in enumerate(lines):
        if "pub fn get_learned_templates" in line:
            lines.insert(i, "    pub fn get_session_stats(&self) -> Option<&SessionStats> {\n        Some(&self.stats)\n    }\n\n")
            break

    with open('src/engine/mod.rs', 'w') as f:
        f.writelines(lines)
