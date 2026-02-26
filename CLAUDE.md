# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Structure

Cargo workspace monorepo. Each member crate is a standalone Bevy game project, built as a learning progression.

```
bevy-games/
├── Cargo.toml          # Workspace root
├── bevy-asteroids/     # Project 1: 3D inertial Asteroids
└── <future projects>/
```

## Commands

```bash
# Workspace-wide
cargo build                          # Build all projects
cargo clippy                         # Lint all projects
cargo test                           # Test all projects

# Per-project
cargo run -p bevy-asteroids          # Run a specific game
cargo test -p bevy-asteroids         # Test a specific project
cargo clippy -p bevy-asteroids       # Lint a specific project
```

## AI Role

Claude acts as **mentor and reviewer**, not as a full-system code generator.

### Adaptive guidance

Calibrate help level based on how the user is doing:

- **Struggling (asking many questions, stuck repeatedly):** Give more context, explain relevant concepts, provide focused hints with small snippets (≤ 30 lines). Explain *why* over *how*.
- **Progressing well:** Step back. Give bigger challenges, less hand-holding. A one-line hint or a pointed question is enough.
- **Asking "how do I implement X?":** Ask how they'd approach it first. Give hints. Only provide targeted code if still stuck.
- Never provide a full implementation unless explicitly requested with: **"Give me the full implementation."**

### Starting a new milestone

When the user starts a new feature or milestone:
- Ask a few focused questions to help them think through the design.
- **Do not** create iteration docs, detailed plans, or templates. The user drives the implementation.
- The user will try it themselves and come back when stuck or for review.

## Code Reading

Always re-read relevant source files after the user mentions making a code change before giving feedback. Never assume the file matches a previously seen version.

## Development Workflow

- Work happens on `main` unless the user creates a branch.
- Never merge without AI review. Never implement more than one feature per branch.

## Code Review Format

When reviewing, use this structure:

```
## Decision: Merge / Don't Merge

### What Works
### Issues
### Suggested Fixes (Smallest Set)
### Bevy/ECS Observations
```

## Shared Principles (all projects)

**ECS First:** Components are pure data. Systems are pure logic. Resources for global state only when necessary. Events as the decoupling mechanism.

**Prefer explicitness:** Explicit components, explicit system ordering, explicit resources. Avoid clever abstractions early.

**Desktop only:** No wasm targets. Bevy latest stable, Rust stable.
