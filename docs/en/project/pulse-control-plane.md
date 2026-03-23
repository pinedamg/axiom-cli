# 🛰️ Project Plan: Axiom Pulse (The Control Plane)

**Version**: 1.0.0 (Standalone Industrial Architecture)  
**Parent Project**: [Axiom CLI](https://github.com/mpineda/axiom)  
**Vision**: From raw telemetry to semantic business intelligence.

---

## 1. Executive Summary
**Axiom Pulse** is the independent observability hub for the Axiom CLI ecosystem. It is designed as a **Cell Architecture (Isolated Island)** to ensure that the telemetry ingestion and analysis do not interfere with existing infrastructure. It provides real-time insights into token savings, schema gaps, and system health.

---

## 2. Host System Context (Target: Desktop)
The deployment is optimized for the following specifications:
- **OS**: Ubuntu 22.04.5 LTS (x86_64)
- **Resources**: 12 vCPUs / 31GiB RAM / 25GB Avail. Disk.
- **Network**: 300Mbps Down / 30Mbps Up (Behind Cloudflare Zero Trust).

---

## 3. High-Level Architecture (The "Island" Strategy)
Axiom Pulse operates on its own dedicated Docker network (`pulse-internal`), completely isolated from other stacks like `proxy-net` or `data-net`.

### Components:
1.  **`pulse-api` (Ingestion Engine)**: High-performance Rust (Axum) binary.
2.  **`pulse-db` (Storage)**: PostgreSQL 16 with **TimescaleDB** extension for time-series optimization.
3.  **`pulse-redis` (Buffer)**: Dedicated Redis instance to handle ingestion spikes.
4.  **`pulse-dashboard` (UI)**: Next.js 14 + Tremor.so for "Vercel-style" metrics.
5.  **`pulse-tunnel` (Ingress)**: Standalone Cloudflare Tunnel (`cloudflared`) for secure, port-less access.

---

## 4. Database Schema (TimescaleDB)
Designed to handle millions of events with instant aggregation.

### Table: `installations`
Tracks unique Axiom instances.
- `iid`: UUID (Primary Key)
- `os`: String (linux, macos, windows)
- `arch`: String (x86_64, aarch64)
- `version`: String (e.g., 0.1.0)
- `is_pro`: Boolean
- `created_at`: Timestamp

### Table: `usage_events` (Hypertable)
Tracks every command execution.
- `id`: BigInt
- `iid`: UUID (FK)
- `command_bin`: String (e.g., `git`, `npm`)
- `raw_bytes`: Integer
- `saved_bytes`: Integer
- `saving_ratio`: Float
- `created_at`: Timestamp (Time-partitioned)

### Table: `crashes`
Tracks system failures via the Panic Hook.
- `iid`: UUID (FK)
- `error_msg`: Text
- `location`: String (file:line)
- `created_at`: Timestamp

---

## 5. Vision Features: "The 4 Pillars of Understanding"

### I. Schema Discovery Engine
Identifies commands used by users that **Axiom CLI** is currently not compressing.
- **Logic**: Any `usage_event` where `saving_ratio < 5%` and `command_bin` is not in our known Tier list.
- **Goal**: Proactive schema development based on real-world demand.

### II. Token Economy Dashboard
Real-time ROI visualization.
- **Metrics**: Total tokens saved globally.
- **Financials**: Estimated USD saved (Total Tokens * $0.01 / 1k).
- **LinkedIn Hook**: *"Axiom Pulse has saved the community $X,XXX in the last 30 days."*

### III. Health & Stability monitor
Real-time tracking of CLI crashes.
- **Logic**: Grouping crashes by version and OS.
- **Goal**: Rapid hot-fixes for v0.1.1+ before users report them.

### IV. Version Distribution
Monitoring the adoption of new releases.
- **Logic**: Counting unique `iid` per `version` in the last 24h.

---

## 6. Implementation Strategy (The "Axiom Pulse" Repo)

### Repository Structure:
```text
axiom-pulse/
├── api/                # Axum (Rust)
├── dashboard/          # Next.js + Tremor
├── db/                 # SQL Init scripts
├── .env                # CF_TUNNEL_TOKEN, DB_PASSWORD
└── docker-compose.yml  # Standalone Stack
```

### Ingress Flow:
`CLI` -> `Cloudflare Edge` -> `Cloudflare Tunnel (pulse-tunnel)` -> `pulse-api` -> `pulse-db`

---

## 7. Deployment Instructions

1.  **Cloudflare Setup**: Create a new Tunnel named `axiom-pulse` in the Zero Trust Dashboard. Obtain the Token.
2.  **Environment Configuration**:
    - `CF_PULSE_TOKEN`: Your Cloudflare Tunnel Token.
    - `DATABASE_URL`: Connection string for internal network.
3.  **Run with Docker Compose**:
    ```bash
    docker-compose up -d --build
    ```

---

## 8. Connection to Axiom CLI
The **Axiom CLI** (v0.1.0) must be updated to point to the new Pulse API endpoint:
- **Old Endpoint**: `https://subdomain.mpineda.com.ar/api/collections/telemetry/records`
- **New Pulse Endpoint**: `https://pulse-api.mpineda.com.ar/v1/report`

---
*“Axiom Pulse: Turning raw metrics into semantic growth.”*
