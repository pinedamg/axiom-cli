# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica filtros de privacidad, compresión semántica y entrega el flujo optimizado.

### `axiom install`
Instala la integración de shell de Axiom.
- **Uso**: `axiom install`
- **Comportamiento**: Proporciona instrucciones y alias para añadir a la configuración de tu shell para enrutar automáticamente los comandos ruidosos a través de Axiom.

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Uso**: `axiom gain`
- **Banderas (Flags)**:
  - `-s, --history`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

### `axiom discovery`
Lista las plantillas estructurales aprendidas actualmente.
- **Uso**: `axiom discovery`
- **Comportamiento**: Muestra los patrones estructurales que Axiom ha aprendido de tus comandos y sus frecuencias.

### `axiom check-ai`
Comprueba si el proceso actual fue llamado por un agente de IA.
- **Uso**: `axiom check-ai`
- **Comportamiento**: Detecta si la shell que ejecuta el comando pertenece a un agente de IA (como Cursor o Claude Code) y sale con 0 si es cierto, 1 si es falso.

### `axiom intent <acción>`
Gestiona el Descubrimiento de Intenciones (Intent Discovery) y los Niveles de Inteligencia.
- **Acciones**:
  - `enable [modo]`: Habilita la inteligencia de intenciones. `modo` puede ser `fuzzy` (palabras clave) o `neural` (embeddings de IA). Por defecto es `fuzzy`.
  - `disable`: Deshabilita la inteligencia de intenciones (mantiene el formato pero muestra todos los archivos).
  - `status`: Muestra el estado actual del descubrimiento de intenciones, incluyendo ID de Sesión, Modo de Inteligencia, Proceso Padre y Última Intención.

## Comandos de Configuración

### `axiom config init`
Inicializa un archivo de configuración local `.axiom.yaml` con valores predeterminados.
- **Uso**: `axiom config init`
- **Comportamiento**: Crea un nuevo archivo `.axiom.yaml` en el directorio actual si no existe.
