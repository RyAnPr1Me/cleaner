#!/bin/bash
# Build script for Cleaner - Rust-based performance improver

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building Cleaner...${NC}"

# Build release version
cargo build --release

# Create build directory
mkdir -p build

# Copy binary to build directory
cp target/release/cleaner build/cleaner

echo -e "${GREEN}✓ Build complete!${NC}"
echo ""
echo "Binary location: build/cleaner"
ls -lh build/cleaner
echo ""
echo "To run: ./build/cleaner --help"
