# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado.
- **Banderas (Flags)**:
  - `--raw`: Omite todo el procesamiento y síntesis de Axiom. Muestra el flujo exacto del proceso hijo.
  - `--markdown`: Habilita la transformación automática de tablas de terminal al formato Markdown.
  - `--yes`: Responde automáticamente que sí a todas las indicaciones.

### `axiom install`
Instala la integración de shell de Axiom y el contexto de IA.
- **Uso**: `axiom install`
- **Banderas (Flags)**:
  - `--path <PATH>`: Ruta del proyecto para sincronizar el contexto de IA (por defecto: directorio actual).
  - `--context-only`: Solo instala el contexto de IA, salta los alias del shell.

### `axiom uninstall`
Elimina todo rastro de Axiom del sistema.
- **Uso**: `axiom uninstall`
- **Banderas (Flags)**:
  - `--path <PATH>`: Ruta del proyecto para eliminar el contexto de IA (por defecto: directorio actual).

### `axiom doctor`
Ejecuta diagnósticos y comprobaciones de salud del sistema.
- **Uso**: `axiom doctor`
- **Banderas (Flags)**:
  - `--path <PATH>`: Ruta del proyecto para revisar el contexto de IA (por defecto: directorio actual).
  - `--fix`: Intenta arreglar automáticamente los problemas detectados.

### `axiom self-update`
Actualiza Axiom a la última versión disponible en GitHub.
- **Uso**: `axiom self-update`

### `axiom last`
Muestra la salida en bruto del último comando ejecutado.
- **Uso**: `axiom last`
- **Banderas (Flags)**:
  - `--tail <LINES>`: Número de líneas a mostrar desde el final.
  - `--grep <KEYWORD>`: Filtra las líneas por una palabra clave.

### `axiom check-ai`
Verifica si el proceso actual fue llamado por un agente de IA.
- **Uso**: `axiom check-ai`

### `axiom discovery`
Lista o gestiona las plantillas estructurales aprendidas actualmente.
- **Subcomandos**:
  - `list`: Muestra todas las plantillas aprendidas (por defecto).
  - `clear`: Elimina todos los patrones aprendidos.
  - `forget <PATTERN>`: Olvida un patrón de plantilla específico.

### `axiom intent`
Gestiona los niveles de inteligencia y filtrado de relevancia.
- **Subcomandos**:
  - `enable <mode>`: Habilita el filtrado basado en la intención. Modos: `fuzzy` (por defecto), `neural`.
  - `disable`: Configura la inteligencia al Nivel 1 (OFF). Solo se procesa la estructura y privacidad.
  - `status`: Muestra el modo de inteligencia actual y la intención descubierta.

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas (Flags)**:
  - `--history`, `-s`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

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

### `axiom config`
Gestión de la configuración.
- **Subcomandos**:
  - `init`: Inicializa un archivo local `.axiom.yaml` con valores predeterminados.
  - `show`: Muestra la configuración actual.
  - `set <KEY> <VALUE>`: Establece un valor de configuración (ej. `config set intelligence neural`).
  - *(Modo Interactivo)*: Ejecutar `axiom config` sin subcomandos abre un menú interactivo para configurar el modo de inteligencia, soporte markdown, telemetría, patrones de privacidad y fuentes de intención.
