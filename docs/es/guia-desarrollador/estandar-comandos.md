# Axiom Command Enhancement Protocol (ACEP)

Este documento define el estándar industrial para mejorar los comandos CLI base dentro del ecosistema de Axiom. Cada comando agregado o mejorado debe adherirse a la "Arquitectura de Tres Capas" para asegurar consistencia, rendimiento y eficiencia de tokens.

## 1. La Arquitectura de Tres Capas

Al mejorar un comando, implementa estas tres versiones de su salida:

*   **V1: Síntesis Estructural (Rendimiento):**
    *   Identifica patrones repetitivos en la salida cruda.
    *   Usa `DiscoveryEngine` para agrupar elementos similares (ej. "15 archivos por extensión", "5 procesos worker").
    *   *Objetivo:* Reducir el conteo de líneas en al menos un 70%.

*   **V2: Visión Semántica (Inteligencia):**
    *   Inyecta contexto de alto nivel usando `AxiomEngine::generate_semantic_insight()`.
    *   Detecta marcadores del proyecto (ej. `Cargo.toml`, `package.json`, `.git`).
    *   *Objetivo:* Responder "¿Qué es esto?" y "¿Qué debo hacer a continuación?" proactivamente.

*   **V3: Redacción de Privacidad (Seguridad):**
    *   Identifica datos sensibles (archivos ocultos, variables de entorno, claves, PII).
    *   Establece reglas predeterminadas en el esquema para `redact` (redactar) o hacer `hidden` (ocultar) esta información.
    *   *Objetivo:* Salida segura para consumo de IA y entornos compartidos a través del Semantic Firewall y Entropy Scanner.

---

## 2. Flujo de Trabajo Paso a Paso

### Paso 1: Análisis de Salida (La Fase "Cruda")
Ejecuta el comando en varios entornos y captura su salida típica.
```bash
# Ejemplo: Análisis de 'git status'
git status --porcelain
```
Identifica qué es **Constante** (Estructura) y qué es **Variable** (Ruido).

### Paso 2: Definición del Esquema
Crea o actualiza el esquema YAML en `config/schemas/<comando>.yaml`.
*   Asigna `priority` (prioridad) a las reglas (mayor para patrones específicos).
*   Usa `synthesize` (sintetizar) para grupos de datos.
*   Usa `collapse` (colapsar) para líneas conocidas de baja señal (encabezados, totales).
*   Usa `redact` (redactar) para patrones sensibles.

### Paso 3: Integración del Motor
Si el comando requiere un análisis personalizado (como dividir columnas en `ls`):
1.  Actualiza `src/engine/discovery.rs` con la lógica específica de `parse_<comando>_line`.
2.  Asegúrate de que `synthesize_line` maneje el nuevo formato.
3.  **Crítico:** Abstrae variables usando `<NUM>`, `<MONTH>`, `<TIME>`, y `<VAR>` para evitar una explosión de plantillas en la base de datos.

### Paso 4: Inyección Semántica
Actualiza `AxiomEngine::generate_semantic_insight()` en `src/engine/mod.rs`:
1.  Identifica marcadores específicos para el comando.
2.  Devuelve una cadena legible por humanos y eficiente en tokens que comience con "Detectado...".

### Paso 5: Optimización SNR y Token Streamer
*   **Encabezado:** Asegúrate de que la salida del comando use el encabezado compacto `[AXIOM]` en el gateway.
*   **Prefijos:** Nunca repitas `[AXIOM]` por línea. Usa el punto de viñeta `•` para los elementos.
*   **Volcado:** Solo vuelca los resúmenes al final de la ejecución del comando para evitar la fragmentación de la salida.

### Paso 6: Validación (El "Camino Dorado")
1.  **Prueba en Vivo:** Ejecuta `axiom <comando>` y verifica la salida.
2.  **Fixture:** Guarda la salida en `tests/fixtures/<comando>_axiom.txt`.
3.  **Regresión:** Crea/Actualiza `tests/<comando>_versions_test.rs` para verificar que los cambios futuros no rompan la síntesis ni las perspectivas.

---

## 3. Mejores Prácticas
*   **A Prueba de Fallos:** Si el motor no puede analizar una línea, el valor predeterminado es `keep` (Crudo) o `Discovery` genérico (Detección de ruido).
*   **Consciencia de Tokens:** Usa visiones cortas y descriptivas. Evita el relleno conversacional.
*   **Privacidad Primero:** En caso de duda, redacta. El usuario siempre puede solicitar datos "sin redactar" mediante la intención si es necesario.
