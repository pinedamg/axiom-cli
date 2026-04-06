with open('src/engine/mod.rs', 'r') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "pub fn set_markdown_mode" in line:
        lines.insert(i, "    pub fn with_plugins(mut self, plugins: WasmPluginManager) -> Self {\n        self.plugins = Some(plugins);\n        self\n    }\n\n")
        break

with open('src/engine/mod.rs', 'w') as f:
    f.writelines(lines)
