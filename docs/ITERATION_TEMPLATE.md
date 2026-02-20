# Iteration: <Feature Name>

## Branch
dev/<feature-name>

---

## Goal

Describe the small feature being implemented.

Example:
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

(To be answered before coding)

- What components are needed?
- What systems are needed?
- What resources (if any) are needed?
- In which schedule should systems run?

---

## Post-Implementation Review Request

AI Agent, please review:

1. ECS correctness
2. Idiomatic Bevy usage
3. Over-complexity
4. Hidden performance issues
5. Merge readiness

