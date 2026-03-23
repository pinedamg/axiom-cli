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

### `axiom proxy <cmd>`
Ejecuta el comando en bruto sin filtrado. Útil para depuración o para saltarse Axiom por completo en una ejecución específica.
- **Uso**: `axiom proxy npm install`

### `axiom discover`
*(Beta)* Analiza el historial de agentes de IA locales (como Claude Code) para encontrar oportunidades perdidas donde Axiom podría haber ahorrado tokens.
- **Uso**: `axiom discover`

## Comandos de Configuración

### `axiom config telemetry <nivel>`
Establece tu nivel de telemetría preferido.
- **Niveles**: `full`, `discovery`, `anonymous`, `off` (solo Pro).
- **Ejemplo**: `axiom config telemetry discovery`

### `axiom config license <clave>`
Aplica una clave de licencia Pro para desbloquear funciones premium como el modo de telemetría Offline.
- **Ejemplo**: `axiom config license abc-123-xyz`
