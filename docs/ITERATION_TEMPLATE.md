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

### Components
- What new components are needed? (name, inner type, why)
- Which module do they live in? (hint: if multiple systems share them, think `common`)
- Which entities need these components attached at spawn time, and where does that spawn happen?

### Systems
- What systems are needed? (one per responsibility)
- For each system:
  - What module does it go in?
  - What does it read? (Query components, Resources)
  - What does it write?
  - One sentence: what does it do?

### Resources
- What built-in Bevy resources are needed? (e.g. `Time`, `Input<KeyCode>`)
- Any new resources required, or is entity-level component data sufficient?

### Scheduling
- Which schedule? (`Startup` / `Update` / `FixedUpdate`)
- Does ordering between systems matter? If so, which must run before which and why?

---

## Post-Implementation Review Request

AI Agent, please review:

1. ECS correctness
2. Idiomatic Bevy usage
3. Over-complexity
4. Hidden performance issues
5. Merge readiness

