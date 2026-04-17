# Creación de Schemas

Axiom depende de esquemas (schemas) YAML para entender la salida estructural de varias herramientas CLI. Estos esquemas definen cómo filtrar, colapsar y resumir logs repetitivos o ruidosos.

Los esquemas se encuentran en el directorio `config/schemas/`.

## Anatomy of a Schema (Anatomía de un Esquema)

Un archivo de esquema típico (ej. `npm.yaml`) se ve así:

```yaml
name: npm
command_pattern: "^npm|^npx"

rules:
  - name: DownloadProgress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    priority: 10
    
  - name: SuccessInstall
    pattern: "^added \\d+ packages"
    action: keep
    priority: 5
```

### Campos

- `name`: Un nombre legible para la herramienta.
- `command_pattern`: Un patrón regex para coincidir con el comando que activa este esquema.
- `rules`: Una lista de reglas de coincidencia aplicadas a cada línea de salida. Gana la regla con la prioridad más alta que coincida.

### Acciones de las Reglas

- `keep`: Permite que la línea se imprima normalmente. Se usa para añadir líneas importantes a una "lista blanca".
- `collapse`: Oculta la línea que coincide. Si varias líneas consecutivas coinciden, se reemplazan por una única línea de resumen.
- `redact`: Redacta información sensible de la línea.
- `hidden`: Elimina completamente la línea del flujo sin ningún resumen.
- `synthesize`: Reemplaza las líneas con un resumen inteligente usando el agregador inteligente.

## Cómo Contribuir con un Schema

1. Crea un nuevo archivo `.yaml` en `config/schemas/`.
2. Identifica los patrones de "ruido" comunes para la herramienta usando regex.
3. Define reglas de `collapse` o `drop` para el ruido.
4. Prueba localmente usando `cargo run -- <tu-comando>`.
5. ¡Envía un Pull Request!
