# Axiom WASM Plugin Guide

This guide explains how to create, compile, and use external plugins to extend Axiom's semantic processing capabilities.

## 1. Why WASM?
Axiom uses WebAssembly for its plugin system to ensure:
- **Security**: Plugins run in a restricted sandbox.
- **Performance**: Near-native execution speed.
- **Portability**: Write plugins in Rust, C, Go, or any language that targets WASM.

## 2. Plugin Interface (ABI)
Every Axiom plugin must export two core functions:

1.  `axiom_alloc(size: u32) -> u32`: Allocates memory in the plugin for the host to write the input string.
2.  `axiom_transform(ptr: u32, len: u32) -> u64`: Processes the string. Returns a 64-bit integer where the high 32 bits are the pointer to the result and the low 32 bits are the length.

## 3. Creating a Plugin in Rust

### Step 1: Initialize project
```bash
cargo new --lib my-axiom-plugin
cd my-axiom-plugin
```

### Step 2: Configure `Cargo.toml`
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
# No dependencies needed for simple logic
```

### Step 3: Implement logic (`src/lib.rs`)
Here is a template for a plugin that converts all text to uppercase:

```rust
use std::mem;

#[no_mangle]
pub extern "C" fn axiom_alloc(size: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn axiom_transform(ptr: *mut u8, len: u32) -> u64 {
    // 1. Read input from Axiom
    let input = unsafe { 
        String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len as usize)) 
    };

    // 2. Perform custom logic (e.g., Uppercase)
    let result = input.to_uppercase();

    // 3. Return pointer and length to Axiom
    let result_len = result.len() as u64;
    let result_ptr = result.as_ptr() as u64;
    
    mem::forget(result); // Don't drop the string yet!
    
    (result_ptr << 32) | result_len
}
```

### Step 4: Compile to WASM
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

### Step 5: Install
Copy the resulting `.wasm` file to Axiom's plugin directory:
```bash
mkdir -p config/plugins
cp target/wasm32-unknown-unknown/release/my_axiom_plugin.wasm config/plugins/
```

## 4. How Axiom Executes Plugins
Axiom executes plugins in the order they are found in the `config/plugins` directory. The output of one plugin becomes the input of the next. If a plugin returns an empty string, the line is effectively collapsed.
