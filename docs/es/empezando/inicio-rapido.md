# Inicio Rápido

Axiom actúa como un comando proxy. Para usarlo, simplemente antepón `axiom` a tus comandos habituales de la terminal.

## Uso Básico

Cada vez que ejecutes un comando que genere mucho ruido, antepón el prefijo:

```bash
axiom npm install
```

```bash
axiom git diff
```

```bash
axiom docker-compose up
```

## Cómo Funciona

1. **Intercepción**: Axiom captura el `stdout` y `stderr` del comando.
2. **Análisis**: Identifica la herramienta que se está ejecutando (ej. `npm`) y aplica las reglas de procesamiento correspondientes.
3. **Compresión**: Se elimina el ruido repetitivo y los logs estructuralmente similares se resumen en una única línea densa.
4. **Protección**: Antes de que nada se imprima en pantalla, el Escudo de Privacidad de Axiom escanea cadenas de alta entropía (como claves API) y las redacta.

## Consultando tus Ahorros

Axiom realiza un seguimiento local de cuántos tokens te ha ahorrado.

```bash
axiom gain
```

Para ver un historial detallado de tus ahorros de tokens por comando:

```bash
axiom gain --history
```

## Ajustando la Inteligencia

Dependiendo de tu tarea, puedes cambiar la agresividad con la que Axiom filtra la salida:

*   **Deep Debugging (Depuración Profunda)**: `axiom intent enable neural` (Usa embeddings de IA locales).
*   **Modo Estándar**: `axiom intent enable fuzzy` (Por defecto, basado en palabras clave).
*   **Solo Resúmenes**: `axiom intent disable` (Sin filtrado de relevancia de IA).
*   **Bypass Total**: `axiom --raw <comando>` (Salida cruda).
