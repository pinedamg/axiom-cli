# Conceptos Clave

Axiom se basa en dos conceptos principales: **Compresión Semántica** y **Privacidad Local**.

## Compresión Semántica

Los agentes de IA actuales están "hambrientos de contexto". Cuando ejecutas un comando, el 90% de la salida es ruido repetitivo que:
1.  **Drena tu billetera:** Pagas por miles de tokens innecesarios.
2.  **Pierde el contexto:** Los errores críticos quedan enterrados bajo miles de líneas de logs de éxito.

Axiom soluciona esto actuando como un **Firewall Semántico**.

- **Agregación Inteligente**: Axiom no solo corta los logs. Entiende la estructura. Comprime más de 100 líneas de éxito en un solo resumen denso.
  *Ejemplo*: En lugar de 100 líneas diciendo "Downloading package X", obtienes `[AXIOM] 124 packages downloaded successfully. IDs: [X...Y]`.
- **Anulación de Intento (Intent Overriding)**: Axiom detecta si un comando está fallando. Si estás depurando un error específico, Axiom muestra forzosamente los logs relevantes mientras suprime el resto, asegurando que el agente de IA vea solo la señal, no el ruido.

## Escudo de Privacidad Local

Axiom asegura que los datos sensibles nunca salgan de tu máquina.

- **Escaneo de Entropía**: Axiom detecta y redacta automáticamente cadenas de alta entropía (como claves API, secretos de la nube y tokens de autenticación) utilizando métricas de Entropía de Shannon.
- **Redacción de PII**: Un motor integrado enmascara correos electrónicos, IPs y patrones sensibles antes de que lleguen a la ventana de contexto del agente de IA.

Todo esto ocurre localmente en tu máquina con una latencia de menos de 10ms. No se envían logs en bruto a los servidores de Axiom.
