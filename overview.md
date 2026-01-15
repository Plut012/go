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
