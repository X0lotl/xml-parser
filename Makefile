# Variables
CARGO = cargo
TARGET = target/debug/xml-parser

# Default target
all: build

# Build the project
build:
	$(CARGO) build

# Run the project
run: build
	$(TARGET)

# Run tests
test:
	$(CARGO) test

# Format the code
format:
	$(CARGO) fmt

# Check for linting issues
clippy:
	$(CARGO) clippy -- -D warnings

# Clean the project
clean:
	$(CARGO) clean

# Prepare for commit (format and clippy)
pre-commit: format clippy

# Help message
help:
	@echo "Makefile commands:"
	@echo "  all         - Build the project"
	@echo "  build       - Build the project"
	@echo "  run         - Run the project"
	@echo "  test        - Run tests"
	@echo "  format      - Format the code"
	@echo "  clippy      - Check for linting issues"
	@echo "  clean       - Clean the project"
	@echo "  pre-commit  - Format and check for linting issues before committing"
	@echo "  help        - Display this help message"

.PHONY: all build run test format clippy clean pre-commit help