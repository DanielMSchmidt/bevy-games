# Iteration: Collision

## Branch
dev/collision

---

## Goal

Implement sphere-based collision detection between bullets/asteroids and ship/asteroids.
Bullets despawn asteroids (or split them). Ship collision triggers game over (for now: just despawn the ship).
Collision detection is decoupled from response via events.

---

## Learning Focus

- Querying multiple entity types in one system
- Firing events from a detection system
- Reacting to events in separate response systems
- Iterating all pairs of entities (nested queries)

---

## Constraints

- Sphere-based only — distance < sum of radii
- No world boundary wrapping yet
- Game over = ship despawns (no UI yet)
- Asteroid splitting: two children, half size, parent velocity rotated ±45°

---

## Acceptance Criteria

- [ ] Bullet hitting an asteroid despawns the bullet
- [ ] Bullet hitting a size 1.0 asteroid despawns the asteroid
- [ ] Bullet hitting a size 2.0 or 4.0 asteroid splits it into two smaller ones
- [ ] Ship touching an asteroid despawns the ship
- [ ] Split asteroids inherit parent position and spread velocity
- [ ] No crashes or warnings on compile
- [ ] No unused components or systems

---

## Questions Before Implementation

(To be answered before coding)

### Collision Detection

- Two spheres collide when: distance between centers < sum of radii. Where does each entity's radius come from?
    - From the `Size` component on asteroids. Bullets use a fixed radius constant. Meshes are rendering only — never read them for gameplay logic.
- The detection system needs to compare bullets against asteroids, and ship against asteroids. How do you query two different entity types in the same system? (hint: you can have two separate `Query` parameters)
  - You can have two separate `Query` parameters, one for each entity type.
- Why can't you use a single query for both bullets and asteroids in the nested loop?
    - Rust's borrow checker prevents iterating the same query twice simultaneously. Two separate queries are separate borrows, so Bevy can handle them correctly.

### Events

- What two collision events are needed? What data should each carry?
    - bullet-asteroid collision event: bullet entity, asteroid entity
    - ship-asteroid collision event: ship entity, asteroid entity
- Which module do they live in?
    - `gameplay` module
- Does the detection system need to know what happens after a collision, or does it just fire the event?
    - It just fires the event 

### Response Systems

- What does the bullet/asteroid response system do? List each step.
  - Despawn the bullet
  - Check the size of the asteroid
  - If size == 1.0, despawn the asteroid
  - If size > 1.0, split the asteroid into two smaller ones:
    - Spawn two new asteroid entities with half the size of the original
    - Set their position to the original asteroid's position
    - Rotate the original velocity by ±45° and assign it to the children
- What does the ship/asteroid response system do?
    - End the game by despawning the ship
- Where do these systems live — `gameplay` or somewhere else?
    - `gameplay` module
- When an asteroid splits, the two children need rotated velocity. Which Bevy/math type rotates a Vec3?
    - `Quat::from_rotation_y(angle).mul_vec3(velocity)`. Use ±`std::f32::consts::FRAC_PI_4` for ±45°.

### Scheduling

- Which `GameSystemSet` does collision detection belong in? (hint: it needs to run after physics has moved everything)
    - A new collisions one?
- Should response systems run in the same set as detection, or after?
    - After
- Does `GameSystemSet` need a new variant?
  - Yes: two new variants — `CollisionDetection` and `CollisionResponse`, both after `Physics` in the chain.

---

## Post-Implementation Review Request

AI Agent, please review:

1. ECS correctness
2. Idiomatic Bevy usage
3. Over-complexity
4. Hidden performance issues
5. Merge readiness
