.PHONY: build clean release debug install

# Build release binary into build directory
build: release

# Build release version
release:
	@echo "Building release binary..."
	@cargo build --release
	@mkdir -p build
	@cp target/release/cleaner build/cleaner
	@echo "✓ Binary built successfully: build/cleaner"
	@ls -lh build/cleaner

# Build debug version
debug:
	@echo "Building debug binary..."
	@cargo build
	@mkdir -p build
	@cp target/debug/cleaner build/cleaner
	@echo "✓ Debug binary built: build/cleaner"
	@ls -lh build/cleaner

# Clean all build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf build
	@echo "✓ Clean complete"

# Install to system (requires appropriate permissions)
install: release
	@echo "Installing cleaner to /usr/local/bin..."
	@install -m 755 build/cleaner /usr/local/bin/cleaner
	@echo "✓ Installation complete"

# Show help
help:
	@echo "Cleaner Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build    - Build release binary to build/ directory (default)"
	@echo "  release  - Build optimized release binary"
	@echo "  debug    - Build debug binary"
	@echo "  clean    - Remove all build artifacts"
	@echo "  install  - Install binary to /usr/local/bin (requires sudo)"
	@echo "  help     - Show this help message"
