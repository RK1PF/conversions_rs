#!/bin/bash
# Build script for compiling conversions_rs to WebAssembly

set -e

echo "Building conversions_rs for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed. Please install it first:"
    echo "cargo install wasm-pack"
    exit 1
fi

# Create output directory if it doesn't exist
mkdir -p pkg

# Build for web target
echo "Building for web target..."
wasm-pack build --target web --out-dir pkg/web --features wasm

# Build for node target
echo "Building for Node.js target..."
wasm-pack build --target nodejs --out-dir pkg/nodejs --features wasm

# Build for bundler target (webpack, rollup, etc.)
echo "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg/bundler --features wasm

echo "Build complete! Generated packages:"
echo "  - Web: pkg/web/"
echo "  - Node.js: pkg/nodejs/"
echo "  - Bundler: pkg/bundler/"
echo ""
echo "Example usage:"
echo "  import init, { convert_length_wasm } from './pkg/web/conversions_rs.js';"
echo ""
echo "For more examples, see the demo.html file."