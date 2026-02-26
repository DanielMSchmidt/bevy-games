#!/usr/bin/env bash
# Downloads and extracts curated space assets from Kenney's Space Kit (CC0).
# Run from the project root: ./scripts/fetch_assets.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ASSETS_DIR="$PROJECT_DIR/assets"
MODELS_DIR="$ASSETS_DIR/models"
TMP_DIR=$(mktemp -d)

ZIP_URL="https://kenney.nl/media/pages/assets/space-kit/cceeafbd0c-1677698978/kenney_space-kit.zip"

echo "=== Kenney Space Kit Asset Fetcher ==="
echo ""

# Create directories
mkdir -p "$MODELS_DIR"

echo "Downloading Kenney Space Kit..."
curl -L -o "$TMP_DIR/space-kit.zip" "$ZIP_URL"

echo "Extracting..."
unzip -q "$TMP_DIR/space-kit.zip" -d "$TMP_DIR/space-kit"

# Find the GLTF directory (Kenney packs vary in structure)
GLTF_DIR=$(find "$TMP_DIR/space-kit" -type d -iname "*gltf*" | head -1)

if [ -z "$GLTF_DIR" ]; then
    # Try GLB format
    GLTF_DIR=$(find "$TMP_DIR/space-kit" -type d -iname "*glb*" | head -1)
fi

if [ -z "$GLTF_DIR" ]; then
    echo "Could not find GLTF/GLB directory. Listing contents:"
    find "$TMP_DIR/space-kit" -type d | head -20
    echo ""
    echo "Trying to find model files directly..."
    GLTF_DIR="$TMP_DIR/space-kit"
fi

echo "Found models in: $GLTF_DIR"

# Curated models we want:
# - craft_speederA: main ship model
# - rock_largeA: large asteroid
# - rock_smallA: small asteroid
# - hangar_roundA: space station / HQ
MODELS=(
    "craft_speederA"
    "rock_largeA"
    "rocks_smallA"
    "hangar_roundA"
    "craft_speederB"
    "craft_speederC"
)

copied=0
for model in "${MODELS[@]}"; do
    # Try .glb first, then .gltf
    src=$(find "$GLTF_DIR" -iname "${model}.glb" -type f | head -1)
    if [ -z "$src" ]; then
        src=$(find "$GLTF_DIR" -iname "${model}.gltf" -type f | head -1)
    fi

    if [ -n "$src" ]; then
        cp "$src" "$MODELS_DIR/"
        echo "  Copied: $(basename "$src")"
        copied=$((copied + 1))
    else
        echo "  WARNING: Could not find model '$model'"
    fi
done

# Cleanup
rm -rf "$TMP_DIR"

echo ""
echo "Done! Copied $copied models to $MODELS_DIR"
echo ""

if [ "$copied" -eq 0 ]; then
    echo "ERROR: No models were found. The Kenney pack structure may have changed."
    echo "Please download manually from https://kenney.nl/assets/space-kit"
    echo "and place GLTF/GLB files in assets/models/"
    exit 1
fi

echo "You can now run the game with: cargo run -p bevy-space-rts"
