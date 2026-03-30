# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado.
- **Banderas (Flags)**:
  - `--raw`: Omite todo el procesamiento y la síntesis de Axiom. Muestra el flujo exacto del proceso secundario.
  - `--markdown`: Habilita la transformación automática de tablas de terminal al formato Markdown.
  - `--yes`: Responde automáticamente "sí" a todas las confirmaciones.

### `axiom install`
Instala la integración con el shell de Axiom y el contexto de IA.
- **Uso**: `axiom install [OPCIONES]`
- **Banderas (Flags)**:
  - `--path <RUTA>`: Ruta del proyecto para sincronizar el contexto de IA (por defecto: directorio actual).
  - `--context-only`: Solo instala el contexto de IA, omite los alias de shell.

### `axiom uninstall`
Elimina todos los rastros de Axiom del sistema.
- **Uso**: `axiom uninstall [OPCIONES]`
- **Banderas (Flags)**:
  - `--path <RUTA>`: Ruta del proyecto para eliminar el contexto de IA (por defecto: directorio actual).

### `axiom doctor`
Ejecuta la verificación de estado del sistema y los diagnósticos.
- **Uso**: `axiom doctor [OPCIONES]`
- **Banderas (Flags)**:
  - `--path <RUTA>`: Ruta del proyecto para revisar el contexto de IA (por defecto: directorio actual).
  - `--fix`: Intenta corregir automáticamente los problemas detectados.

### `axiom self-update`
Actualiza Axiom a la última versión desde GitHub.
- **Uso**: `axiom self-update`

### `axiom last`
Muestra el resultado sin procesar del último comando ejecutado.
- **Uso**: `axiom last [OPCIONES]`
- **Banderas (Flags)**:
  - `--tail <N>`: Número de líneas para mostrar desde el final.
  - `--grep <PALABRA_CLAVE>`: Filtra las líneas por una palabra clave.

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas (Flags)**:
  - `--history`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

### `axiom discovery`
Enumera o gestiona las plantillas estructurales aprendidas actualmente.
- **Subcomandos**:
  - `list`: Enumera todas las plantillas aprendidas (por defecto).
  - `clear`: Borra todos los patrones aprendidos.
  - `forget <patrón>`: Olvida un patrón de plantilla específico.

### `axiom check-ai`
Comprueba si el proceso actual fue llamado por un agente de IA.
- **Uso**: `axiom check-ai`

### `axiom intent`
Gestiona los niveles de filtrado de inteligencia y relevancia.
- **Subcomandos**:
  - `enable <modo>`: Habilita el filtrado basado en la intención. Modos: `fuzzy` (predeterminado), `neural`.
  - `disable`: Establece la inteligencia en el Nivel 1 (OFF). Solo se procesan la estructura y la privacidad.
  - `status`: Muestra el modo de inteligencia actual y la intención descubierta.

## Comandos de Configuración

### `axiom config`
Gestión de la configuración.
- **Uso**: `axiom config [COMANDO]`
- **Comportamiento**: Sin subcomandos, abre un menú interactivo.
- **Subcomandos**:
  - `init`: Inicializa un `.axiom.yaml` local con valores por defecto.
  - `show`: Muestra la configuración actual.
  - `set <clave> <valor>`: Establece un valor de configuración (ej. `config set intelligence neural`).
