## Memory Footprint of `DiscoveryEngine`

`DiscoveryEngine` holds the state for line buffering and summary synthesis.
When stream output is heavily templated, the `variable_buffer` holds `Vec<Vec<String>>` tracking all matching components for every matching template.

```rust
pub variable_buffer: BTreeMap<String, Vec<Vec<String>>>,
```

If a pipeline yields 100k lines, and many of those match templates, the size of this `variable_buffer` will grow proportionally, accumulating large amounts of `Vec<String>`.
If we only report `summaries.push(format!("Line matched {} more times: {}", var_sets.len(), template));` we only care about the *count* of `var_sets`.
The contents of `var_sets` (`Vec<String>`) are never actually used for the summary! Wait, let's look closer at `flush_variable_summary`.

```rust
        for (template, var_sets) in std::mem::take(&mut self.variable_buffer) {
            if var_sets.len() > 1 {
                summaries.push(format!("Line matched {} more times: {}", var_sets.len(), template));
            }
        }
```

Indeed, the variable extractions are just pushed to the buffer, but we never use those variables later, only their count!
Wait, in `flush_variable_summary`, it just does `var_sets.len()`. This means we are allocating and storing huge amounts of string extractions (`Vec<String>`) for no reason other than to count them later.
We can change `variable_buffer` from `BTreeMap<String, Vec<Vec<String>>>` to `BTreeMap<String, usize>` (just the count), dropping the allocation completely.
Wait, `extract_parts` builds a vector of variables. We only need the variable part extracted to know if we've correctly masked out dynamic content (by returning the string). So `extract_parts` can just return `String`!

Let's check `extract_parts`:
```rust
    pub fn extract_parts(&self, line: &str) -> (String, Vec<String>) {
// ...
        // Pre-allocate variable list to avoid reallocations
        let mut variables = Vec::with_capacity(8);

        let s = re_uuid.replace_all(line, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<UUID>" });
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<HEX>" });
        let s = re_path.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<PATH>" });
        let s = re_months.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<MONTH>" });
        let s = re_time.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<TIME>" });
        let s = re_num.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<NUM>" });

        (s.to_string(), variables)
    }
```
We can replace `extract_parts` to just return `String` and remove the overhead of variable tracking, or change `variable_buffer` to `BTreeMap<String, usize>` and `extract_parts` to return just `String`. Wait, if we return just `String`, we can also use `.replace_all(line, "<UUID>")` directly.

Wait, the Regex `replace_all` method takes a `Replacer`. A string slice `"<UUID>"` is a valid `Replacer` and it's much faster than allocating and calling a closure!

So we can rewrite `extract_parts`:
```rust
    pub fn extract_parts(&self, line: &str) -> String {
// ...
        let s = re_uuid.replace_all(line, "<UUID>");
        let s = re_hex.replace_all(&s, "<HEX>");
        let s = re_path.replace_all(&s, "<PATH>");
        let s = re_months.replace_all(&s, "<MONTH>");
        let s = re_time.replace_all(&s, "<TIME>");
        let s = re_num.replace_all(&s, "<NUM>");

        s.to_string()
    }
```
This is a huge allocation win because we don't need to push matching variables to a `Vec` for each pattern matched.

And in `process_and_check_noise`:
```rust
    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
        if self.synthesize_line(line, handler, command) { return true; }
        let template = self.extract_parts(line);

        let count = self.templates.entry(template.clone()).or_insert(0);

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
    }
```
And in `flush_variable_summary`:
```rust
        for (template, match_count) in std::mem::take(&mut self.variable_buffer) {
            if match_count > 1 {
                summaries.push(format!("Line matched {} more times: {}", match_count, template));
            }
        }
```
This fits exactly the Bolt memory optimization brief.
Wait, let's make sure the return type of `get_saved_bytes` doesn't break.
```rust
    pub fn get_saved_bytes(&self) -> usize {
        // Approximate saved bytes from synthesis and variable buffers
        let mut total = 0;
        for items in self.synthesis_buffer.values() {
            for item in items {
                total += item.name.len() + 20; // Plus overhead
            }
        }
        for (template, match_count) in &self.variable_buffer {
            total += template.len() * match_count;
        }
        total
    }
```
Currently it is:
```rust
        for (template, var_sets) in &self.variable_buffer {
            total += template.len() * var_sets.len();
        }
```
So changing `var_sets.len()` to `match_count` is perfectly fine!

Let's double check `process_and_check_noise` where it handles threshold:
```rust
    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
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
    }
```
Changed:
```rust
    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
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
    }
```
And the field definition:
```rust
    pub variable_buffer: BTreeMap<String, usize>,
```
In `Default`:
```rust
            variable_buffer: BTreeMap::new(),
```

This is an extremely obvious memory optimization (dropping continuous allocation of `Vec<String>`).
What about `get_key` in `ps.rs`? I noticed a comment:
```rust
    fn get_category(&self, _perms: &str) -> String {
        // We use is_dir as a proxy flag for kernel processes in our ps implementation
        // This is a bit of a hack but it's KISS for this specific tool.
        // Actually, we should check meta during get_key, but the trait only gives perms.
        // Let's improve the trait to pass metadata or just handle it in get_key.
        "PROC".to_string()
    }
```
Wait, the memory memory notes from the prompt:
- `In Axiom, CommandHandler::get_category accepts a &LineMetadata reference (e.g., _meta: &LineMetadata) instead of a string slice, allowing command handlers to utilize the full context (such as the is_dir flag or perms) to accurately determine categories.`
Ah!
Let's see the trait definition of `CommandHandler` in `src/engine/commands/mod.rs`.
Wait, `CommandHandler::get_category` should use `&LineMetadata` per the rules, but that's a different change from my Bolt memory PR. However, the rule explicitly says:
`In Axiom, CommandHandler::get_category accepts a &LineMetadata reference (e.g., _meta: &LineMetadata) instead of a string slice, allowing command handlers to utilize the full context (such as the is_dir flag or perms) to accurately determine categories.`
Wait, if it is a rule, then I should probably update `get_category` as well, or is the prompt strictly about memory optimization?
"Misión: Sos un ingeniero de sistemas obsesionado con la eficiencia de memoria y el rendimiento de bajo nivel en Rust."
"Si encontrás una mejora medible, creá un PR con: Título: ⚡ Bolt: [Optimización de Memoria en X]"

Okay, the memory improvement here is to avoid allocating `Vec<String>` in `extract_parts` and avoiding `Vec<Vec<String>>` in `variable_buffer`.
Wait, there is another memory instruction:
"To avoid continuous heap allocations in line-processing hot paths (like Axiom's `stage_deduplicate`), reuse existing `Option<String>` buffers by extracting them with `.take()`, clearing them with `.clear()`, and appending new data with `.push_str()` instead of allocating a new `String` per loop."
Let's look at `stage_deduplicate`.
In `stage_deduplicate`, `self.discovery.last_line = Some(line.to_string());` allocates a new string every time a new line comes.
According to the instruction: "To avoid continuous heap allocations in line-processing hot paths (like Axiom's `stage_deduplicate`), reuse existing `Option<String>` buffers by extracting them with `.take()`, clearing them with `.clear()`, and appending new data with `.push_str()` instead of allocating a new `String` per loop."

Wait, how do we use `take`, `clear` and `push_str`?
```rust
    fn stage_deduplicate<'a>(&mut self, line: &'a str) -> (Option<String>, PipelineAction<'a>, String) {
        if self.discovery.last_line.as_deref() == Some(line) {
            self.discovery.repeat_count += 1;
            (None, PipelineAction::Swallow, "Identical to previous line".to_string())
        } else {
            let prefix = if self.discovery.repeat_count > 0 {
                Some(format!("... (previous line repeated {} more times)", self.discovery.repeat_count))
            } else { None };

            let mut buf = self.discovery.last_line.take().unwrap_or_default();
            buf.clear();
            buf.push_str(line);
            self.discovery.last_line = Some(buf);

            self.discovery.repeat_count = 0;
            (prefix, PipelineAction::Continue(Cow::Borrowed(line)), "New line".to_string())
        }
    }
```
This avoids allocating a brand new String for `last_line` when lines change.

Another memory optimization:
"In `PsHandler::generate_insight` (`src/engine/commands/ps.rs`), the top process name is tracked using an `Option<&str>` (referencing the data in the `DiscoveryBuffer`) to avoid redundant `String` clones inside the iteration loop."

Let's check `PsHandler::generate_insight`.
In `src/engine/commands/ps.rs`:
```rust
    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut max_cpu = 0.0;
        let mut top_proc = String::new(); // <-- Change to Option<&str>
```
Wait, the instruction literally says:
"In `PsHandler::generate_insight` (`src/engine/commands/ps.rs`), the top process name is tracked using an `Option<&str>` (referencing the data in the `DiscoveryBuffer`) to avoid redundant `String` clones inside the iteration loop."

So we change it to:
```rust
    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut max_cpu = 0.0;
        let mut top_proc: Option<&str> = None;
        let mut total_procs = 0;

        for (key, items) in buffer {
            if key.starts_with("PROC:") {
                total_procs += items.len();
                for item in items {
                    if let Ok(cpu) = item.size.parse::<f64>() {
                        if cpu > max_cpu {
                            max_cpu = cpu;
                            top_proc = Some(&item.name);
                        }
                    }
                }
            }
        }

        if total_procs > 0 {
            if max_cpu > 10.0 {
                let top_proc_name = top_proc.unwrap_or("unknown");
                Some(format!("High CPU load detected: {} is using {}% CPU. Total active processes: {}.", top_proc_name, max_cpu, total_procs))
            } else {
                Some(format!("System health stable. Total active processes: {}. No single process exceeding 10% CPU.", total_procs))
            }
        } else {
            None
        }
    }
```
This is awesome! Another memory optimization perfectly aligning with the memory profile objective.

Let's review the instructions:
1. `DiscoveryEngine` HashMap vs BTreeMap? Wait, the file already has `// ⚡ Bolt: Using BTreeMap instead of HashMap`.
2. "To avoid continuous heap allocations in line-processing hot paths (like Axiom's `stage_deduplicate`), reuse existing `Option<String>` buffers by extracting them with `.take()`, clearing them with `.clear()`, and appending new data with `.push_str()` instead of allocating a new `String` per loop."
3. "In `PsHandler::generate_insight` (`src/engine/commands/ps.rs`), the top process name is tracked using an `Option<&str>` (referencing the data in the `DiscoveryBuffer`) to avoid redundant `String` clones inside the iteration loop."
4. Avoid allocating variables in `extract_parts`! "Wait, `variable_buffer` holds `Vec<Vec<String>>`..." Changing `extract_parts` to just return `String` and `variable_buffer` to hold `usize` is super aligned with:
- "Analizá src/engine/mod.rs y src/engine/discovery.rs buscando acumulaciones de datos innecesarias en memoria (buffers que no se liberan, clones innecesarios)."
- "Implementá el uso de Capacity pre-asignada en Vec y String cuando el tamaño sea predecible para evitar re-asignaciones."

Let's do all these 3 modifications and then create a branch and run `cargo test -- --test-threads=1`. Wait, how is the testing done?
- "Execution plans must include an explicit, distinct step to run the test suite (e.g., `cargo test -- --test-threads=1`) immediately before the pre-commit step to satisfy the Completeness Rule."

Let's verify `PsHandler` tests.
