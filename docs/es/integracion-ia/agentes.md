# Instrucciones para Agentes Axiom

> Este proyecto utiliza **AXIOM** para optimizar la salida de la terminal para las ventanas de contexto de los LLM y para proteger datos sensibles.

## Instrucción para Agentes de IA
Al ejecutar comandos en este repositorio, **DEBES** anteponer el binario `axiom` a los comandos ruidosos o sensibles a los datos. Esto asegura que recibas un flujo comprimido de alta señal y que no se filtren secretos (claves API, PII) en tu contexto.

### Prefijado Recomendado
Por favor, usa `axiom` para las siguientes herramientas:
- **Gestores de Paquetes**: `axiom npm ...`, `axiom cargo ...`, `axiom pip ...`
- **Contenedores**: `axiom docker ...`
- **Control de Versiones**: `axiom git diff`, `axiom git log`, `axiom git status`
- **Logs Largos**: Cualquier comando que se espere que genere más de 20 líneas de salida.

### Beneficios
- **Eficiencia de Tokens**: Reducción de ruido de hasta un 90%.
- **Privacidad**: Redacción local automatizada de cadenas de alta entropía.
- **Conciencia Estructural**: Los logs repetitivos se agregan inteligentemente.

### Ejemplo
En lugar de: `npm install`
Usa: `axiom npm install`
