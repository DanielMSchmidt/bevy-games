# Iteration: Inertia Movement

## Branch
add-velocity

---

## Goal

Implement inertial ship movement with velocity and angular velocity.

---

## Learning Focus

- Bevy systems
- Components
- Delta time
- Transform updates
- Separation of input and motion

---

## Constraints

- No external physics engine
- No over-engineering
- Keep systems small and clear

---

## Acceptance Criteria

- [ ] Ship rotates with torque (continues spinning after input release)
- [ ] Ship accelerates in forward direction
- [ ] Movement is framerate independent
- [ ] Code compiles cleanly
- [ ] No unused components or systems

---

## Questions Before Implementation


- What components are needed?
    - common::Velocity(Vec3) and common::AngularVelocity(f32)
- What systems are needed?
    - input::read_movement_input
    - physics::apply_movement
- What resources (if any) are needed?
    - time
- In which schedule should systems run?
    - Update
- We need to attach the components in the rendering.
---

## Post-Implementation Review Request

AI Agent, please review:

1. ECS correctness
2. Idiomatic Bevy usage
3. Over-complexity
4. Hidden performance issues
5. Merge readiness

