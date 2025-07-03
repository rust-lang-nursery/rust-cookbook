#!/bin/bash

set -e

echo "Rust Cookbook Development Script"
echo "================================"

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null; then
    echo "Installing mdbook..."
    cargo install mdbook --vers "0.4.43"
fi

# Build the book
echo "Building the book..."
mdbook build

# Copy assets
echo "Copying assets..."
cp -r assets/ book/

# Run tests
echo "Running tests..."
cargo test

# Run spellcheck
echo "Running spellcheck..."
./ci/spellcheck.sh list

echo "Build complete! Open book/index.html in your browser to view the site."
echo "Or run 'mdbook serve --open' to start a local server." 