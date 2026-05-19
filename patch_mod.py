import sys

def apply_diff(file_path, search, replace):
    with open(file_path, "r") as f:
        content = f.read()
    if search not in content:
        print(f"Error: Search text not found in {file_path}")
        sys.exit(1)
    content = content.replace(search, replace)
    with open(file_path, "w") as f:
        f.write(content)
    print(f"Applied patch to {file_path}")

# Patch mod.rs - plugins type
apply_diff("src/engine/mod.rs",
    "pub plugins: Option<WasmPluginManager>,",
    "pub plugins: Option<Box<WasmPluginManager>>,")

apply_diff("src/engine/mod.rs",
    "        self.plugins = Some(manager);",
    "        // ⚡ Bolt: Box the plugin manager to minimize the memory footprint of AxiomEngine struct\n        self.plugins = Some(Box::new(manager));")

# Patch mod.rs - stage_deduplicate
search_dedup = """            let prefix = if self.discovery.repeat_count > 0 {
                Some(format!("... (previous line repeated {} more times)", self.discovery.repeat_count))
            } else { None };
            self.discovery.last_line = Some(line.to_string());
            self.discovery.repeat_count = 0;
            (prefix, PipelineAction::Continue(Cow::Borrowed(line)), "New line".to_string())"""
replace_dedup = """            let prefix = if self.discovery.repeat_count > 0 {
                Some(format!("... (previous line repeated {} more times)", self.discovery.repeat_count))
            } else { None };

            // ⚡ Bolt: Reuse the existing string buffer instead of allocating a new string on each different line.
            if let Some(mut buf) = self.discovery.last_line.take() {
                buf.clear();
                buf.push_str(line);
                self.discovery.last_line = Some(buf);
            } else {
                self.discovery.last_line = Some(line.to_string());
            }

            self.discovery.repeat_count = 0;
            (prefix, PipelineAction::Continue(Cow::Borrowed(line)), "New line".to_string())"""
apply_diff("src/engine/mod.rs", search_dedup, replace_dedup)

print("Done patching mod.rs.")
