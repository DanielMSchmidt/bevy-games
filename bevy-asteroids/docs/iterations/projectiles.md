# Iteration: Projectiles

## Branch
dev/projectiles

---

## Goal

Player can fire projectiles by pressing spacebar. Bullets spawn at the ship's position,
travel in the ship's forward direction, and despawn after a short lifetime.
A cooldown prevents the player from firing continuously.

---

## Learning Focus

- Bevy events (`Event`, `EventWriter`, `EventReader`)
- Decoupling intent (input) from action (spawning)
- Component-based lifetime / cooldown tracking
- Entity despawning

---

## Constraints

- No external physics engine
- One bullet type for now (speed is a constant)
- Keep systems small and focused

---

## Acceptance Criteria

- [ ] Spacebar fires a bullet in the ship's forward direction
- [ ] Bullet travels at constant speed with true inertia (uses existing `Velocity` component)
- [ ] Cooldown prevents firing faster than once per N seconds
- [ ] Bullets despawn after a fixed lifetime
- [ ] Code compiles cleanly with no warnings
- [ ] No unused components or systems

---

## Questions Before Implementation

(To be answered before coding)

### Components

- What marker component identifies a bullet entity? What module does it live in?
  - `Bullet` component in the `common` module
- Bullets need to despawn after time — what component tracks remaining lifetime, and what type does it hold?
  - `Lifetime` component in the `common` module, holds a `f32` representing remaining time
- The ship needs a cooldown between shots — what component tracks this, and what type does it hold?
  - `Cooldown` component in the `common` module, holds a `f32` representing remaining cooldown time
- Which entities get which components at spawn time, and which system is responsible for each spawn?
  -  Bullets get `Bullet` and `Lifetime` components, added by the bullet spawner system
  - The ship gets a `Cooldown` component, added in the rendering system when the ship is spawned

### Event

- What is the event called, and where does it live (which module)?
  - `ShootEvent` in the `gameplay` module
- What two fields does it carry, and what are their types?
   - `origin: Vec3` and `direction: Vec3`
- Which system sends it (`EventWriter`)? Which system receives it (`EventReader`)?
   - The shoot input system sends it, and the bullet spawner system receives it.

### Systems

For each of these three systems, answer: what module, what does it read, what does it write, one sentence summary.

- **Shoot input system** — decides whether to fire
  - reads ship cooldown, velocity, and transform, as well as keyboard input; writes `ShootEvent` and ship cooldown
- **Bullet spawner system** — reacts to the event and creates the bullet entity
  -  reads `ShootEvent`; writes new bullet entities with `Bullet`, `Lifetime`, `Velocity`, and `Transform` components
- **Bullet lifetime system** — ticks lifetime down and despawns expired bullets
  - reads `Lifetime` and delta time; writes updated `Lifetime` and despawns entities when lifetime expires

### Resources

- Which built-in Bevy resources do these systems need?
  - time (for delta time) and input (for keyboard input)
- Is `commands` a resource? (hint: look at how `spawn_scene` receives it)
  -  Yes, `Commands` is a resource that systems can receive to issue commands like spawning and despawning entities.

### Scheduling

- Which schedule do all three systems run in?
    - Update
- Does ordering matter between any of them? Think through what happens if the spawner runs before the input system on a given frame.
  - Yes: `bullet_lifetime` → `shoot_input` → `bullet_spawner` → `apply_movement`
  - Lifetime runs first so dead bullets are despawned before any other system processes them
  - Shoot input runs before the spawner so events sent this frame are received this frame
  - Physics runs last so all new bullets have their velocity set before movement is applied

---

## Post-Implementation Review Request

AI Agent, please review:

1. ECS correctness
2. Idiomatic Bevy usage
3. Over-complexity
4. Hidden performance issues
5. Merge readiness
