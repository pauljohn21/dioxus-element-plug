.PHONY: help build-css watch-css build dev clean

help: ## Show this help message
	@echo "Usage: make [target]" && echo && \
	grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s%s\\n", $$1, $$2}'

# Directories
SCSS_SRC := scss
CSS_DIST := dist
NODE_MODULES := node_modules

# Check if dependencies are installed
HAS_NODE := $(shell command -v node 2> /dev/null)
HAS_NPM := $(shell command -v npm 2> /dev/null)
HAS_SASS := $(shell command -v sass 2> /dev/null)

build-css: ## Compile SCSS to CSS
ifndef HAS_NODE
	@echo "Error: node is not installed"
	@exit 1
endif
ifndef HAS_NPM
	@echo "Error: npm is not installed"
	@exit 1
endif
ifndef HAS_SASS
	@echo "Installing sass..."
	npm install -g sass
endif
	@mkdir -p $(CSS_DIST)
	@echo "Compiling SCSS files from organized structure..."
	sass $(SCSS_SRC)/index.scss $(CSS_DIST)/theme-chalk.css --style=compressed
	@echo "CSS compiled to $(CSS_DIST)/theme-chalk.css"

watch-css: ## Watch and compile SCSS files on change
ifndef HAS_SASS
	npm install -g sass
endif
	sass --watch $(SCSS_SRC)/index.scss:$(CSS_DIST)/theme-chalk.css

build: build-css ## Build the Rust library and CSS
	@cargo build --release
	@echo "Build complete! CSS available at $(CSS_DIST)/theme-chalk.css"

dev: build-css ## Build for development
	@cargo build
	@echo "Development build complete!"

setup: ## Setup development environment
ifndef HAS_NODE
	@echo "Please install Node.js first"
	@exit 1
endif
ifndef HAS_NPM
	@echo "Please install npm first"
	@exit 1
endif
	@echo "Installing sass compiler..."
	npm install -g sass
	@echo "Setup complete! Run 'make build-css' to compile CSS"

clean: ## Clean build artifacts
	rm -rf target/
	rm -rf $(CSS_DIST)/
	@echo "Clean complete"

# Create package.json if it doesn't exist
package.json:
	@echo "Creating package.json..."
	@echo '{ \
  "name": "theme-chalk", \
  "version": "1.0.0", \
  "description": "Element UI theme-chalk CSS", \
  "main": "index.js", \
  "scripts": { \
    "build": "sass src/index.scss dist/theme-chalk.css --style=compressed", \
    "watch": "sass --watch src/index.scss:dist/theme-chalk.css" \
  }, \
  "devDependencies": { \
    "sass": "^1.77.0" \
  } \
}' > package.json

npm-setup: package.json ## Setup npm dependencies
	npm install

# Install cargo-watch for Rust development (optional)
cargo-watch:
	@cargo install cargo-watch

# Production build
production: build-css
	@cargo build --release
	@echo "Production build complete with optimized CSS"
