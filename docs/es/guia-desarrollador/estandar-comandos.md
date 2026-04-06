# Protocolo de Mejora de Comandos de Axiom (ACEP)

Este documento define el estándar industrial para mejorar los comandos CLI base dentro del ecosistema de Axiom. Cada comando añadido o mejorado debe adherirse a la "Arquitectura de Tres Capas" para asegurar la consistencia, el rendimiento y la eficiencia de tokens.

## 1. La Arquitectura de Tres Capas

Al mejorar un comando, implementa estas tres versiones de su salida:

*   **V1: Síntesis Estructural (Rendimiento):**
    *   Identifica patrones repetitivos en la salida bruta.
    *   Usa `DiscoveryEngine` para agrupar elementos similares (ej., "15 archivos por extensión", "5 procesos worker").
    *   *Objetivo:* Reducir el conteo de líneas en al menos un 70%.

*   **V2: Insight Semántico (Inteligencia):**
    *   Inyecta contexto de alto nivel usando `AxiomEngine::generate_semantic_insight()`.
    *   Detecta marcadores de proyecto (ej., `Cargo.toml`, `package.json`, `.git`).
    *   *Objetivo:* Responder a "¿Qué es esto?" y "¿Qué debo hacer ahora?" de forma proactiva.

*   **V3: Redacción de Privacidad (Seguridad):**
    *   Identifica datos sensibles (archivos ocultos, variables de entorno, claves, PII).
    *   Establece reglas predeterminadas en el schema para `redact` o hacer `hidden` esta información.
    *   *Objetivo:* Salida segura para el consumo de IA y entornos compartidos.

---

## 2. Flujo de Trabajo Paso a Paso

### Paso 1: Análisis de Salida (La fase "Cruda")
Ejecuta el comando en varios entornos y captura su salida típica.
```bash
# Ejemplo: Análisis de 'git status'
git status --porcelain
```
Identifica qué es **Constante** (Estructura) y qué es **Variable** (Ruido).

### Paso 2: Definición de Schema
Crea o actualiza el schema YAML en `config/schemas/<comando>.yaml`.
*   Asigna `priority` a las reglas (mayor para patrones específicos).
*   Usa `synthesize` para grupos de datos.
*   Usa `collapse` para líneas de baja señal conocidas (cabeceras, totales).
*   Usa `redact` para patrones sensibles.

### Paso 3: Integración con el Motor (Engine)
Si el comando requiere un análisis personalizado (como dividir columnas en `ls`):
1.  Actualiza `src/engine/discovery.rs` con lógica específica `parse_<comando>_line`.
2.  Asegúrate de que `synthesize_line` maneje el nuevo formato.
3.  **Crítico:** Abstrae las variables usando `<NUM>`, `<MONTH>`, `<TIME>`, y `<VAR>` para evitar una explosión de plantillas en la BD.

### Paso 4: Inyección Semántica
Actualiza `AxiomEngine::generate_semantic_insight()` en `src/engine/mod.rs`:
1.  Identifica marcadores específicos para el comando.
2.  Devuelve una cadena legible por humanos y eficiente en tokens que comience con "Detectado...".

### Paso 5: SNR y Optimización de Tokens
*   **Cabecera:** Asegúrate de que la salida del comando utilice la cabecera compacta `[AXIOM]` en el gateway.
*   **Prefijos:** Nunca repitas `[AXIOM]` por línea. Usa la viñeta `•` para los elementos.
*   **Vaciado (Flush):** Solo realiza un vaciado de los resúmenes al final de la ejecución del comando para evitar la fragmentación de la salida.

### Paso 6: Validación (El "Camino Dorado")
1.  **Prueba en Vivo:** Ejecuta `axiom <comando>` y verifica la salida.
2.  **Fixture:** Guarda la salida en `tests/fixtures/<comando>_axiom.txt`.
3.  **Regresión:** Crea/Actualiza `tests/<comando>_versions_test.rs` para verificar que los cambios futuros no rompan la síntesis o los insights.

---

## 3. Mejores Prácticas
*   **Falla Segura (Fail Safe):** Si el motor no puede analizar una línea, por defecto haz `keep` (Crudo) o un `Discovery` genérico (Detección de ruido).
*   **Conciencia de Tokens:** Usa insights cortos y descriptivos. Evita el relleno conversacional (chamuyo).
*   **Privacidad Primero:** Ante la duda, redacta. El usuario siempre puede solicitar la versión "sin redactar" a través de la intención si es necesario.
