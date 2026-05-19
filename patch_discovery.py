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

# Patch discovery.rs - variable_buffer type
apply_diff("src/engine/discovery.rs",
    "pub variable_buffer: BTreeMap<String, Vec<Vec<String>>>,",
    "// ⚡ Bolt: Tracing matches via usize counter instead of tracking full vectors of extracted regex matches.\n    pub variable_buffer: BTreeMap<String, usize>,")

# Patch discovery.rs - extract_parts return type & logic
search_extract = """    pub fn extract_parts(&self, line: &str) -> (String, Vec<String>) {
        use std::sync::OnceLock;
        static RE_UUID: OnceLock<Regex> = OnceLock::new();
        static RE_HEX: OnceLock<Regex> = OnceLock::new();
        static RE_PATH: OnceLock<Regex> = OnceLock::new();
        static RE_MONTHS: OnceLock<Regex> = OnceLock::new();
        static RE_TIME: OnceLock<Regex> = OnceLock::new();
        static RE_NUM: OnceLock<Regex> = OnceLock::new();

        let re_uuid = RE_UUID.get_or_init(|| Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap());
        let re_hex = RE_HEX.get_or_init(|| Regex::new(r"0x[0-9a-fA-F]+").unwrap());
        let re_path = RE_PATH.get_or_init(|| Regex::new(r"/[a-zA-Z0-9\\._\\-/]+").unwrap());
        let re_months = RE_MONTHS.get_or_init(|| Regex::new(r"(?i)(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)").unwrap());
        let re_time = RE_TIME.get_or_init(|| Regex::new(r"\\d{1,2}:\\d{2}").unwrap());
        let re_num = RE_NUM.get_or_init(|| Regex::new(r"\\d+").unwrap());

        // Pre-allocate variable list to avoid reallocations
        let mut variables = Vec::with_capacity(8);

        let s = re_uuid.replace_all(line, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<UUID>" });
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<HEX>" });
        let s = re_path.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<PATH>" });
        let s = re_months.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<MONTH>" });
        let s = re_time.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<TIME>" });
        let s = re_num.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<NUM>" });

        (s.to_string(), variables)
    }"""

replace_extract = """    pub fn extract_parts(&self, line: &str) -> String {
        use std::sync::OnceLock;
        static RE_UUID: OnceLock<Regex> = OnceLock::new();
        static RE_HEX: OnceLock<Regex> = OnceLock::new();
        static RE_PATH: OnceLock<Regex> = OnceLock::new();
        static RE_MONTHS: OnceLock<Regex> = OnceLock::new();
        static RE_TIME: OnceLock<Regex> = OnceLock::new();
        static RE_NUM: OnceLock<Regex> = OnceLock::new();

        // ⚡ Bolt: Using explicit expect context to aid debugging and avoid bare panics
        let re_uuid = RE_UUID.get_or_init(|| Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").expect("Invalid hardcoded regex"));
        let re_hex = RE_HEX.get_or_init(|| Regex::new(r"0x[0-9a-fA-F]+").expect("Invalid hardcoded regex"));
        let re_path = RE_PATH.get_or_init(|| Regex::new(r"/[a-zA-Z0-9\._\-/]+").expect("Invalid hardcoded regex"));
        let re_months = RE_MONTHS.get_or_init(|| Regex::new(r"(?i)(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)").expect("Invalid hardcoded regex"));
        let re_time = RE_TIME.get_or_init(|| Regex::new(r"\d{1,2}:\d{2}").expect("Invalid hardcoded regex"));
        let re_num = RE_NUM.get_or_init(|| Regex::new(r"\d+").expect("Invalid hardcoded regex"));

        // ⚡ Bolt: Avoid variable extraction and use into_owned() to prevent allocating a new string if no replacements are made.
        let s = re_uuid.replace_all(line, "<UUID>");
        let s = re_hex.replace_all(&s, "<HEX>");
        let s = re_path.replace_all(&s, "<PATH>");
        let s = re_months.replace_all(&s, "<MONTH>");
        let s = re_time.replace_all(&s, "<TIME>");
        let s = re_num.replace_all(&s, "<NUM>");

        s.into_owned()
    }"""
apply_diff("src/engine/discovery.rs", search_extract, replace_extract)

# Patch discovery.rs - process_and_check_noise
search_process = """    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
        if self.synthesize_line(line, handler, command) { return true; }
        let (template, vars) = self.extract_parts(line);

        let count = self.templates.entry(template.clone()).or_insert(0);

        // If we already have high confidence in this pattern (e.g. loaded from DB with high frequency),
        // collapse it immediately. Otherwise, wait for the threshold.
        if *count > self.threshold {
            self.variable_buffer.entry(template).or_default().push(vars);
            return true;
        }

        *count += 1;
        if *count > self.threshold {
            self.variable_buffer.entry(template).or_default().push(vars);
            true
        } else {
            false
        }
    }"""
replace_process = """    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
        if self.synthesize_line(line, handler, command) { return true; }
        let template = self.extract_parts(line);

        let count = self.templates.entry(template.clone()).or_insert(0);

        // If we already have high confidence in this pattern (e.g. loaded from DB with high frequency),
        // collapse it immediately. Otherwise, wait for the threshold.
        if *count > self.threshold {
            *self.variable_buffer.entry(template).or_insert(0) += 1;
            return true;
        }

        *count += 1;
        if *count > self.threshold {
            *self.variable_buffer.entry(template).or_insert(0) += 1;
            true
        } else {
            false
        }
    }"""
apply_diff("src/engine/discovery.rs", search_process, replace_process)

# Patch discovery.rs - get_saved_bytes
search_saved = """        for (template, var_sets) in &self.variable_buffer {
            total += template.len() * var_sets.len();
        }"""
replace_saved = """        for (template, _match_count) in &self.variable_buffer {
            // ⚡ Bolt: Calculate actual memory footprint by key capacity and value size.
            total += template.capacity() + std::mem::size_of::<usize>();
        }"""
apply_diff("src/engine/discovery.rs", search_saved, replace_saved)

# Patch discovery.rs - flush_variable_summary
search_flush = """        for (template, var_sets) in std::mem::take(&mut self.variable_buffer) {
            if var_sets.len() > 1 {
                summaries.push(format!("Line matched {} more times: {}", var_sets.len(), template));
            }
        }"""
replace_flush = """        for (template, count) in std::mem::take(&mut self.variable_buffer) {
            if count > 1 {
                summaries.push(format!("Line matched {} more times: {}", count, template));
            }
        }"""
apply_diff("src/engine/discovery.rs", search_flush, replace_flush)

print("Done patching discovery.")
