# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

Cuando ejecutas `axiom` seguido de cualquier comando que no esté en la lista de abajo, actúa como un **Firewall Semántico**, interceptando la salida del comando, aplicando filtros de privacidad y compresión semántica, y devolviendo un flujo de salida optimizado de alta señal.

## Banderas del Proxy

Cuando se ejecuta en modo proxy (ej., `axiom npm install`), puedes usar las siguientes banderas (flags) globales:

- `--raw`: Omite todo el procesamiento y síntesis de Axiom. Muestra el flujo exacto del proceso hijo.
- `--markdown`: Habilita la transformación automática de tablas de la terminal a formato Markdown.
- `--yes`: Responde automáticamente "sí" a todas las preguntas.
- `--dev`: Habilita el Modo de Laboratorio para Desarrolladores (Rastreo de Decisiones).

## Comandos Principales

### `axiom enable`
Habilita Axiom a nivel global.

### `axiom disable`
Deshabilita Axiom a nivel global (modo passthrough).

### `axiom bypass <acción>`
Omite el filtrado de Axiom.
- **Acciones**:
  - `count <CANTIDAD>`: Omite los siguientes N comandos (ej. `bypass count 3`).
  - `always <COMANDO>`: Añade un comando a la lista negra permanente.
  - `never <COMANDO>`: Elimina un comando de la lista negra.
  - `run <ARGS>...`: Ejecuta un único comando sin filtrado.

### `axiom install`
Instala la integración de shell de Axiom y el contexto de IA.
- **Banderas**:
  - `-p, --path <PATH>`: Ruta del proyecto para sincronizar el contexto de IA (por defecto: directorio actual).
  - `--context-only`: Instala solo los archivos de contexto de IA (ej., `AGENTS.md`, `.cursorrules`), omitiendo los alias de la shell.
  - `--funnel-id <ID>`: ID de sesión anónimo desde el script de instalación.

### `axiom uninstall`
Elimina todos los rastros de Axiom del sistema.
- **Banderas**:
  - `-p, --path <PATH>`: Ruta del proyecto para eliminar el contexto de IA (por defecto: directorio actual).

### `axiom doctor`
Ejecuta un chequeo de salud y diagnósticos del sistema.
- **Banderas**:
  - `-p, --path <PATH>`: Ruta del proyecto para verificar el contexto de IA (por defecto: directorio actual).
  - `-f, --fix`: Intenta solucionar automáticamente los problemas detectados.

### `axiom self-update`
Actualiza Axiom a la última versión desde GitHub.

### `axiom last`
Muestra la salida cruda del último comando ejecutado.
- **Banderas**:
  - `-t, --tail <LÍNEAS>`: Número de líneas a mostrar desde el final.
  - `-g, --grep <PALABRA_CLAVE>`: Filtra las líneas por una palabra clave.

### `axiom dev <args>`
Ejecuta un comando en Modo de Laboratorio para Desarrolladores (Rastreo de Decisiones).

### `axiom gain`
Muestra análisis sobre el ahorro de tokens.
- **Banderas**:
  - `-s, --history`: Muestra el historial detallado de ahorros.

### `axiom check-ai`
Verifica si el proceso actual fue llamado por un agente de IA. Sale con código 0 si es detectado, 1 en caso contrario.

## Comandos de Configuración y Descubrimiento

### `axiom intent <acción>`
Gestiona el Descubrimiento de Intenciones (Intent Discovery) y los Niveles de Inteligencia.
- **Acciones**:
  - `enable <modo>`: Habilita la inteligencia de intenciones. Modos: `fuzzy` (palabras clave) o `neural` (embeddings de IA). Por defecto es `fuzzy`.
  - `disable`: Deshabilita la inteligencia de intenciones (mantiene el formato pero muestra todos los archivos).
  - `status`: Muestra el estado actual del descubrimiento de intenciones y los archivos relevantes.

### `axiom discovery <acción>`
Enumera o gestiona las plantillas estructurales aprendidas actualmente.
- **Acciones**:
  - `list` *(por defecto)*: Lista todas las plantillas aprendidas.
  - `clear`: Limpia todos los patrones aprendidos.
  - `forget <patrón>`: Olvida el patrón de una plantilla específica.

### `axiom config <acción>`
Gestión de la configuración. Si no se proporciona ninguna acción, se inicia un menú de configuración interactivo.
- **Acciones**:
  - `init`: Inicializa un archivo local `.axiom.yaml` con valores predeterminados.
  - `show`: Muestra la configuración actual.
  - `set <clave> <valor>`: Establece un valor de configuración (ej., `axiom config set intelligence neural`).
