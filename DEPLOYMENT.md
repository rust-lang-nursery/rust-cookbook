# Deployment Guide

This document explains how the Rust Cookbook is deployed locally by maintainers.

## Overview

The project uses local deployment scripts to build and deploy to GitHub Pages. This approach allows maintainers to deploy without needing GitHub Actions workflow permissions.

## Local Deployment

### Deployment Script (`scripts/deploy.sh`)

The deployment script handles:
- **Building**: Builds the mdbook and copies assets
- **Testing**: Runs cargo tests and spellcheck
- **Deployment**: Pushes the built site to the `gh-pages` branch

### Makefile Commands

For convenience, a Makefile provides easy commands:

```bash
make help      # Show all available commands
make build     # Build the book locally
make test      # Run all tests
make dev       # Build and test (development workflow)
make deploy    # Deploy to GitHub Pages
make serve     # Serve locally with live reload
make clean     # Clean build artifacts
```

## Local Development

For local development and testing:

```bash
# Use the Makefile (recommended)
make dev

# Or run the development script
./scripts/dev.sh

# Or manually:
cargo install mdbook --vers "0.4.43"
mdbook build
cp -r assets/ book/
cargo test
./ci/spellcheck.sh list
```

## GitHub Pages Configuration

The site is deployed to GitHub Pages from the `gh-pages` branch. The GitHub Actions workflow automatically:
1. Builds the mdbook to the `book/` directory
2. Copies assets from `assets/` to `book/`
3. Pushes the contents to the `gh-pages` branch
4. GitHub Pages serves the site from this branch

## Troubleshooting

### Build Failures
- Check the GitHub Actions logs for specific error messages
- Ensure all code examples compile and pass tests
- Verify that all links in the documentation are valid

### Deployment Issues
- Ensure the repository has GitHub Pages enabled in Settings > Pages
- Check that the `GITHUB_TOKEN` secret is available (this is automatic for public repositories)
- Verify that the `gh-pages` branch is being created and updated

### Local Build Issues
- Make sure you have Rust and Cargo installed
- Install mdbook with the exact version: `cargo install mdbook --vers "0.4.43"`
- Run `cargo clean` if you encounter dependency issues

## Migration from Travis CI

This setup replaces the previous Travis CI configuration:
- `ci/deploy.sh` - Replaced by local deployment script (`scripts/deploy.sh`)
- `ci/test_script.sh` - Functionality integrated into local scripts
- `appveyor.yml` - Windows testing can be added if needed

The new setup provides the same functionality while allowing maintainers to deploy without needing GitHub Actions workflow permissions. 