#!/bin/bash
# Get the directory of the current script
script_dir=$(dirname "$(readlink -f "$0")")

# Define the relative paths to the binaries
tauri_dist_binary="$script_dir/src-tauri/bin"
core_dist_binary="$script_dir/../core/dist"

# Create the tauri_dist_binary directory if it does not exist
mkdir -p "$tauri_dist_binary"

# Copy all files in core_dist_binary to tauri_dist_binary
cp -r "$core_dist_binary/"* "$tauri_dist_binary"

# make all files in tauri_dist_binary executable
if [[ "$OSTYPE" != "msys" ]]; then
  find "$tauri_dist_binary" -type f -exec chmod +x {} \;
fi

