# Creación de Schemas

Axiom depende de esquemas (schemas) YAML para entender la salida estructural de varias herramientas CLI. Estos esquemas definen cómo filtrar, colapsar y resumir logs repetitivos o ruidosos.

Los esquemas se encuentran en el directorio `config/schemas/`.

## Anatomy of a Schema (Anatomía de un Esquema)

Un archivo de esquema típico (ej. `npm.yaml`) se ve así:

```yaml
name: npm
description: "Node Package Manager"
command_pattern:
  "^npm"

rules:
  - name: npm_download_progress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    summary: "[AXIOM] Se colapsaron {count} logs de descarga/avisos de paquetes."
    
  - name: npm_success_install
    pattern: "^added \\d+ packages"
    action: keep # Mantiene esta línea
    priority: 10
```

### Campos

- `name`: Un nombre legible para la herramienta.
- `command_pattern`: Un patrón regex que coincide con el nombre del ejecutable CLI. Si escribes `axiom npm install`, Axiom compara `npm` contra este patrón.
- `rules`: Una lista de reglas de coincidencia aplicadas secuencialmente a cada línea de salida.

### Acciones de las Reglas

- `collapse`: Oculta la línea que coincide. Si varias líneas consecutivas coinciden, se reemplazan por una única línea de `summary`. La variable `{count}` puede usarse en el resumen.
- `keep`: Permite que la línea se imprima normalmente. Se usa para añadir líneas importantes a una "lista blanca".
- `hidden`: Elimina completamente la línea del flujo sin ningún resumen.
- `redact`: Redacta la línea que coincide.
- `synthesize`: Se usa para agrupamiento inteligente.

## Cómo Contribuir con un Schema

1. Crea un nuevo archivo `.yaml` en `config/schemas/`.
2. Identifica los patrones de "ruido" comunes para la herramienta usando regex.
3. Define reglas de `collapse` o `hidden` para el ruido.
4. Prueba localmente usando `cargo run -- <tu-comando>`.
5. ¡Envía un Pull Request!
