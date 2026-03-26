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

### `axiom check-ai`
Comprueba si el proceso actual fue invocado por un agente de IA (ej. Cursor, Claude Code) o una terminal humana.
- **Uso**: `axiom check-ai`

### `axiom intent status`
Muestra la salud actual, la configuración, la intención y el estado de la telemetría de tu instalación de Axiom.
- **Uso**: `axiom intent status`
- **Salida**: Session ID, Intelligence Mode, Parent Process y Last Intent.

### `axiom intent enable <modo>`
Activa la inteligencia de intención para filtrar la salida basándose en la intención del usuario/agente.
- **Modos**: `fuzzy` (palabras clave) o `neural` (embeddings de IA).
- **Uso**: `axiom intent enable neural`

### `axiom intent disable`
Desactiva la inteligencia de intención. Axiom mantendrá el formato pero mostrará todos los archivos sin filtrado por relevancia.
- **Uso**: `axiom intent disable`

### `axiom proxy <cmd>`
Ejecuta el comando en bruto sin filtrado. Útil para depuración o para saltarse Axiom por completo en una ejecución específica.
- **Uso**: `axiom proxy npm install`

### `axiom discovery`
Muestra las plantillas estructurales aprendidas actualmente y su frecuencia de uso en la sesión.
- **Uso**: `axiom discovery`

## Comandos de Configuración

### `axiom config init`
Inicializa un archivo `.axiom.yaml` de configuración local con valores por defecto.
- **Uso**: `axiom config init`

## Flags Globales

### `-m, --markdown`
Habilita la transformación automática de salidas alineadas por espacios en tablas Markdown reales.
- **Uso**: `axiom -m kubectl get pods`
