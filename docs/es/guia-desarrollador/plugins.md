# Guía de Plugins WASM para Axiom

Esta guía explica cómo crear, compilar y usar plugins externos para extender las capacidades de procesamiento semántico de Axiom.

## 1. ¿Por qué WASM?
Axiom utiliza WebAssembly para su sistema de plugins para asegurar:
- **Seguridad**: Los plugins se ejecutan en un sandbox restringido.
- **Rendimiento**: Velocidad de ejecución cercana a la nativa.
- **Portabilidad**: Puedes escribir plugins en Rust, C, Go o cualquier lenguaje que compile a WASM.

## 2. Interfaz del Plugin (ABI)
Cada plugin de Axiom debe exportar dos funciones principales:

1.  `axiom_alloc(size: u32) -> u32`: Reserva memoria en el plugin para que el host escriba la cadena de entrada.
2.  `axiom_transform(ptr: u32, len: u32) -> u64`: Procesa la cadena. Devuelve un entero de 64 bits donde los 32 bits superiores son el puntero al resultado y los 32 bits inferiores son la longitud.

## 3. Crear un Plugin en Rust

### Paso 1: Inicializar proyecto
```bash
cargo new --lib mi-plugin-axiom
cd mi-plugin-axiom
```

### Paso 2: Configurar `Cargo.toml`
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
# No se necesitan dependencias para lógica simple
```

### Paso 3: Implementar la lógica (`src/lib.rs`)
Aquí tienes una plantilla para un plugin que convierte todo el texto a mayúsculas:

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
    // 1. Leer entrada desde Axiom
    let input = unsafe { 
        String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len as usize))
    };

    // 2. Realizar lógica personalizada (ej., Mayúsculas)
    let result = input.to_uppercase();

    // 3. Retornar puntero y longitud a Axiom
    let result_len = result.len() as u64;
    let result_ptr = result.as_ptr() as u64;
    
    mem::forget(result); // ¡No eliminar la cadena todavía!
    
    (result_ptr << 32) | result_len
}
```

### Paso 4: Compilar a WASM
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

### Paso 5: Instalar
Copia el archivo `.wasm` resultante al directorio de plugins de Axiom:
```bash
mkdir -p config/plugins
cp target/wasm32-unknown-unknown/release/mi_plugin_axiom.wasm config/plugins/
```

## 4. Cómo ejecuta Axiom los Plugins
Axiom ejecuta los plugins en el orden en que se encuentran en el directorio `config/plugins`. La salida de un plugin se convierte en la entrada del siguiente. Si un plugin devuelve una cadena vacía, la línea se colapsa efectivamente.
