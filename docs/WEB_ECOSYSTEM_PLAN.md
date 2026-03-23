# 🌐 Project Brief: Axiom Web Ecosystem

**Target Audience for this Document**: AI Agents and Engineering Teams tasked with building the standalone web ecosystem for the Axiom Project.

---

## 1. What is Axiom CLI? (Project Context)

### The Problem
Modern AI coding assistants (like Cursor, Claude Code, and Gemini CLI) interact heavily with the user's terminal to build, debug, and run code. However, standard CLI tools (`npm install`, `docker logs`, `git diff`) were designed for human eyes, not LLM context windows. They output thousands of lines of redundant structural noise, progress bars, and successful boilerplate.
This "terminal noise" causes three critical issues for AI agents:
1.  **Massive Context Waste**: Users burn through millions of tokens (and money) just reading successful `npm` logs.
2.  **Context Dilution**: Critical error messages get buried under thousands of lines of "OK" messages, causing the AI to lose track of the actual problem.
3.  **Privacy Risks**: Unfiltered raw logs can accidentally leak sensitive API keys, PII, or internal IPs to third-party LLM providers.

### The Solution: Axiom CLI ("The Semantic Firewall")
Axiom is a high-performance, local-first CLI proxy written in Rust. It acts as an intelligent intermediary between the terminal and the AI agent. 
- **Intent-Aware Compression**: Instead of showing 500 lines of successful npm package installations, Axiom compresses it to: `[AXIOM] 542 packages installed successfully in 12s.`
- **Error Surfacing**: If a command fails, Axiom dynamically expands the output to show the exact error lines, ensuring the AI gets the context it needs to fix the bug.
- **Privacy Engine**: It uses Shannon Entropy and regex scanning to detect and redact secrets (API keys) and PII before the text is ever sent to the LLM.

**The Result**: A 60% to 90% reduction in token usage for everyday developer operations, while completely eliminating context dilution and data leaks. It operates with a sub-10ms overhead.

---

## 2. The Web Ecosystem Vision

Because Axiom runs purely as a local Rust binary, it needs a robust web presence to explain its value, show its global impact, and manage its operations. 

**Important Architectural Note**: The Web Ecosystem must be a **completely standalone project** (separate repository, separate stack). It should not be tangled with the Rust CLI's core logic. 

We are building three distinct web experiences:

### I. 🚀 Marketing Landing Page (The "Hook")
**Goal**: Educate developers and AI engineers about the problem they didn't know they had (context waste), and convince them to install Axiom.
- **Hero Section**: A powerful, side-by-side animated visualization. 
  - *Left side*: A noisy, scrolling terminal running `npm install` or `docker build` (burning tokens visually).
  - *Right side*: The Axiom-filtered output (clean, dense, 90% fewer tokens).
- **Core Pillars**: Highlight Privacy (Local-First), Speed (Native Rust), and Token Economy (Money saved).
- **Call to Action**: Installation instructions (`cargo install axiom` / `axiom install`).

### II. 📊 Public Dashboard (The "Open Metrics" View)
**Goal**: Provide transparent "Social Proof" to the community. Axiom collects *anonymized* telemetry (only binary names and byte savings, no arguments or PII). This dashboard proves the tool's worth.
- **Global Token Counter**: A massive, real-time ticker showing "Total Tokens Saved by the Axiom Community."
- **Financial Impact**: A translated metric showing "Estimated USD Saved" based on average API costs.
- **Top Schemas**: A leaderboard of which commands (e.g., `git`, `docker`, `ps`) are being compressed the most globally.
- **Adoption Graph**: Real-time chart of unique daily active nodes (installations).

### III. 🔒 Private Dashboard (Axiom Pulse - The Control Plane)
**Goal**: The internal admin panel for the Axiom creators to monitor system health and plan the roadmap.
- **Schema Discovery Engine**: If thousands of users are running a command that Axiom *doesn't* know how to compress yet (e.g., `kubectl`), it appears here as a "Missed Opportunity." This dictates the engineering roadmap.
- **Crash & Health Monitoring**: Real-time alerts if specific CLI versions are throwing Rust panics on certain OS environments.
- **User & License Management**: For managing Pro tier subscriptions and enterprise telemetry off-switches.

---

## 3. Technology Stack Guidelines

Since this is a greenfield standalone project, the AI should use modern, high-performance web standards:

- **Frontend Framework**: Next.js 14+ (React) or a lightweight alternative like Astro.
- **Styling**: **Vanilla CSS** is heavily preferred for the Landing Page to ensure absolute control over complex, high-performance animations (like the Hero terminal comparison). Tailwind can be used for the dashboards if development speed is required.
- **Visual Language**: "Industrial Cybernetic."
  - Dark mode native.
  - Colors: Deep Charcoal backgrounds, Electric Cyan accents, Safety Orange for alerts/savings highlights.
  - Typography: A strict mix of Monospace (JetBrains Mono) for data/terminals, and a clean Sans-Serif (Inter) for copy.
- **Backend/API (Pulse API)**: A fast ingestion engine. Node.js (Express/FastAPI) or Rust (Axum) depending on the team's preference.
- **Database**: PostgreSQL (with TimescaleDB for time-series telemetry data) and Redis for high-speed counter caching.

---

## 4. Infrastructure & Connectivity ("The Island Strategy")

To ensure the web ecosystem is secure and decoupled, it must follow an **Isolated Cell Architecture**.

### I. Cloudflare Zero Trust Integration
The entire stack (API, Dashboards, DB) should reside behind a **Cloudflare Tunnel (`cloudflared`)**. This allows the server to stay behind a strict firewall with **zero open inbound ports**.
- **Ingress Flow**: `CLI/Browser` -> `Cloudflare Edge` -> `Cloudflare Tunnel` -> `Internal Docker Network` -> `Web-API / Dashboard`.
- **Security**: Access to the Private Admin Dashboard must be gated via **Cloudflare Access** (OTP, GitHub Auth, or Service Tokens).

### II. Deployment Environment
- **Target OS**: Linux (Ubuntu 22.04 LTS optimized).
- **Orchestration**: Docker Compose with a dedicated internal network (`axiom-web-net`).
- **Isolation**: The database and Redis instances must **not** be exposed to the host; they should only be reachable by the Web-API container within the Docker network.

---

## 5. Execution Directive for Future AI Agents

When prompted to build this ecosystem:
1. Start with the **Landing Page** structure. Focus on the interactive Terminal Comparison component first, as it is the core selling point.
2. Ensure the design is highly responsive and looks like a premium, developer-focused infrastructure tool (think Vercel, Linear, or Stripe docs).
3. Build the backend API completely decoupled from the frontend, ensuring the CLI can send telemetry payloads via simple HTTP POST requests without needing complex authentication for anonymous events.