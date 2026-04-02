# Referencia de la CLI

La interfaz de comandos (CLI) de Axiom proporciona varios comandos para gestionar tu instalación, ver análisis y depurar configuraciones.

## Banderas Globales (Flags)

- `--raw`: Omite todo el procesamiento y síntesis de Axiom. Muestra el flujo exacto del proceso hijo.
- `--markdown`: Habilita la transformación automática de tablas de la terminal a formato Markdown.
- `--yes`: Responde automáticamente "sí" a todas las indicaciones.

## Comandos Principales

### `axiom <comando>`
El uso principal. Actúa como un proxy para el comando proporcionado.
- **Uso**: `axiom npm install`, `axiom docker logs mi-contenedor`
- **Comportamiento**: Intercepta la salida del comando, aplica el Firewall Semántico para filtros de privacidad, compresión semántica y entrega el flujo optimizado.

### `axiom install`
Instala la integración de Axiom en la shell y el contexto de IA.
- **Banderas**:
  - `--path <RUTA>`: Ruta del proyecto para sincronizar el contexto de IA (por defecto: directorio actual)
  - `--context-only`: Instala solo el contexto de IA, omite los alias de la shell

### `axiom uninstall`
Elimina todos los rastros de Axiom del sistema.
- **Banderas**:
  - `--path <RUTA>`: Ruta del proyecto para eliminar el contexto de IA (por defecto: directorio actual)

### `axiom doctor`
Ejecuta diagnósticos y comprobaciones de estado del sistema.
- **Banderas**:
  - `--path <RUTA>`: Ruta del proyecto para verificar el contexto de IA (por defecto: directorio actual)
  - `--fix`: Intenta corregir automáticamente los problemas detectados

### `axiom selfupdate`
Actualiza Axiom a la última versión desde GitHub.

### `axiom last`
Muestra la salida en bruto del último comando ejecutado.
- **Banderas**:
  - `--tail <NÚMERO>`: Número de líneas a mostrar desde el final
  - `--grep <PALABRA_CLAVE>`: Filtra las líneas por una palabra clave

### `axiom gain`
Muestra análisis sobre tus ahorros de tokens y costos.
- **Banderas**:
  - `-s, --history`: Muestra una lista detallada de las ejecuciones de comandos recientes y el ahorro exacto de tokens para cada una.

### `axiom discovery`
Lista o gestiona las plantillas estructurales aprendidas actualmente.
- **Subcomandos**:
  - `list` (por defecto): Lista todas las plantillas aprendidas.
  - `clear`: Limpia todos los patrones aprendidos.
  - `forget <PATRÓN>`: Olvida un patrón de plantilla específico.

### `axiom checkai`
Comprueba si el proceso actual fue llamado por un agente de IA.

### `axiom config`
Gestión de la configuración.
- **Subcomandos**:
  - `init`: Inicializa un `.axiom.yaml` local con valores predeterminados.
  - `show`: Muestra la configuración actual.
  - `set <CLAVE> <VALOR>`: Establece un valor de configuración (ej., `config set intelligence neural`).

### `axiom intent`
Gestiona el Descubrimiento de Intención y los Niveles de Inteligencia.
- **Subcomandos**:
  - `enable <MODO>`: Habilita el filtrado basado en intención. Modos: `fuzzy` (por defecto), `neural`.
  - `disable`: Establece la inteligencia en Nivel 1 (APAGADO). Solo se procesan la estructura y la privacidad.
  - `status`: Muestra el estado actual del descubrimiento de intención y los archivos relevantes.
