#!/bin/bash

set -e

# Settings

NAME="GlicolVST"

# Script

VST_NAME="$NAME.vst"
MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"
TMP_DIR="tmp"

cargo build --release
./scripts/osx_vst_bundler.sh "$NAME" ./target/release/libglicol_vst.dylib

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

if mv "$TMP_DIR/$VST_NAME" "$MOVE_TO"; then
    echo "Copied VST bundle to $MOVE_TO"
fi