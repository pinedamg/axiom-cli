# AXIOM: Log de Desarrollo y Registro de Contexto

Acá llevamos la bitácora posta del proyecto: decisiones arquitectónicas, en qué andamos ahora, y cómo validamos que todo funcione joya para asegurar la continuidad.

## 1. Contexto Actual (Marzo 2026)
- **Fase**: Alpha/Beta (Fases 1, 2 y 3 completadas como prototipos).
- **Estado**: Sistema de intercepción, escudo de privacidad, lógica de intención y motor de auto-descubrimiento funcionales.
- **Hito Reciente**: Validación empírica del "Auto-Descubrimiento" con feedback visual de colapso.

## 2. Estrategia de Validación (Pruebas)
Para asegurarnos que Axiom sea un caño y no "rompa" información crítica para el LLM:

### Capa 1: Pruebas Unitarias (Lógica Atómica)
- Validación del motor de entropía (detección de secretos).
- Matcher de esquemas YAML.

### Capa 2: Simulación de Intención (Context Mocking)
- Uso de `IntentContext` para simular prompts de usuario (ej. "Encuentra el error en X").
- Verificación de que la salida comprimida contiene las palabras clave necesarias para resolver el prompt.

### Capa 3: Benchmarking de Tokens
- Cada prueba de integración debe reportar el ahorro de tokens (caracteres ahorrados).
- Objetivo: >60% de ahorro en logs ruidosos (npm, docker, maven).

## 3. Estrategia de Integración con IA
Axiom tiene que ser transparente para el agente de IA (Gemini, Claude, Cursor, etc.), que ni se dé cuenta de que estamos ahí metidos:

1.  **Intercepción por Proxy**:
    - El agente ejecuta `git status`.
    - El shell (vía alias o hook) ejecuta `axiom git status`.
    - Axiom limpia la salida y la devuelve al agente.
2.  **Detección de Intención**:
    - Axiom buscará archivos temporales o variables de entorno donde el agente guarda el "Contexto del Chat" para ajustar su filtro dinámicamente.

## 4. Decisiones Clave de Diseño
- **Arquitectura Lib-First**: El núcleo de Axiom vive en `lib.rs` para facilitar las pruebas.
- **Privacidad Primero**: El escaneo de entropía ocurre antes de cualquier lógica semántica.
- **Feedback Visual (Fase 3+)**: Se implementó un contador de colapso para dar visibilidad al usuario sobre el ahorro de tokens sin perder el estado del proceso.

---
*Última actualización: Metiéndole pata con el prototipo de la Fase 3 y el registro del Agregador Inteligente.*
