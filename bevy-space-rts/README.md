# Space RTS — Asset Preview

A Bevy project showcasing space-themed 3D assets with camera controls and basic selection. This is the foundation for a space RTS game — no gameplay mechanics yet, just a motivating "wow it runs" scene.

## Setup

### 1. Fetch Assets

The game uses free 3D models from the [Kenney Space Kit](https://kenney.nl/assets/space-kit) (CC0 license).

```bash
./scripts/fetch_assets.sh
```

This downloads and extracts the curated models into `assets/models/`.

### 2. Run

```bash
cargo run -p bevy-space-rts
```

With dev inspector:

```bash
cargo run -p bevy-space-rts --features dev
```

## Controls

| Key | Action |
|-----|--------|
| WASD / Arrow keys | Pan camera |
| Mouse wheel | Zoom in/out |
| Q / E | Rotate camera |
| Left click | Select ship |

## Assets

All models are from the Kenney Space Kit (CC0). See [`assets/CREDITS.md`](assets/CREDITS.md) for full attribution.

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # App builder, plugin registration
├── camera/          # RTS camera (pan, zoom, rotate)
├── selection/       # Click-to-select + selection ring highlight
├── rendering/       # Lights, environment
├── ui/              # Help overlay
├── assets/          # Asset loading + scene spawning
└── common/          # Shared components and types
```
