# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado.

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas (Flags)**:
  - `--history`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

### `axiom status`
Muestra la salud actual, la configuración y el estado de la telemetría de tu instalación de Axiom.
- **Uso**: `axiom status`
- **Salida**: Edición (Community/Pro), Nivel de Telemetría, ID de Instalación y esquemas activos.

### `axiom check-ai`
Verifica si el proceso actual fue llamado por un agente de IA. Utiliza el `ProcessDetective` para determinar si el proceso padre es un agente de IA conocido (como Cursor, Claude Code, o Gemini CLI) o una terminal humana.
- **Uso**: `axiom check-ai`

### `axiom discovery`
Lista todas las plantillas estructurales aprendidas actualmente que el motor Discovery Engine ha sintetizado de ejecuciones de comandos anteriores.
- **Uso**: `axiom discovery`

### `axiom proxy <cmd>`
Ejecuta el comando en bruto sin filtrado. Útil para depuración o para saltarse Axiom por completo en una ejecución específica.
- **Uso**: `axiom proxy npm install`

### `axiom discover`
*(Beta)* Analiza el historial de agentes de IA locales (como Claude Code) para encontrar oportunidades perdidas donde Axiom podría haber ahorrado tokens.
- **Uso**: `axiom discover`

## Gestión de Intención (Intent)

### `axiom intent enable <mode>`
Habilita la inteligencia de intención. El modo (`mode`) puede ser `fuzzy` (basado en palabras clave) o `neural` (usando embeddings de IA locales).
- **Uso**: `axiom intent enable fuzzy` o `axiom intent enable neural`

### `axiom intent disable`
Deshabilita la inteligencia de intención. Axiom mantendrá el formato y la redacción de privacidad, pero no ocultará archivos ni salidas basadas en su relevancia.
- **Uso**: `axiom intent disable`

### `axiom intent status`
Muestra el estado actual del descubrimiento de intención, el modo de inteligencia, el ID de la sesión y los archivos/intenciones relevantes detectados desde el proceso padre.
- **Uso**: `axiom intent status`

## Comandos de Configuración

### `axiom config init`
Inicializa un archivo de configuración local `.axiom.yaml` en el directorio actual con los valores predeterminados.
- **Uso**: `axiom config init`

### `axiom config telemetry <nivel>`
Establece tu nivel de telemetría preferido.
- **Niveles**: `full`, `discovery`, `anonymous`, `off` (solo Pro).
- **Ejemplo**: `axiom config telemetry discovery`

### `axiom config license <clave>`
Aplica una clave de licencia Pro para desbloquear funciones premium como el modo de telemetría Offline.
- **Ejemplo**: `axiom config license abc-123-xyz`

## Banderas Globales (Flags)

### `--markdown` (`-m`)
Habilita la transformación de tablas a Markdown. Cuando se pasa esta bandera, Axiom intenta detectar salidas tabulares (como de `ps` o `docker ps`) y las transforma en tablas Markdown estándar para una mejor lectura por parte del LLM.
- **Uso**: `axiom --markdown <comando>`
