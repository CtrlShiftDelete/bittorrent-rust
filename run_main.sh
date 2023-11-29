#!/bin/bash

# Ensure the Rust program is built before running
cargo build --release

# Check if the build was successful
if [ $? -eq 0 ]; then
    # Run the Rust program with the provided arguments
    cargo run --release "$@"
else
    echo "Error: Rust build failed."
fi
