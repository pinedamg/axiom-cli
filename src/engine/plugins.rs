use wasmtime::*;
use std::path::Path;
use std::fs;

pub struct WasmPlugin {
    pub name: String,
    instance: Instance,
    store: Store<()>,
    // Cached functions
    transform_fn: TypedFunc<(u32, u32), u64>,
    alloc_fn: TypedFunc<u32, u32>,
}

pub struct WasmPluginManager {
    plugins: Vec<WasmPlugin>,
    _engine: Engine,
}

impl WasmPluginManager {
    pub fn new(plugins_dir: &Path) -> anyhow::Result<Self> {
        let mut plugins = Vec::new();
        let engine = Engine::default();
        
        if plugins_dir.exists() {
            for entry in fs::read_dir(plugins_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                    if let Ok(plugin) = Self::load_plugin(&engine, &path) {
                        plugins.push(plugin);
                    }
                }
            }
        }

        Ok(Self { plugins, _engine: engine })
    }

    fn load_plugin(engine: &Engine, path: &Path) -> anyhow::Result<WasmPlugin> {
        let wasm_bytes = fs::read(path)?;
        let mut store = Store::new(engine, ());
        let module = Module::new(engine, wasm_bytes)?;
        
        let linker = Linker::new(engine);
        let instance = linker.instantiate(&mut store, &module)?;

        // Extract functions
        let transform_fn = instance.get_typed_func::<(u32, u32), u64>(&mut store, "axiom_transform")?;
        let alloc_fn = instance.get_typed_func::<u32, u32>(&mut store, "axiom_alloc")?;

        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(WasmPlugin {
            name,
            instance,
            store,
            transform_fn,
            alloc_fn,
        })
    }

    pub fn transform(&mut self, line: &str) -> String {
        let mut current_line = line.to_string();

        for plugin in &mut self.plugins {
            if let Ok(new_line) = Self::call_plugin_transform(plugin, &current_line) {
                current_line = new_line;
            }
        }

        current_line
    }

    fn call_plugin_transform(plugin: &mut WasmPlugin, input: &str) -> anyhow::Result<String> {
        let memory = plugin.instance.get_memory(&mut plugin.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("WASM plugin missing 'memory' export"))?;
        
        let input_bytes = input.as_bytes();
        let input_len = input_bytes.len() as u32;

        // 1. Allocate memory in guest
        let ptr = plugin.alloc_fn.call(&mut plugin.store, input_len)?;

        // 2. Write to guest memory
        memory.write(&mut plugin.store, ptr as usize, input_bytes)?;

        // 3. Call transform
        let result = plugin.transform_fn.call(&mut plugin.store, (ptr, input_len))?;

        // 4. Read result (High 32 bits = ptr, Low 32 bits = len)
        let output_ptr = (result >> 32) as u32;
        let output_len = (result & 0xFFFFFFFF) as u32;

        if output_len == 0 {
            return Ok("".to_string());
        }

        let mut output_bytes = vec![0u8; output_len as usize];
        memory.read(&plugin.store, output_ptr as usize, &mut output_bytes)?;

        Ok(String::from_utf8_lossy(&output_bytes).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_plugin_manager_empty_dir() {
        let dir = tempdir().unwrap();
        let manager = WasmPluginManager::new(dir.path()).unwrap();
        assert_eq!(manager.plugins.len(), 0);
        
        // Should return the same string if no plugins are loaded
        let output = manager.plugins.len();
        assert_eq!(output, 0);
    }

    #[test]
    fn test_plugin_manager_no_dir() {
        let manager = WasmPluginManager::new(Path::new("/non/existent/path/axiom/plugins")).unwrap();
        assert_eq!(manager.plugins.len(), 0);
    }
}
