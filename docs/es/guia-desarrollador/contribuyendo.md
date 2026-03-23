# Contribuyendo a Axiom

¡Damos la bienvenida a todas las contribuciones de la comunidad! Ya sea que quieras añadir un nuevo esquema, corregir un error o escribir un plugin WASM, tu ayuda es apreciada.

## Cómo Empezar

1. Haz un Fork del repositorio.
2. Clona tu fork localmente.
3. Asegúrate de tener Rust instalado (`rustup`).
4. Ejecuta `cargo build` para asegurar que todo compila.
5. Ejecuta `cargo test` para ejecutar la suite de pruebas.

## Formas de Contribuir

### 1. Añadiendo Schemas
La forma más fácil de contribuir es añadiendo soporte para nuevas herramientas CLI. Lee la guía de [Creación de Schemas](schemas.md) para aprender cómo crear definiciones YAML que filtren el ruido de tus herramientas favoritas.

### 2. Desarrollo del Núcleo (Core)
Si quieres trabajar en el núcleo en Rust (ej. el motor de telemetría, el escudo de privacidad o el descubrimiento de intención), consulta primero la [Guía de Arquitectura](arquitectura.md) para entender el diseño por capas.

### 3. Plugins WASM
Axiom soporta procesamiento complejo a través de WebAssembly. Consulta la [Guía de Plugins WASM](plugins.md) para detalles sobre la ABI y cómo escribir un plugin.

## Código de Conducta y CLA

Por favor, revisa el archivo `CONTRIBUTING.md` principal en la raíz del repositorio para conocer las pautas legales y de comportamiento. Todas las contribuciones deben adherirse a la Licencia Apache 2.0.
