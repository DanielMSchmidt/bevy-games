# bevy-space-rts

## Game

3D-looking, top-down space RTS with asteroid-based base building and supply chain logistics.

The game takes place on a 2D gameplay plane using 3D visuals (low poly ships, asteroids, stations). Players expand across asteroid fields, construct industrial infrastructure, establish automated transport routes, and build fleets to destroy the enemy HQ.

Ships, stations, and logistics operate through clearly separated simulation systems (movement, economy, combat). The focus is on clean ECS architecture and scalable simulation.

---

## Core Gameplay Loop

1. Expand to asteroids  
2. Mine resources  
3. Transport materials between nodes  
4. Refine into advanced components  
5. Produce ships  
6. Protect supply lines  
7. Destroy the enemy base  

---

## Learning Goals

### ECS & Architecture
- Large-scale ECS simulation (100–500+ entities)
- Strict separation of simulation and rendering
- Command-based architecture (Orders as components/events)
- Data-driven production chains
- Clear scheduling and system ordering

### RTS-Specific Concepts
- Unit selection (single + drag-box)
- Command issuing (move, attack, build)
- Formation-friendly movement
- Basic pathfinding or steering + avoidance
- State machines via components (Idle, Moving, Attacking, Mining, Transporting)

### Economy & Logistics
- Resource storage and transfer systems
- Automated transport routes
- Production queues
- Build constraints (large asteroid required for shipyard)

### 3D in 2D Plane
- Orthographic camera in a 3D world
- Raycasting from screen to plane
- Visual clarity and selection feedback

### AI (later phase)
- Simple build order AI
- Attack wave generation
- Target prioritization

---

## Libraries Policy (No Reinventing the Wheel)

Using community libraries/plugins is explicitly encouraged when it saves time on solved problems (input mapping, debug tooling, pathfinding, UI). The goal is not to avoid libraries, but to:

- Keep the dependency footprint small
- Understand how each library integrates into Bevy ECS (read examples/source)
- Implement *small, focused* systems from scratch when educationally valuable

Libraries should be added **phase-by-phase**, only when needed.

---

## Architectural Principles

### Simulation First

Rendering must never contain gameplay logic.

Simulation should remain deterministic and independent from visuals where possible.

---

### Orders, Not Direct Mutation

Never directly mutate units in input systems. Use command components or events:

- `MoveOrder`
- `AttackOrder`
- `BuildOrder`
- `RouteAssigned`

Flow: Input → Intent → Commands → Simulation


---

### System Separation

Movement, economy, and combat must be isolated domains.

- Movement systems never read raw input
- Economy systems never mutate transforms
- Combat systems never perform pathfinding
- Rendering systems never modify gameplay state

---

### No Physics Engine Initially

Movement should use:
- Acceleration + max speed
- Steering behaviors
- Simple obstacle avoidance

A physics engine is optional later, not required for MVP.

---

## World Structure

- Small asteroids → mining only  
- Large asteroids → allow shipyards  
- Planets → provide rare resources (later phase)  
- Free space → fleet movement & combat  

All movement occurs on a single gameplay plane (Z fixed).

---

## MVP Scope (Strictly Limited)

### Resources
- Ore (single resource for MVP — no refining step)

### Buildings
- HQ
- Mine (produces Ore, has local storage with max capacity)
- Shipyard (large asteroid only, consumes Ore to produce ships)

### Units
- Construction Ship (flies to build site, starts construction, then transporters deliver materials, building completes after a delay once materials arrive)
- Transport Ship (automated — finds nearest storage with needed resource, delivers to consuming buildings on demand)
- Fighter (attack-move for MVP, explicit target control later)

### Economy Flow
All production and construction uses the same transport mechanic:
1. A building (or construction site) needs resources
2. Nearest free transporter picks up from nearest source with stock
3. Transporter delivers to the destination
4. This adds natural latency — distance and transporter availability matter strategically

### Construction Flow
1. Player places a building → ghost appears
2. Construction ship flies to site and begins setup
3. Transporters deliver required materials
4. Building completes after materials arrive + build time

### Victory Condition
- Destroy enemy HQ

No tech tree.  
No diplomacy.  
No fog of war (initially).  

---

## Planned Module Structure

src/
├── main.rs
├── app.rs
├── camera/
├── input/ # Input → Intent only
├── commands/ # Order components + events
├── movement/ # Steering + pathing
├── selection/ # Drag-box, raycast, selection state
├── economy/ # Resources, storage, production
├── logistics/ # Transport routes + delivery logic
├── combat/ # Targeting, cooldowns, damage
├── construction/ # Building placement + validation
├── ai/ # Opponent logic (later)
├── rendering/ # Meshes, selection rings, VFX
├── ui/
└── common/


---

## Development Progression + Recommended Libraries

### Phase 0 — Codebase Walkthrough

Goal: Understand every piece of the existing scaffold before writing new code.

Walk through the full project structure (plugins, modules, components, systems) and explain:
- How the Bevy app is wired together (plugins, system registration)
- The camera system (orthographic, pan/zoom/rotate, `compute_camera_transform`)
- How assets are loaded (glTF scenes, `SceneRoot`, `AssetServer`)
- The component model so far (`Ship`, `Asteroid`, `Station`, `Selectable`)
- Selection system (raycasting, click-to-select)
- Rendering setup (lighting, clear color, the gameplay plane)
- UI overlay
- How `bevy_inspector_egui` integrates behind a feature flag

This is lecture-style — no coding, just building a mental model of what exists.

---

### Phase 0.5 — Dev Quality-of-Life (Optional but Recommended)

Goal: Faster iteration and debugging.

**Recommended Libraries:**
- `bevy_inspector_egui` — inspect entities/resources live (already added)
- `bevy_egui` — debug UI panels, toggles, metrics

These can be limited to dev builds.

---

### Phase 1 — RTS Core (Camera, Selection, Move)

- Camera (orthographic or angled)
- Click selection + drag selection box
- Right-click move orders
- Simple steering movement

**Recommended Libraries:**
- `leafwing-input-manager` — clean action mapping
- Optional: `bevy_mod_raycast` for raycasting utilities

Avoid pathfinding libraries at this stage.

---

### Phase 2 — Asteroid Building (Placement + Constraints)

- Asteroids as build anchors
- Snap-based placement & validation
- Large asteroid requirement for shipyard
- Storage component per node

**Recommended Libraries:**
- None required (custom implementation preferred)

---

### Phase 3 — Economy (Mining + Production)

- Mines produce Ore into local storage (capped)
- Shipyard consumes Ore to produce ships
- Production queues with build time

**Recommended Libraries:**
- None required

---

### Phase 4 — Logistics (Automated Transport)

- Transport ships (automated, not player-routed)
- Find nearest source with stock, deliver to consuming building
- Transport availability and distance create natural strategic tension

**Recommended Libraries:**
- None required
- Optional: `rand` or `bevy_turborand` for procedural variation

---

### Phase 5 — Combat (Targeting, Damage, Death)

- Range detection
- Cooldowns
- Health and destruction
- Optional projectiles

**Recommended Libraries:**
- None required
- Optional: `bevy_rapier3d` if physics-based projectiles are desired (not required)

---

### Phase 6 — Navigation Upgrade (Pathfinding / Avoidance)

Add only if steering becomes insufficient.

Options:
- Custom minimal grid-based A*
- Or a Bevy navigation plugin (if actively maintained)

**Recommended Libraries (Optional):**
- A Bevy-compatible pathfinding/navigation crate

Keep this limited to one navigation solution.

---

### Phase 7 — AI Opponent

- Simple build order logic
- Attack waves
- Harass supply lines

**Recommended Libraries:**
- Optional: `big-brain` (utility AI)
- Otherwise custom goal/state logic

---

## Dependency Budget

To avoid bloat:

- Max 1 input library
- Max 1 UI/debug library
- Max 1 navigation library
- Physics only if justified

Each dependency must have a clear purpose.

---

## Constraints

- No feature creep
- No premature optimization
- No over-abstracted engine architecture
- Each phase must produce a playable slice

---

## Stretch Goals (Late Phase)

- Additional resource types (energy from stars, water from planets)
- Supply chains (refining raw materials into advanced components)
- Explicit target control for combat units
- Deterministic simulation
- Replay system
- Fog of war
- Fleet formations
- Multiplayer lockstep

---

## Design Philosophy

This project prioritizes clean, scalable RTS simulation architecture over visual polish.

The primary educational goal is:

> Designing and implementing a multi-system RTS economy + combat loop correctly using Bevy ECS.
