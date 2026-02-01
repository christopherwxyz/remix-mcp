# justfile for remix-mcp
# https://github.com/casey/just

set shell := ["bash", "-cu"]

# Default recipe - show available commands
default:
    @just --list

# Build the project
build:
    cargo build

# Build release version
release:
    cargo build --release

# Run all checks (format, lint, test)
check: fmt-check lint test

# Run tests
test:
    cargo test

# Run tests with nextest (faster)
nextest:
    cargo nextest run

# Run integration tests (requires Ableton Live with AbletonOSC)
integration:
    cargo test --test integration -- --ignored --test-threads=1

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Check formatting
fmt-check:
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Run the MCP server
serve:
    cargo run

# Install AbletonOSC Remote Script
install:
    cargo run -- install

# Check AbletonOSC installation status
status:
    cargo run -- status

# Generate changelog
changelog:
    git cliff -o CHANGELOG.md

# Generate changelog for unreleased changes
changelog-unreleased:
    git cliff --unreleased

# Clean build artifacts
clean:
    cargo clean

# Check dependencies for vulnerabilities and license issues
deny:
    cargo deny check

# Update dependencies
update:
    cargo update

# Run all CI checks
ci: fmt-check lint test deny

# Watch for changes and run tests
watch:
    cargo watch -x test

# Count lines of code
loc:
    @tokei --type Rust

# Show tool count
tools:
    @grep -r '#\[tool(' src/tools/*.rs | grep -v tool_router | wc -l | tr -d ' '
