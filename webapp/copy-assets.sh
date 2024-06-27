#!/bin/bash
# Get the directory of the current script
script_dir=$(dirname "$(readlink -f "$0")")

# Define the relative paths to the binaries
tauri_dist_binary="$script_dir/src-tauri/bin/core-aarch64-apple-darwin"
core_dist_binary="$script_dir/../core/dist/core-aarch64-apple-darwin"

# Check if the core_dist_binary exists
if [ ! -f "$core_dist_binary" ]; then
    echo "Error: Source binary $core_dist_binary does not exist."
    exit 1
fi

# Check if the tauri_dist_binary exists
if [ ! -f "$tauri_dist_binary" ]; then
    echo "Destination binary $tauri_dist_binary does not exist. Copying from $core_dist_binary."
    cp "$core_dist_binary" "$tauri_dist_binary"
else
    # Compare the binaries
    if cmp -s "$tauri_dist_binary" "$core_dist_binary"; then
        echo "Binaries are the same. No action required."
    else
        echo "Binaries are different. Replacing $tauri_dist_binary with $core_dist_binary."
        cp "$core_dist_binary" "$tauri_dist_binary"
    fi
fi