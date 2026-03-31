# Telemetría y Privacidad

Axiom está construido pensando en la privacidad. Recolectamos metadatos anónimos para mejorar los algoritmos de compresión y descubrir esquemas de comandos faltantes. **Ningún argumento de comando, secreto o PII sale jamás de tu máquina.**

## Niveles de Telemetría

Axiom ofrece cuatro niveles de transparencia, configurables mediante:
```bash
axiom config telemetry <level>
```

1. **`Full` (Por defecto)**: Comparte métricas de ahorro anónimas + Nombres de los binarios utilizados + Métricas internas (IDs de coincidencia de reglas).
2. **`Discovery`**: Comparte métricas de ahorro anónimas + Nombres de binarios (ej. `git`, `npm`). Esto nos ayuda a priorizar qué nuevos esquemas de herramientas debemos construir a continuación.
3. **`Basic`**: Solo envía ahorros de tokens agregados, tu SO y la versión de Axiom. No se envían nombres de comandos.
4. **`Off`**: Apagón total. No se envía ningún dato desde tu máquina.

## Transparencia Ante Todo

Ejecuta `axiom config show` en cualquier momento para ver exactamente qué datos se están compartiendo.

## Cómo Te Protegemos

- **Sanitización de Comandos**: Incluso en el modo `Full`, **SOLO** capturamos el nombre del binario (ej. `npm`), nunca los argumentos (ej. `install paquete-secreto`).
- **ID Anónimo**: Usamos un `installation_id` aleatorio para contar instancias activas sin saber quién eres.
