# Justfile for Uranian Astrology project

# Default recipe
default:
  @just --list

# Build the project
build:
  cargo build

# Build for release
build-release:
  cargo build --release

# Run the application
run:
  cargo run

# Run with debug logging
run-debug:
  RUST_LOG=debug cargo run

# Watch for changes and rebuild
watch:
  cargo-watch -x build

# Watch for changes and run
watch-run:
  cargo-watch -x run

# Run tests
test:
  cargo nextest run

# Run tests with coverage
test-coverage:
  cargo test --all-features

# Format code
fmt:
  cargo fmt

# Lint code
lint:
  cargo clippy -- -D warnings

# Check code without building
check:
  cargo check

# Clean build artifacts
clean:
  cargo clean

# Setup development environment
setup:
  cargo install cargo-watch cargo-nextest cargo-expand just bacon
  
# Generate documentation
docs:
  cargo doc --open

# Update dependencies
update:
  cargo update

# Audit dependencies for security issues
audit:
  cargo audit
