# Referencia de la CLI

Che, la interfaz de comandos (CLI) de Axiom te da las herramientas precisas para gestionar tu instalación, chusmear los análisis y ajustar configuraciones a tu medida. ¡Vamos a ver qué hace cada cosa!

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado actuando como un verdadero **Firewall Semántico**.
- **Banderas Globales**:
  - `-m, --markdown`: Habilita la transformación automática de tablas de terminal al formato Markdown.
  - `-r, --raw`: Muestra la salida en bruto, saltándose todo el procesamiento de Axiom.
  - `-y, --yes`: Responde automáticamente sí a todas las preguntas.

### `axiom install`
Instala la integración de shell de Axiom y el contexto de IA.
- **Uso**: `axiom install`
- **Banderas**:
  - `-p, --path <ruta>`: Ruta del proyecto para sincronizar el contexto de IA (por defecto: directorio actual).
  - `--context-only`: Solo instala el contexto de IA, omite los alias de shell.

### `axiom uninstall`
Elimina todo rastro de Axiom del sistema.
- **Uso**: `axiom uninstall`
- **Banderas**:
  - `-p, --path <ruta>`: Ruta del proyecto de donde eliminar el contexto (por defecto: directorio actual).

### `axiom doctor`
Realiza un chequeo de salud y diagnóstico del sistema. ¡Para que todo ande joya!
- **Uso**: `axiom doctor`
- **Banderas**:
  - `-p, --path <ruta>`: Ruta del proyecto para revisar (por defecto: directorio actual).
  - `-f, --fix`: Intenta arreglar automáticamente los problemas detectados.

### `axiom self-update`
Actualiza Axiom a la última versión disponible en GitHub.
- **Uso**: `axiom self-update`

### `axiom last`
Muestra la salida cruda del último comando ejecutado.
- **Uso**: `axiom last`
- **Banderas**:
  - `-t, --tail <usize>`: Número de líneas a mostrar desde el final.
  - `-g, --grep <cadena>`: Filtra las líneas por una palabra clave.

### `axiom gain`
Muestra el análisis de tu ahorro de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas**:
  - `-s, --history`: Muestra una lista detallada de los ahorros recientes.

### `axiom discovery`
Lista o gestiona las plantillas estructurales que Axiom ha aprendido.
- **Subcomandos**:
  - `list` (por defecto): Lista todas las plantillas aprendidas.
  - `clear`: Limpia todos los patrones aprendidos.
  - `forget <patron>`: Olvida un patrón de plantilla específico.

### `axiom check-ai`
Comprueba si el proceso actual fue llamado por un agente de IA.
- **Uso**: `axiom check-ai`

### `axiom intent`
Gestiona el Descubrimiento de Intenciones y los Niveles de Inteligencia.
- **Subcomandos**:
  - `enable <modo>`: Activa la inteligencia de intenciones (`fuzzy` o `neural`, por defecto: `fuzzy`).
  - `disable`: Desactiva la inteligencia de intenciones.
  - `status`: Muestra el estado actual del descubrimiento de intenciones.

## Comandos de Configuración

### `axiom config`
Gestión de configuración.
- **Uso**: `axiom config` (Abre un menú interactivo re piola).
- **Subcomandos**:
  - `init`: Inicializa un archivo local `.axiom.yaml` con valores por defecto.
  - `show`: Muestra la configuración actual.
  - `set <clave> <valor>`: Establece un valor de configuración (ej. `axiom config set intelligence neural`). Claves soportadas: `intelligence`, `markdown`.
