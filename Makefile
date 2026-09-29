.PHONY: help build serve test lint fmt fmt-check deploy deploy-skip-tests dev clean install-mdbook

help: ## Show this help message
	@echo "Rust Cookbook - Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

install-mdbook: ## Install mdbook with the correct version
	cargo install mdbook --vers "0.5.3"

build: install-mdbook ## Build the book locally
	mdbook build
	cp -r assets/ book/
	@echo "Build complete! Open book/index.html in your browser."

fmt: ## Format workspace crates and the code blocks inline in the markdown
	cargo fmt --all
	cargo xtask fmt-md

fmt-check: ## Check formatting without writing changes
	cargo fmt --all -- --check
	cargo xtask fmt-md --check

lint: fmt-check ## Run the formatting checks and clippy
	cargo clippy --workspace --all-targets -- -D warnings

test: ## Run all tests
	cargo test
	./ci/spellcheck.sh list

dev: build test ## Build and test (development workflow)
	@echo "Development build complete!"

deploy: clean dev ## Deploy to GitHub Pages (requires maintainer permissions)
	./scripts/deploy.sh --skip-tests

deploy-skip-tests: build ## Deploy to GitHub Pages without running tests
	./scripts/deploy.sh --skip-tests

clean: ## Clean build artifacts
	cargo clean
	rm -rf book/
	@echo "Clean complete!"

serve: install-mdbook ## Serve the book locally with live reload
	mdbook serve --open
