# Repository Kickoff – 3D Inertial Asteroids (Bevy)

## Objective

Create a clean Bevy project structure for a 3D inertial Asteroids-like game.

This is a one-time bootstrap.
After this commit, the AI becomes mentor/reviewer only.

---

## Requirements

### Engine
- Latest stable Bevy
- Rust stable toolchain

### Project Type
- 3D game
- Desktop target
- No wasm initially

---

## Initial Structure

src/
│
├── main.rs
├── app.rs
├── input/
│   └── mod.rs
├── physics/
│   └── mod.rs
├── gameplay/
│   └── mod.rs
├── rendering/
│   └── mod.rs
├── ui/
│   └── mod.rs
└── common/
    └── mod.rs

assets/
├── models/
├── textures/
├── audio/

docs/
    (this folder)

---

## Bootstrap Responsibilities

The AI may:

- Initialize cargo project
- Configure Bevy dependency
- Add a basic 3D scene:
  - Camera
  - Directional light
  - Simple ship mesh (primitive OK)
- Ensure project runs

The AI must NOT:

- Implement gameplay features
- Implement inertial movement
- Implement shooting
- Implement collision

---

## Asset Guidelines

Use simple primitives first.
Optional: Include small CC0 placeholder models.

No asset complexity yet.

---

## Deliverable

- Compiling project
- Clean module boundaries
- Clear app setup
- Documented main entry point

After bootstrap, stop implementing features.
Switch to mentor mode permanently.
