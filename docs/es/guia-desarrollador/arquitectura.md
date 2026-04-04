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
    - **Transformer**: Aplica las reglas de transformación (ej. formato Markdown, umbrales del Guardián).
    - **Plugins**: Ejecuta lógica de terceros a través de WebAssembly aislado.

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

1.  **Ejecución de Comando**: Inicia `axiom npm install`.
2.  **Detective de Procesos**: Identifica `npm` y el contexto actual del proyecto.
3.  **Captura de Flujo**: Se leen bytes en bruto del subproceso.
4.  **Deduplicate**: El Engine descarta líneas duplicadas de forma secuencial.
5.  **Transform**: Ajustes de formato (ej. conversión a Markdown).
6.  **Guard**: Aborta o limita la salida para procesos extremadamente ruidosos (Modo Guardián).
7.  **Redact (Escudo de Privacidad)**: Las líneas se escanean y se redactan si es necesario.
8.  **Analyze**: Validación de esquemas, resumen estructural y ejecución de Intención Semántica.
9.  **Plugins**: Las líneas mutan opcionalmente a través de filtros WASM.
10. **Salida Final**: La salida de alta señal se imprime en la terminal para que el agente de IA la consuma.
11. **Analíticas**: Se calculan los ahorros y se almacenan en la BD local SQLite.

## 4. Estándares de Seguridad

- **Política de Cero Logs**: Los datos capturados en bruto **nunca** se escriben en los logs propios de Axiom ni en la telemetría.
- **Local-First (Local Primero)**: Todo el trabajo pesado (Redacción, embeddings BERT, transformación) ocurre localmente en la CPU del usuario.
