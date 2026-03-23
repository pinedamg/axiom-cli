# 🛰️ Plan del Proyecto: Axiom Pulse (El Plano de Control)

**Versión**: 1.0.0 (Arquitectura Industrial Independiente)  
**Proyecto Padre**: [Axiom CLI](https://github.com/mpineda/axiom)  
**Visión**: De telemetría bruta a inteligencia de negocio semántica.

---

## 1. Resumen Ejecutivo
**Axiom Pulse** es el centro de observabilidad independiente para el ecosistema de Axiom CLI. Está diseñado como una **Arquitectura de Celda (Isla Aislada)** para asegurar que la ingesta y el análisis de telemetría no interfieran con la infraestructura existente. Proporciona información en tiempo real sobre el ahorro de tokens, brechas en los esquemas y la salud del sistema.

---

## 2. Contexto del Sistema Host (Objetivo: Escritorio)
El despliegue está optimizado para las siguientes especificaciones:
- **SO**: Ubuntu 22.04.5 LTS (x86_64)
- **Recursos**: 12 vCPUs / 31GiB RAM / 25GB de Disco Disponible.
- **Red**: 300Mbps Bajada / 30Mbps Subida (Detrás de Cloudflare Zero Trust).

---

## 3. Arquitectura de Alto Nivel (La Estrategia de "Isla")
Axiom Pulse opera en su propia red dedicada de Docker (`pulse-internal`), completamente aislada de otros stacks como `proxy-net` o `data-net`.

### Componentes:
1.  **`pulse-api` (Motor de Ingesta)**: Binario de alto rendimiento en Rust (Axum).
2.  **`pulse-db` (Almacenamiento)**: PostgreSQL 16 con la extensión **TimescaleDB** para optimización de series temporales.
3.  **`pulse-redis` (Buffer)**: Instancia dedicada de Redis para manejar picos de ingesta.
4.  **`pulse-dashboard` (UI)**: Next.js 14 + Tremor.so para métricas estilo "Vercel".
5.  **`pulse-tunnel` (Ingreso)**: Túnel de Cloudflare independiente (`cloudflared`) para acceso seguro y sin puertos abiertos.

---

## 4. Esquema de Base de Datos (TimescaleDB)
Diseñado para manejar millones de eventos con agregación instantánea.

### Tabla: `installations`
Rastrea instancias únicas de Axiom.
- `iid`: UUID (Clave Primaria)
- `os`: String (linux, macos, windows)
- `arch`: String (x86_64, aarch64)
- `version`: String (ej. 0.1.0)
- `is_pro`: Booleano
- `created_at`: Timestamp

### Tabla: `usage_events` (Hypertable)
Rastrea cada ejecución de comando.
- `id`: BigInt
- `iid`: UUID (FK)
- `command_bin`: String (ej. `git`, `npm`)
- `raw_bytes`: Entero
- `saved_bytes`: Entero
- `saving_ratio`: Float
- `created_at`: Timestamp (Particionado por tiempo)

---

## 5. Características de Visión: "Los 4 Pilares del Entendimiento"

### I. Motor de Descubrimiento de Esquemas
Identifica comandos utilizados por los usuarios que **Axiom CLI** no está comprimiendo actualmente.
- **Lógica**: Cualquier `usage_event` donde `saving_ratio < 5%` y `command_bin` no está en nuestra lista de Tiers conocidos.
- **Objetivo**: Desarrollo proactivo de esquemas basado en la demanda del mundo real.

### II. Dashboard de Economía de Tokens
Visualización del ROI en tiempo real.
- **Métricas**: Total de tokens ahorrados globalmente.
- **Finanzas**: USD estimados ahorrados (Total Tokens * $0.01 / 1k).
- **LinkedIn Hook**: *"Axiom Pulse ha ahorrado a la comunidad $X,XXX en los últimos 30 días."*

---

## 6. Estrategia de Implementación (El Repo "Axiom Pulse")

### Estructura del Repositorio:
```text
axiom-pulse/
├── api/                # Axum (Rust)
├── dashboard/          # Next.js + Tremor
├── db/                 # Scripts de inicio SQL
├── .env                # CF_TUNNEL_TOKEN, DB_PASSWORD
└── docker-compose.yml  # Stack Independiente
```

---
*“Axiom Pulse: Convirtiendo métricas brutas en crecimiento semántico.”*
