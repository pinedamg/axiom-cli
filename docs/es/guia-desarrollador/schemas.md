# Creación de Schemas

Axiom depende de esquemas (schemas) YAML para entender la salida estructural de varias herramientas CLI. Estos esquemas definen cómo filtrar, colapsar y resumir logs repetitivos o ruidosos.

Los esquemas se encuentran en el directorio `config/schemas/`.

## Anatomy of a Schema (Anatomía de un Esquema)

Un archivo de esquema típico (ej. `npm.yaml`) se ve así:

```yaml
name: npm
description: "Node Package Manager"
command_pattern: "^(npm|npx)$"

rules:
  - name: npm_download_progress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    priority: 10
    summary: "[AXIOM] Se colapsaron {count} logs de descarga/avisos de paquetes."
    
  - name: npm_success_install
    pattern: "^added \\d+ packages"
    action: keep # Deja pasar esto pero quizás dale formato
    priority: 5
```

### Campos

- `name`: Un nombre legible para la herramienta.
- `command_pattern`: Una expresión regular que coincide con el ejecutable CLI que activa este esquema. Si escribes `axiom npm install`, Axiom busca `npm` contra este patrón.
- `rules`: Una lista de reglas de coincidencia aplicadas a cada línea de salida, ordenadas por `priority`.

### Campos de las Reglas
- `name`: El identificador de la regla.
- `pattern`: El patrón regex para coincidir con la línea de salida.
- `action`: La acción de transformación a aplicar (ver abajo).
- `priority`: Un entero que determina el orden de ejecución de las reglas. Los valores más altos se evalúan primero.

### Acciones de las Reglas

- `keep`: Permite que la línea se imprima normalmente. Se usa para añadir líneas importantes a una "lista blanca".
- `collapse`: Oculta la línea que coincide. Si varias líneas consecutivas coinciden, se reemplazan por una única línea de `summary`. La variable `{count}` puede usarse en el resumen.
- `redact`: Reemplaza información sensible en la línea con un marcador redactado.
- `hidden`: Elimina completamente la línea del flujo sin ningún resumen.
- `synthesize`: Agrupa múltiples líneas que coinciden para un resumen inteligente.

## Cómo Contribuir con un Schema

1. Crea un nuevo archivo `.yaml` en `config/schemas/`.
2. Identifica los patrones de "ruido" comunes para la herramienta usando regex.
3. Define reglas de `collapse` o `hidden` para el ruido.
4. Prueba localmente usando `cargo run -- <tu-comando>`.
5. ¡Envía un Pull Request!
