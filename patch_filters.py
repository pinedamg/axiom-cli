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

# Patch filters.rs - buffer replace
apply_diff("src/gateway/filters.rs",
    "let line = std::mem::take(&mut self.buffer);",
    "// ⚡ Bolt: Extract and pre-allocate the next buffer to avoid dynamically growing the string from 0.\n                let line = std::mem::replace(&mut self.buffer, String::with_capacity(1024));")

print("Done patching filters.rs.")
