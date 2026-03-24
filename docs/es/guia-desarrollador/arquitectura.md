# AXIOM: Arquitectura del Sistema

Este documento describe la arquitectura de alto rendimiento y capas de **Axiom**. Está diseñada para una latencia mínima (<10ms) y máxima seguridad mientras procesa flujos de terminal.

## 1. Arquitectura de Alto Nivel

Axiom sigue una **Arquitectura Limpia por Capas (Layered Clean Architecture)** adaptada a las necesidades de rendimiento de Rust. Los datos fluyen a través de un pipeline de módulos especializados:

### 📥 1. Gateway (Capa de Ingreso)
- **Ubicación**: `src/gateway/`
- **Responsabilidad**: Interactúa con el Sistema Operativo. Captura `stdin`, `stdout` y `stderr` del proceso hijo.
- **Tecnología**: Usa `tokio::process` para E/S no bloqueante.
- **PTY (Futuro)**: Planes para migrar de tuberías (pipes) simples a Pseudo-Terminales para preservar colores e interactividad.

### 🛡️ 2. Privacy (Capa de Firewall)
- **Ubicación**: `src/privacy/`
- **Responsabilidad**: El primer punto de procesamiento. Asegura que los datos sensibles nunca salgan de la máquina.
- **Mecanismos**: 
    - **Escáner de Entropía**: Detecta cadenas de alta entropía (claves API, secretos) usando métricas de Entropía de Shannon.
    - **Redactor**: Enmascara PII (Correos, IPs, etc.) antes de que la siguiente capa vea los datos.

### 🧩 3. Schema (Capa de Dominio)
- **Ubicación**: `src/schema/`
- **Responsabilidad**: Define cómo entender varias herramientas CLI.
- **Lógica**: Carga archivos YAML desde `config/schemas/` y los compara con el comando actual.

### 🧠 4. Engine (Capa de Inteligencia)
- **Ubicación**: `src/engine/`
- **Responsabilidad**: El orquestador. Coordina:
    - **Discovery**: Identifica automáticamente la herramienta y su intención.
    - **Intelligence**: Usa coincidencia de palabras clave, fuzzy y neural (basada en BERT) para determinar la relevancia.
    - **Transformer**: Aplica las reglas de transformación (Colapsar, Descartar, Pasar).

### 📊 5. Persistence (Capa de Analíticas)
- **Ubicación**: `src/persistence/`
- **Responsabilidad**: Almacenamiento local para las analíticas de ahorro de tokens e historial de comandos.
- **Tecnología**: SQLite para almacenamiento estructurado local y rápido.

## 2. Pautas Técnicas

- **Lenguaje**: Rust (Edición 2021).
- **Asincronía**: `tokio` para E/S no bloqueante de alta concurrencia.
- **Serialización**: `serde` para el manejo de YAML y JSON.
- **Gestión de Errores**: `thiserror` para errores internos y `anyhow` para la superficie de la CLI.

## 3. Flujo de Datos (El Pipeline de Flujo)

El orquestador del motor (`src/engine/mod.rs`) procesa cada línea a través de un pipeline estricto:

1. **Fase 1: Preprocesamiento Estructural**: Transforma las líneas que parecen tablas en formato Markdown real si el modo markdown está habilitado.
2. **Fase 2: Protección de Recursos**: Previene desbordamientos de buffer o quema de tokens mediante la aplicación de límites de longitud de archivo (ej. resumiendo después de 100 líneas) y comprobaciones de ruido de auto-descubrimiento.
3. **Fase 3: Seguridad y Privacidad**: El paso obligatorio del `PrivacyRedactor`, donde se eliminan los patrones sensibles de la línea de trabajo.
4. **Fase 4: Relevancia Semántica**: Evalúa si la línea es explícitamente relevante para el `IntentContext` actual (a través de la anulación por prioridad de intención) y elude la compresión en caso de serlo.
5. **Fase 5: Compresión Basada en Patrones**: Coincide la línea con las reglas YAML de `ToolSchema`. Las líneas pueden mantenerse, colapsarse en resúmenes, redactarse o ocultarse completamente.
6. **Fase 6: Lógica Externa (Plugins WASM)**: Aplica plugins de WebAssembly cargados para transformaciones lógicas externas.

## 4. Estándares de Seguridad

- **Política de Cero Logs**: Los datos capturados en bruto **nunca** se escriben en los logs propios de Axiom ni en la telemetría.
- **Local-First (Local Primero)**: Todo el trabajo pesado (Redacción, embeddings BERT, transformación) ocurre localmente en la CPU del usuario.
