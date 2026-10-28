#!/bin/bash

set -e

echo "Rust Cookbook Development Script"
echo "================================"

# The version is in .mdbook-version; set MDBOOK_VERSION to override it.
MDBOOK_VERSION="${MDBOOK_VERSION:-$(cat "$(dirname "${BASH_SOURCE[0]}")/../.mdbook-version")}"

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null; then
    echo "Installing mdbook..."
    cargo install mdbook --version "=${MDBOOK_VERSION}" --locked
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