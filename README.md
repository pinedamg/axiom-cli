# AXIOM: The Semantic Token Streamer 🦀

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-Alpha-yellow.svg)](#)
[![Speed](https://img.shields.io/badge/overhead-%3C10ms-green.svg)](#)

> **"Deja de desperdiciar el 90% de tu ventana de contexto en ruido de la terminal."**

**Axiom** es un proxy inteligente escrito en Rust que actúa como un **Firewall Semántico** entre tu terminal y tus agentes de IA (Cursor, Claude Code, Gemini CLI). Transforma salidas ruidosas y masivas en un flujo condensado de alta señal, optimizado para LLMs.

---

## ⚡ El Efecto Axiom (Antes vs Después)

**Sin Axiom (Ruido Puro - 2000 tokens):**
```text
npm WARN deprecated inflight@1.0.6: ...
npm notice scanning for vulnerabilities...
fetch http://registry.npmjs.org/axios/-/axios-1.6.2.tgz
downloading [####################] 100%
added 124 packages in 5s...
(y 200 líneas más de progreso...)
```

**Con Axiom (Señal Pura - 50 tokens):**
```text
[AXIOM] Collapsed 124 package fetching/warning logs.
✔ Added 124 packages in 5s. 
[AXIOM] Privacy Shield: 0 secrets detected. 85% context saved.
```

---

## 🚀 ¿Por qué instalarlo hoy?

1.  **Ahorra Dinero**: Reduce el consumo de tokens entre un **60% y 90%** en tareas comunes.
2.  **IA más Inteligente**: Al eliminar el ruido, el modelo se enfoca en el código real y los errores, no en barras de progreso.
3.  **Privacidad Total**: Axiom redacta automáticamente claves API y secretos **localmente** antes de que lleguen a la nube.

---

## 📖 Documentación / Documentation

Selecciona tu idioma para empezar:

### 🇪🇸 [Español (ES)](docs/es/README.md)
- 🚀 **[Instalación en 1 Minuto](docs/es/empezando/instalacion.md)**
- ⚡ **[Guía de Inicio Rápido](docs/es/empezando/inicio-rapido.md)**
- 💡 **[Conceptos y Privacidad](docs/es/guia-usuario/conceptos-clave.md)**

### 🇺🇸 [English (EN)](docs/en/README.md)
- 🚀 **[Getting Started](docs/en/getting-started/installation.md)**
- 💡 **[Core Concepts](docs/en/user-guide/core-concepts.md)**

---

## 🛠️ Instalación Rápida

```bash
cargo install --git https://github.com/mpineda/axiom
axiom install
```

Simplemente antepón `axiom` a cualquier comando:
```bash
axiom npm install
```

---
*“From raw bytes to semantic intent.”*
