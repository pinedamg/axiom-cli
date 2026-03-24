# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Banderas Globales (Flags)

- **`--markdown`**: Habilita la transformación de tablas Markdown de forma global.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado.

### `axiom install`
Instala la integración de Axiom en tu shell.
- **Uso**: `axiom install`
- **Comportamiento**: Muestra los comandos necesarios para habilitar Axiom automáticamente en comandos comunes para tu configuración de shell.

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas (Flags)**:
  - `--history`, `-s`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

### `axiom discovery`
Lista las plantillas estructurales aprendidas actualmente.
- **Uso**: `axiom discovery`

### `axiom check-ai`
Verifica si el proceso actual fue llamado por un agente de IA.
- **Uso**: `axiom check-ai`
- **Comportamiento**: Retorna un código de salida 0 si fue llamado por un agente de IA, o 1 si fue llamado por un humano desde la terminal.

## Comandos de Configuración

### `axiom config init`
Inicializa un archivo `.axiom.yaml` local con los valores por defecto.
- **Uso**: `axiom config init`
