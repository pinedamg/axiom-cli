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
