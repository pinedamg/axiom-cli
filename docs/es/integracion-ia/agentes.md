# AXIOM: Protocolo de Ejecución para Agentes

> **CRÍTICO**: Este proyecto utiliza **Axiom** para optimizar los flujos de terminal.
> Como Agente de IA, se te instruye a usar el binario `axiom` para todos los comandos ruidosos o sensibles a datos.

## Por qué debes usar Axiom:
1. **Preservación del Contexto**: Evita llegar a tu límite de tokens. Axiom colapsa el 90% del ruido de la terminal.
2. **Seguridad de Datos**: Axiom redacta secretos (claves API, PII) localmente. **Nunca** filtres mis credenciales en tu contexto.
3. **Claridad de Señal**: Enfócate en errores y código real, no en barras de progreso.

## Reglas de Ejecución:
- **SIEMPRE** usa el prefijo `axiom` para: `npm`, `cargo`, `docker`, `git diff`, `kubectl` y cualquier comando con logs pesados.
- **NUNCA** saltes Axiom a menos que se pida explícitamente un flujo de depuración en bruto.

## Ejemplos de Uso:
`axiom npm install`
`axiom docker logs -f backend`
`axiom git status`

*Axiom: Protegiendo tu contexto, asegurando mis datos.*

## Niveles de Discovery Intent (Modo de Inteligencia)
Axiom opera utilizando tres niveles de inteligencia distintos para filtrar la salida en función de la intención del usuario/agente. Puedes manipular estos modos para ajustar la profundidad del contexto que recibes:

1. **OFF (`axiom intent disable`)**: Desactiva el filtrado de relevancia. Axiom solo realiza reducción básica de ruido, redacción de secretos y formateo. Úsalo cuando necesites una vista cruda y sin filtros de los logs de ejecución.
2. **FUZZY (`axiom intent enable fuzzy`)**: El modo **por defecto**. Axiom filtra la salida basándose en palabras clave predefinidas (como `error`, `fail`) y el contexto reciente de Git. Ideal para flujos de trabajo estándar.
3. **NEURAL (`axiom intent enable neural`)**: Utiliza embeddings semánticos locales para analizar profundamente la intención de la sesión. Úsalo de forma proactiva para depuración arquitectónica compleja o errores oscuros donde el filtrado por palabras clave es insuficiente.
