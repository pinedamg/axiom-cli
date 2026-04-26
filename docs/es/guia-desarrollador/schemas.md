# Creación de Schemas

Axiom depende de esquemas (schemas) YAML para entender la salida estructural de varias herramientas CLI. Estos esquemas definen cómo filtrar, colapsar y resumir logs repetitivos o ruidosos.

Los esquemas se encuentran en el directorio `config/schemas/`.

## Anatomy of a Schema (Anatomía de un Esquema)

Un archivo de esquema típico (ej. `npm.yaml`) se ve así:

```yaml
name: npm
description: "Node Package Manager"
binary_names:
  - npm
  - npx

rules:
  - id: npm_download_progress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    summary: "[AXIOM] Se colapsaron {count} logs de descarga/avisos de paquetes."
    
  - id: npm_success_install
    pattern: "^added \\d+ packages"
    action: pass # Deja pasar esto pero quizás dale formato
```

### Campos

- `name`: Un nombre legible para la herramienta.
- `binary_names`: Una lista de ejecutables CLI que activan este esquema. Si escribes `axiom npm install`, Axiom busca `npm` en esta lista.
- `rules`: Una lista de reglas de coincidencia aplicadas secuencialmente a cada línea de salida.

### Acciones de las Reglas

- `keep`: Permite que la línea se imprima normalmente. Se usa para añadir líneas importantes a una "lista blanca".
- `collapse`: Oculta la línea que coincide. Si varias líneas consecutivas coinciden, se reemplazan por una única línea de `summary`.
- `redact`: Enmascara PII o secretos en la línea.
- `hidden`: Elimina completamente la línea del flujo sin ningún resumen.
- `synthesize`: Agrupa las líneas coincidentes en un resumen sintetizado al final.

## Cómo Contribuir con un Schema

1. Crea un nuevo archivo `.yaml` en `config/schemas/`.
2. Identifica los patrones de "ruido" comunes para la herramienta usando regex.
3. Define reglas de `collapse` o `drop` para el ruido.
4. Prueba localmente usando `cargo run -- <tu-comando>`.
5. ¡Envía un Pull Request!
