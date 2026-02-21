# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # Build the project
cargo run            # Run the game
cargo check          # Fast type-check without linking
cargo clippy         # Lint
cargo test           # Run all tests
cargo test <name>    # Run a single test by name
```

## Project

3D inertial Asteroids-like game built with Bevy (latest stable) and Rust stable. Desktop target only, no wasm.

## Planned Module Structure

The game is in early bootstrap phase. The intended source layout (not yet implemented) is:

```
src/
├── main.rs
├── app.rs
├── input/      # Input reading only — never mutates velocity or transforms
├── physics/    # Velocity/angular velocity integration
├── gameplay/   # Asteroids, projectiles, collision, game state
├── rendering/  # Mesh/visual concerns
├── ui/
└── common/     # Shared types, components

assets/
├── models/
├── textures/
└── audio/
```

## Architecture Principles

**ECS First:** Components are pure data. Systems are pure logic. Resources for global state only when necessary. Events as the decoupling mechanism.

**Input → Intent → Physics:** These three stages must always be separate systems. Never mix input reading, velocity mutation, and transform mutation.

**Planned progression (do not skip steps):**
1. Scene + camera + ship
2. Inertial movement (velocity + angular velocity, true inertia)
3. Projectiles (via events)
4. Asteroid spawning
5. Collision (sphere-based first)
6. Game state
7. Polish

**Physics:** Custom simple integration — no external physics engine until core loop is validated and fun.

**Prefer explicitness:** Explicit components, explicit system ordering, explicit resources. Avoid clever abstractions early.

## AI Role

Per `docs/AI_AGENT_CHARTER.md`, Claude acts as **mentor and reviewer**, not as a full-system code generator.

Default behavior:
- Guide with questions and hints; provide small illustrative snippets (≤ 30 lines).
- Explain *why* over *how*.
- Never provide a full implementation unless explicitly requested with: **"Give me the full implementation."**

If the user asks "how do I implement X?": first ask how they would approach it, then give hints, then minimal targeted code only if still blocked.

## Code Reading

Always re-read relevant source files after the user mentions making a code change before giving feedback. Never assume the file matches a previously seen version.

## Development Workflow

- Feature branches: `dev/<feature>`
- For each new iteration, **always create** `docs/iterations/<feature>.md` without being asked. Tailor the questions to the specific feature based on what has been discussed — don't leave them generic. The user answers them before coding starts.
- Each iteration: one clear task (30–90 min), with acceptance criteria, pitfalls, and which Bevy concepts are exercised.
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

Use `docs/REVIEW_CHECKLIST.md` as the evaluation guide. Do not rewrite entire systems; provide the smallest fix set.
