#!/bin/bash

set -e

echo "Rust Cookbook Local Deployment Script"
echo "====================================="

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

# Check if we're on master branch
if [[ $(git branch --show-current) != "master" ]]; then
    echo "Warning: You're not on the master branch. Current branch: $(git branch --show-current)"
    echo "This script is designed to deploy from the master branch."
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Get current commit hash
REV=$(git rev-parse --short HEAD)
echo "Building for commit: $REV"

# Create a temporary directory for the gh-pages branch
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: $TEMP_DIR"

# Clone the repository to the temporary directory
echo "Cloning repository to temporary directory..."
git clone --depth 1 --branch gh-pages https://github.com/rust-lang-nursery/rust-cookbook.git "$TEMP_DIR" 2>/dev/null || {
    echo "gh-pages branch doesn't exist yet, creating it..."
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    git init
    git remote add origin https://github.com/rust-lang-nursery/rust-cookbook.git
    cd - > /dev/null
}

# Copy the built book to the temporary directory
echo "Copying built book to temporary directory..."
rm -rf "$TEMP_DIR"/*
cp -r book/* "$TEMP_DIR/"

# Commit and push to gh-pages branch
echo "Committing and pushing to gh-pages branch..."
cd "$TEMP_DIR"
git add -A .
git config user.name "Rust Cookbook Maintainer"
git config user.email "cookbook@rust-lang.org"
git commit -m "Build cookbook at $REV" || {
    echo "No changes to commit"
    exit 0
}

echo "Pushing to gh-pages branch..."
git push origin HEAD:gh-pages --force

# Clean up
cd - > /dev/null
rm -rf "$TEMP_DIR"

echo "Deployment complete!"
echo "The site should be available at: https://rust-lang-nursery.github.io/rust-cookbook"
echo "Note: It may take a few minutes for GitHub Pages to update." 