# bevy-asteroids

## Game

3D inertial Asteroids-like game. The ship has true inertia — thrust adds velocity, there is no friction. Asteroids split on impact. Projectiles are fired via events.

## Learning Goals

- Bevy ECS fundamentals: components, systems, resources, events
- System ordering and explicit scheduling
- 3D scene setup: camera, lighting, meshes
- Input handling separated from game logic (Input → Intent → Physics)
- Custom simple physics integration (no physics engine)
- Collision detection (sphere-based)
- Game state management (menu, playing, game over)

## Architecture

**Input → Intent → Physics:** These three stages must always be separate systems. Never mix input reading, velocity mutation, and transform mutation.

**Physics:** Custom simple integration — no external physics engine until core loop is validated and fun.

## Planned Module Structure

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
```

## Progression

1. ~~Scene + camera + ship~~
2. ~~Inertial movement (velocity + angular velocity, true inertia)~~
3. ~~Projectiles (via events)~~
4. ~~Asteroid spawning~~
5. ~~Collision (sphere-based)~~
6. Game state
7. Polish

## Review Checklist

See `docs/REVIEW_CHECKLIST.md`.
