# Plugins WASM

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

## 3. Ejemplo en Rust

```rust
#[no_mangle]
pub extern "C" fn axiom_transform(ptr: u32, len: u32) -> u64 {
    let input = unsafe { 
        String::from_utf8_lossy(std::slice::from_raw_parts(ptr as *const u8, len as usize)) 
    };
    
    let output = input.replace("noise", "signal");
    
    // Retornar puntero y longitud empaquetados
    let ptr = output.as_ptr() as u32;
    let len = output.len() as u32;
    (ptr as u64) << 32 | len as u64
}
```

## 4. Instalación del Plugin
Coloca el archivo `.wasm` compilado en la carpeta de plugins de Axiom (por defecto `~/.axiom/plugins/`) y actívalo en tu configuración.
