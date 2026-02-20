# Architecture Principles

## ECS First

- Components: pure data
- Systems: pure logic
- Resources: global state only when necessary
- Events: decoupling mechanism

---

## Input → Intent → Physics

Never mix:

- Input reading
- Velocity mutation
- Transform mutation

These must be separate systems.

---

## Keep It Boring First

Before adding:
- Physics engines
- Complex rendering
- Particles
- Advanced collision

Ensure:
- Core loop is fun
- Controls feel good
- Inertia works properly

---

## Prefer Explicitness Over Magic

- Explicit components
- Explicit system ordering
- Explicit resources

Avoid clever abstractions early.

---

## Optimize After Validation

Do not:
- Micro-optimize early
- Abstract prematurely
- Generalize before necessary
