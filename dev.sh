#!/bin/bash

set -e

PLUGIN_NAME="audio-eq-plugin.vst3"
PLUGIN_BUNDLE_PATH="target/bundled/$PLUGIN_NAME"
VST3_SYSTEM_PATH="$HOME/Library/Audio/Plug-Ins/VST3/$PLUGIN_NAME"

echo "🔍 Checking build..."
cargo check

echo "🔨 Building plugin..."
cargo run --package xtask -- bundle audio-eq-plugin

echo "📦 Copying bundle to system VST3 folder..."
cp "$PLUGIN_BUNDLE_PATH" "$VST3_SYSTEM_PATH"

echo "✅ Done! If Reaper is open, re-scan or restart Reaper to detect changes."
