# This is a justfile comment
default:
    @echo "Hello! Run 'just --list' to see available commands"

build:
    cargo build --release
    # Build succeeded

test:
    @echo "Running tests..."
    # Your test commands here
