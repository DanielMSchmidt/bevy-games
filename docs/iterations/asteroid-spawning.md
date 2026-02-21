# Iteration: Asteroid Spawning

## Branch
dev/asteroid-spawning

---

## Goal

Asteroids spawn periodically outside the visible area, move toward the center, and spin with random angular velocity. They vary in size. No collision yet.

---

## Learning Focus

- Bevy `Timer` resource for periodic spawning
- Random number generation in systems (`rand` crate)
- Spawning entities with varied component values
- Resources vs components for global state

---

## Constraints

- No collision detection
- No asteroid splitting
- Simple sphere/ellipsoid mesh as placeholder
- Spawn radius hardcoded (no frustum calculation needed yet)

---

## Acceptance Criteria

- [ ] Asteroids spawn at random positions outside a radius around the origin
- [ ] Each asteroid has a random velocity directed roughly toward the center
- [ ] Each asteroid has a random angular velocity (spin)
- [ ] Asteroids vary in size (at least two distinct sizes)
- [ ] Spawning happens periodically via a timer, not all at once
- [ ] Code compiles cleanly with no warnings
- [ ] No unused components or systems

---

## Questions Before Implementation

(To be answered before coding)

### Components

- What marker component identifies an asteroid? Which module does it live in?
   - common::Asteroid 
- Asteroids vary in size — what component holds size data, and what type does it use?
  - common::Size (f32)
- Which components from previous iterations can asteroids reuse directly? (think about what makes them move and spin)
  - common::Velocity
  - common::AngularVelocity
  - common::Transform
- Does an asteroid need a `Lifetime`? Why or why not?
    - no, it will despawn when destroyed by the player 

### Timer / Spawning Trigger

- What triggers asteroid spawning — a Bevy `Timer`, a  raw `f32` countdown, or something else?
  - timer
- Where does the timer live — as a `Resource` or a component on some entity?
  - resource 
- What is a Bevy `Resource` vs a component? (hint: a `Resource` is global, not attached to any entity)
  - A `Resource` is a global singleton that can be accessed by systems, whereas a component is attached to an entity.
- How do you register a `Resource` in a plugin? (hint: `app.insert_resource(...)`)
  - app.insert_resource

### Systems

- What system handles spawning, and which module does it go in?
  - gameplay::asteroid_spawner
- What does it read? What does it write?
  - timer resource (read), spawns asteroid entities (write)
- How do you pick a random spawn point on a circle of radius R around the origin?
  - Use polar coordinates: angle = random(0, 2π), x = R * cos(angle), z = R * sin(angle)
- How do you make the asteroid's velocity point roughly toward the center from its spawn point?
  - Subtract the spawn position from the origin and normalize the vector

### Scheduling

- Which `GameSystemSet` does the asteroid spawner belong in?
    - gameplay
- Does it need ordering relative to any existing systems?
    - yes, before physics so that new asteroids are included in movement updates immediately

---

## Post-Implementation Review Request

AI Agent, please review:

1. ECS correctness
2. Idiomatic Bevy usage
3. Over-complexity
4. Hidden performance issues
5. Merge readiness
