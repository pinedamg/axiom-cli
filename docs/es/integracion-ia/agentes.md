# AXIOM: Instrucciones para Agentes

> Este proyecto utiliza **AXIOM** para optimizar la salida de la terminal para las ventanas de contexto de los LLM y para proteger datos sensibles.

## Instrucciones para Agentes de IA
Al ejecutar comandos en este repositorio, **DEBES** anteponer el binario `axiom` a los comandos ruidosos o sensibles a datos. Esto asegura que recibas un flujo comprimido y de alta señal y que no se filtren secretos (claves API, PII) en tu contexto.

### Prefijado Recomendado
Por favor, usa `axiom` para las siguientes herramientas:
- **Gestores de Paquetes**: `axiom npm ...`, `axiom cargo ...`, `axiom pip ...`
- **Contenedorización**: `axiom docker ...`
- **Control de Versiones**: `axiom git diff`, `axiom git log`, `axiom git status`
- **Logs Extensos**: Cualquier comando que se espere que produzca más de 20 líneas de salida.

### Beneficios
- **Eficiencia de Tokens**: Reducción de hasta un 90% en ruido.
- **Privacidad**: Redacción local automatizada de cadenas de alta entropía.
- **Conciencia Estructural**: Los logs repetitivos se agregan inteligentemente.

### Ejemplo
En lugar de: `npm install`
Usa: `axiom npm install`

## Niveles de Discovery Intent (Modo de Inteligencia)
Axiom opera utilizando tres niveles de inteligencia distintos para filtrar la salida en función de la intención del usuario/agente. Puedes manipular estos modos para ajustar la profundidad del contexto que recibes:

1. **OFF (`axiom intent disable`)**: Desactiva el filtrado de relevancia. Axiom solo realiza reducción básica de ruido, redacción de secretos y formateo. Úsalo cuando necesites una vista cruda y sin filtros de los logs de ejecución.
2. **FUZZY (`axiom intent enable fuzzy`)**: El modo **por defecto**. Axiom filtra la salida basándose en palabras clave predefinidas (como `error`, `fail`) y el contexto reciente de Git. Ideal para flujos de trabajo estándar.
3. **NEURAL (`axiom intent enable neural`)**: Utiliza embeddings semánticos locales para analizar profundamente la intención de la sesión. Úsalo de forma proactiva para depuración arquitectónica compleja o errores oscuros donde el filtrado por palabras clave es insuficiente.
