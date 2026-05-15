## ⚡ Bolt: Memoria en Discovery Engine

Encontramos un patrón de consumo de memoria específico en la arquitectura de Axiom, en particular dentro de `DiscoveryEngine::process_and_check_noise`.

El `variable_buffer` estaba declarado como `BTreeMap<String, Vec<Vec<String>>>`. Cada línea que coincidía con un patrón extraía y almacenaba las variables usando `extract_parts`. Para flujos de datos grandes, almacenar las variables extraídas para cada línea resultaba en un consumo sustancial de memoria (`Vec` y `String` extra).

**Optimización:**
Se ha cambiado `variable_buffer` a `BTreeMap<String, usize>`, que ahora solo guarda la cantidad de ocurrencias de un patrón en lugar de una matriz con todas las variables extraídas.

*   `extract_parts` ahora devuelve directamente el string formateado como `String` (gracias a `.into_owned()`), simplificando su retorno y evitando llenar un `Vec` pre-alocado de strings (`Vec<String>`).
*   Esta optimización evita la reserva de `Vec::with_capacity(8)` y la memoria acumulada para los *strings* de variables en el `variable_buffer` para cada iteración en bucles largos o archivos grandes procesados.

La reducción estimada de memoria en flujos de datos pesados está relacionada con el tamaño y número de variables que se desechaban al momento del flush. Ahora sólo se actualiza un contador (`usize`).

**Métricas Estimadas:**
Antes de este cambio, procesar 10,000 líneas con 5 variables cada una resultaría en al menos 50,000 instancias de `String` extraídas y almacenadas en la memoria (más las cabeceras `Vec`), ocupando de varios cientos de KB a unos cuantos MB según el tamaño de la cadena.
Con esta optimización, el uso de memoria queda fijado a la capacidad del contador (`usize`) independientemente de la cantidad de repeticiones. Por ejemplo, en 10,000 líneas, un único patrón almacenado sólo utilizará `sizeof(usize)` bytes adicionales.
