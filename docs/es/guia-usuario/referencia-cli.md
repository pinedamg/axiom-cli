# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado.
- **Banderas (Flags)**:
  - `--raw`: Omite todo el procesamiento y la síntesis de Axiom. Muestra el flujo exacto del proceso hijo.
  - `--markdown`: Habilita la transformación automática de tablas de terminal al formato Markdown.

### `axiom install`
Instala la integración de la consola de Axiom y el contexto de IA.
- **Uso**: `axiom install`
- **Banderas (Flags)**:
  - `-p, --path`: Ruta del proyecto para sincronizar el contexto de IA (por defecto: directorio actual).
  - `--context-only`: Solo instala el contexto de IA, omite los alias de la consola.

### `axiom doctor`
Ejecuta la verificación de salud del sistema y los diagnósticos.
- **Uso**: `axiom doctor`
- **Banderas (Flags)**:
  - `-p, --path`: Ruta del proyecto para verificar el contexto de IA (por defecto: directorio actual).

### `axiom selfupdate`
Actualiza Axiom a la última versión desde GitHub.
- **Uso**: `axiom selfupdate`

### `axiom checkai`
Verifica si el proceso actual fue llamado por un agente de IA. Devuelve código de salida 0 si es IA, 1 si es humano.
- **Uso**: `axiom checkai`

### `axiom discovery`
Lista las plantillas estructurales aprendidas actualmente según el motor de Descubrimiento.
- **Uso**: `axiom discovery`

### `axiom intent`
Gestiona la inteligencia y los niveles de filtrado de relevancia.
- **Subcomandos**:
  - `enable <mode>`: Habilita el filtrado basado en la intención. Modos: `fuzzy` (por defecto), `neural`.
  - `disable`: Establece la inteligencia al Nivel 1 (OFF). Solo se procesa la estructura y la privacidad.
  - `status`: Muestra el modo de inteligencia actual y la intención descubierta.

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas (Flags)**:
  - `--history`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

### `axiom status`
Muestra la salud actual, la configuración y el estado de la telemetría de tu instalación de Axiom.
- **Uso**: `axiom status`
- **Salida**: Edición (Community/Pro), Nivel de Telemetría, ID de Instalación y esquemas activos.

### `axiom proxy <cmd>`
Ejecuta el comando en bruto sin filtrado. Útil para depuración o para saltarse Axiom por completo en una ejecución específica.
- **Uso**: `axiom proxy npm install`

### `axiom discover`
*(Beta)* Analiza el historial de agentes de IA locales (como Claude Code) para encontrar oportunidades perdidas donde Axiom podría haber ahorrado tokens.
- **Uso**: `axiom discover`

## Comandos de Configuración

### `axiom config init`
Inicializa un archivo de configuración local `.axiom.yaml` con los valores por defecto.
- **Uso**: `axiom config init`

### `axiom config telemetry <nivel>`
Establece tu nivel de telemetría preferido.
- **Niveles**: `full`, `discovery`, `anonymous`, `off` (solo Pro).
- **Ejemplo**: `axiom config telemetry discovery`

### `axiom config license <clave>`
Aplica una clave de licencia Pro para desbloquear funciones premium como el modo de telemetría Offline.
- **Ejemplo**: `axiom config license abc-123-xyz`
