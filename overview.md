# Project Overview

## Goals
Build a web-based Go (Weiqi/Baduk) game for two players to play against each other.

## Ultimate Goals
- Host as a minimal, lightweight webpage
- Deliver a polished, themeable experience that feels unique
- Maintain a codebase that is easy to understand, extend, and maintain

## Core Concept
A robust Rust backend handles all game logic. A completely independent frontend renders the game state with swappable visual themes. The two layers communicate through a clean API—the backend knows nothing about how it looks, the frontend knows nothing about the rules.

---

# Development Philosophy

**Simple** — No unnecessary abstractions. If 50 lines work, don't write 200.

**Robust** — Handle edge cases. Compile-time guarantees over runtime surprises.

**Concise** — Every line earns its place.

**Clean** — Readable by a stranger. Self-documenting where possible.

**Decoupled** — Backend and frontend are fully independent. Themes never touch game logic. Game logic never assumes a renderer.

---

# Development Workflow

All code must be simple and robust. Development occurs on arch linux system. 

### 1. DISCUSSION
You and I will discuss the goals of a new feature—what problem it solves, who benefits, and whether it aligns with the project vision.

### 2. QUESTIONS / CLEVER IDEAS
We will together explore implementation approaches. Always keeping the user experience in mind. Challenge assumptions. Surface tradeoffs early.

### 3. IMPACT ANALYSIS
Consider which components need to be updated and identify other components that will be affected. Map dependencies before writing code.

### 4. TODO PLAN
Create a clear, detailed technical plan for implementing the feature. This plan lives in `docs/todo_tasks/` and includes:
- Specific files to create or modify
- Data structures and interfaces
- Test cases to cover
- Acceptance criteria

### 5. IMPLEMENTATION
Build the feature, ensuring simplicity and robustness throughout. If a decision point arises during implementation, pause and begin discussion—present the options with pros and cons. We decide together how to proceed.

--- 

# Technical Stack

### Backend
- Rust with Axum — minimal web framework, async, WebSocket support
- No database — game state lives in memory (HashMap of active games)
- Single binary deployment

### Communication
- WebSocket for real-time moves between players
- JSON payloads (simple, debuggable)

### Frontend
- Svelte — reactive, minimal runtime, clean component model
- SVG rendering — scalable pieces, easy theme swapping
- CSS custom properties for palette tokens

### Build/Dev
- Cargo for backend
- Vite for frontend (fast HMR with Svelte)
- Both run independently during development

### Hosting
- Static files (frontend) served from anywhere
- Rust binary on a small VPS, Fly.io, or similar

### Why this stack:
- Zero runtime dependencies on backend (no Node, no Python, no DB)
- Frontend framework disappears at build time (Svelte compiles away)
- WebSocket keeps latency low, code simple
- SVG + CSS variables = themes are just asset folders

