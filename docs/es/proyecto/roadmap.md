# AXIOM: Roadmap de Desarrollo y Herramientas

Este Roadmap combinado define la ruta de implementación de **AXIOM**, priorizando la seguridad, la latencia mínima y la expansión de esquemas inteligentes.

---

## 🏗️ Roadmap de Desarrollo del Núcleo

### Fase 1: Cimientos y Seguridad (Alpha) - [COMPLETADO]
**Objetivo**: Construir el interceptor de flujo y el sistema de protección de datos.
- [x] Proxy CLI, captura no bloqueante, escáner de entropía, redacción de PII y motor YAML.
- [x] **Optimización de Rendimiento**: Se logró una latencia de inicio <10ms (~9ms en Release) mediante el ajuste de SQLite.

### Fase 2: Inteligencia y Contexto (Beta) - [EN PROGRESO]
**Objetivo**: Hacer que Axiom entienda el "por qué" detrás de la ejecución de comandos.
#### 2.1 Auto-Intento e Integración
- [x] Contexto manual mediante variables de entorno.
- [x] Auto-descubrimiento de logs de chat (Cursor, Claude, Gemini CLI) para extraer contexto silenciosamente.
- [x] **Detective de Procesos**: Prefijado automático solo cuando un comando es lanzado por un agente de IA.
- [x] **Contexto de Git**: Priorización automática de los archivos modificados actualmente.
- [ ] **Siguiente**: "Local Shims" (Sobrescritura de binarios a nivel de proyecto en `.axiom/bin`).

#### 2.2 Transformador Semántico
- [x] Lógica de prioridad de intención (Anulación de Intento).
- [x] Coincidencia de relevancia básica basada en palabras clave.

#### 2.3 Persistencia y Analíticas
- [x] Integración de SQLite para el historial local y memoria de plantillas.

### Fase 3: Aprendizaje y Ecosistema (Gamma) - [EN PROGRESO]
**Objetivo**: Automatización total y escalabilidad comunitaria.
- [ ] **Toolset para Desarrolladores**: Expansión de esquemas por defecto para Linux (Ver sección Toolset abajo).
- [x] Agregador Inteligente: Buffer variable y Resumen Sintético.

#### 3.2 Sistema de Plugins WASM - [COMPLETADO]
- [x] Soporte para filtros complejos escritos en WebAssembly.
- [x] Aislamiento total de plugins de terceros.
- [x] **Guía para Desarrolladores de Plugins**: Documentación completa para lógica externa.

#### 3.3 Hub Universal de Schemas
- [ ] Sincronización con un repositorio central de esquemas.
- [ ] Compartición de plantillas estructurales anonimizadas.

### Fase 3.5: Laboratorio de Validación (Battle Arena) - [COMPLETADO]
- [x] **Suite de Benchmarks**: Script para comparar la salida Raw vs Axiom usando LLMs reales (Ollama/Groq).
- [x] **Métricas de Tokens**: Cálculo automatizado del ahorro de tokens por tipo de comando.
- [x] **Feedback de Instrucciones**: Eficacia de [agentes.md](../integracion-ia/agentes.md) verificada con el agente Gemini.

### Fase 4: IA Local e Inteligencia Semántica (Visión) - [EN PROGRESO]
**Objetivo**: Ir más allá de las palabras clave hacia el verdadero significado.

#### 4.1 Embeddings Locales (SLM)
- [x] Integración de **Candle** (Rust puro) para similitud vectorial local.
- [x] Reemplazar/Aumentar la coincidencia de palabras clave con **Similitud Semántica** (basada en BERT).
- [x] Estrategia híbrida: Palabra clave -> Fuzzy -> Neuronal.

#### 4.2 Agregador Neuronal
- [ ] Usar un Modelo de Lenguaje Pequeño (SLM) para narrar resúmenes de logs repetitivos.
- [ ] Detección de anomalías: Resaltar logs que se desvían de la norma estructural.

---

## 🛠️ Revisión Crítica y Evolución Arquitectónica (v0.1.0)

Basado en la auditoría técnica del proyecto, se han identificado los siguientes ejes de mejora crítica:

### 1. Optimización del Pipeline de Inteligencia (Performance)
*   **Problema**: El motor `NeuralIntelligence` (BERT) calcula embeddings en cada línea, inviable en CPU.
*   **Acción**:
    - [x] Implementar **Caching de Intent Embeddings**: Calcular una sola vez por sesión.
    - [ ] **Estrategia Híbrida Agresiva**: Neural como "árbitro" final.
    - [ ] Explorar modelos más ligeros (FastText/SLMs).

### 2. Integración con la Terminal (Fidelidad)
*   **Problema**: Uso de Pipes rompe la interactividad y colores.
*   **Acción**:
    - [ ] **Migración a PTY (Pseudo-Terminales)**.

### 3. Refinamiento de la Privacidad (Falsos Positivos)
*   **Problema**: Entropía genera falsos positivos (Hashes, IDs).
*   **Acción**:
    - [ ] **Context-Aware Redaction**: Lista blanca de patrones (SHA, SemVer).
    - [ ] Ajustar dinámicamente umbrales según `ToolSchema`.

### 4. Robustez de la IA Local (Resiliencia)
*   **Problema**: Descarga en caliente rompe promesa Local-First.
*   **Acción**:
    - [ ] Comando `axiom setup` para pre-cargar modelos.
    - [ ] Mecanismos de *Graceful Degradation*.

---

## 💎 Fase 5: Economía de Tokens Avanzada (Inspirado en RTK)
**Objetivo**: Maximizar el Retorno de Inversión (ROI) de cada token y automatizar la evolución del sistema.

### 5.1 Motor de Predicción y ROI de Tokens
- [ ] **Axiom Gain**: Panel de análisis avanzado que muestra ahorros acumulativos en USD/Tokens.
- [ ] **Advertencia Predictiva**: Alerta a los agentes cuando un comando (ej. `cat` en un archivo enorme) excederá un "Presupuesto de Tokens".
- [ ] **Arbitraje Económico**: Sugerir alternativas más económicas (ej. `grep` vs `cat | grep`) antes de la ejecución.

### 5.2 Bucle de Aprendizaje Autónomo (`axiom learn`)
- [ ] **Descubrimiento de Patrones**: Analizar el historial del shell para identificar comandos de "alto ruido" sin esquemas.
- [ ] **Autogeneración de Esquemas**: Usar LLM para sugerir esquemas YAML basados en salidas ruidosas capturadas.
- [ ] **Corrección de Errores**: Aprender de "Reintentos de Agente" (ej. si un agente ejecuta `ls` luego `ls -a`, Axiom debería ajustar el esquema `ls` predeterminado para ese contexto).

### 5.3 Síntesis Estructural Profunda
- [ ] **Modo Solo-Esquema**: Transformar objetos JSON/YAML masivos en "Resúmenes de Forma" (solo claves y tipos).
- [ ] **Diff Semántico**: Diffs ultra condensados que priorizan cambios lógicos sobre espacios en blanco o actualizaciones triviales.
- [ ] **Minificador Universal**: Un modo de compresión "con pérdida" para logs que preserva el significado semántico mientras destruye el 90% de los caracteres.

---

## 🧰 Roadmap de Expansión del Toolset para Desarrolladores

Esta sección define la expansión de esquemas predeterminados y modos inteligentes para desarrolladores de Linux.

### 🟢 Nivel 1: Los Fundamentos (Alta Frecuencia)
*Objetivo: Eliminar el ruido estructural de los comandos diarios.*
- [x] **ls / tree**: Colapsar archivos ocultos, metadatos y directorios basura.
- [x] **cat / tail / head**: "Modo Guardián" para archivos > 50 líneas (auto-resumen).
- [ ] **grep / rg (ripgrep)**: Agregar coincidencias por archivo y proporcionar resúmenes de densidad.
- [x] **curl / wget**: Ocultar barras de progreso y cabeceras HTTP redundantes.

### 🟡 Nivel 2: Ecosistemas de Construcción y Desarrollo (Contexto)
*Objetivo: Filtrar boilerplate de éxito y enfocarse en advertencias/errores.*
- [x] **npm / pnpm / yarn**: Reducción básica de ruido del instalador.
- [ ] **cargo (Rust)**: Colapsar la descarga/compilación de dependencias. Mostrar forzosamente advertencias de crates locales.
- [ ] **go build / test**: Resumir resultados de pruebas.
- [ ] **pip / poetry / conda**: Limpiar registros de configuración de virtualenv e instalación de paquetes.

### 🟠 Nivel 3: Infraestructura y Nube (Control de Volumen)
*Objetivo: Prevenir la saturación de la ventana de contexto por salidas masivas.*
- [x] **docker / docker-compose**: Colapsar progreso de descarga de capas y bucles de health-check.
- [x] **kubectl**: Resumir estados de pods, limpiar descripciones de recursos.
- [x] **terraform**: Sintetizar `terraform plan`.
- [x] **aws / gcloud / az**: Transformar listados masivos de JSON/Tablas en resúmenes densos.

### 🔵 Nivel 4: Datos y Sistema (Síntesis Estructural)
*Objetivo: Mantener la forma de los datos reduciendo el conteo de tokens.*
- [x] **jq / yq**: Identificar la estructura JSON y resumir arrays.
- [x] **ps / journalctl**: Limpieza profunda de ruido del sistema/kernel.
- [x] **netstat / lsof / ss**: Filtrar puertos reservados del sistema.

### 🚀 Modos Inteligentes Avanzados (Flags de Comportamiento)
- [x] **`--markdown`**: Transformar automáticamente las tablas de salida en tablas Markdown reales.
- **`--diff-only`**: Mostrar solo lo que ha cambiado desde la última ejecución.
- **`--explain`**: Anteponer un resumen en lenguaje natural de lo que Axiom comprimió.
