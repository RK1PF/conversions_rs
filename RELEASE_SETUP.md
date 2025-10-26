# Release Setup Guide

This document explains how to set up the automated release pipeline for the conversions_rs project, including WASM artifact generation and npm publishing.

## Required GitHub Secrets

To enable automated releases, you need to configure the following secrets in your GitHub repository settings (`Settings > Secrets and variables > Actions`):

### 1. CRATES_IO_TOKEN
- **Purpose**: Publishes the Rust crate to crates.io
- **How to get**: 
  1. Visit [crates.io](https://crates.io) and log in
  2. Go to Account Settings → API Tokens
  3. Create a new token with publishing permissions
  4. Add it as a GitHub secret

### 2. NPM_TOKEN
- **Purpose**: Publishes the WASM package to npm
- **How to get**:
  1. Log in to [npmjs.com](https://npmjs.com)
  2. Go to Access Tokens in your account settings
  3. Create a new "Automation" token
  4. Add it as a GitHub secret

## Release Process

### Automatic Release
1. Push a new git tag with the format `v*.*.*` (e.g., `v1.2.0`)
2. The workflow will automatically:
   - Create a GitHub release
   - Build and upload platform-specific binaries
   - Build and upload WASM packages for web, Node.js, and bundler targets
   - Publish to crates.io
   - Publish to npm

### Manual Release Steps
```bash
# Update version in Cargo.toml and package.json files
# Commit your changes
git add .
git commit -m "Bump version to 1.2.0"

# Create and push tag
git tag v1.2.0
git push origin v1.2.0
```

## Generated Artifacts

Each release will include:

### Binary Releases
- `conversions_rs-linux-x86_64` - Linux binary
- `conversions_rs-windows-x86_64.exe` - Windows binary  
- `conversions_rs-macos-x86_64` - macOS binary

### WASM Packages
- `conversions_rs-wasm-web.tar.gz` - Web target package
- `conversions_rs-wasm-nodejs.tar.gz` - Node.js target package
- `conversions_rs-wasm-bundler.tar.gz` - Bundler target package

### Package Registries
- **crates.io**: Rust crate for native usage
- **npm**: WASM package for JavaScript/TypeScript usage

## Testing Before Release

The CI pipeline tests WASM builds on every push to ensure they work correctly. You can also test locally:

```bash
# Test WASM build
./build-wasm.sh

# Run WASM tests
node test-wasm.js
```

## Troubleshooting

### Failed npm Publish
- Ensure the npm token has publishing permissions
- Check that the package name in `pkg/web/package.json` is available
- Verify the version number is higher than the last published version

### Failed crates.io Publish
- Ensure the crates.io token has publishing permissions
- Check that the version in `Cargo.toml` is higher than the last published version
- Verify all required metadata is present in `Cargo.toml`

### WASM Build Failures
- Ensure `wasm-pack` is installed: `cargo install wasm-pack`
- Check that the `wasm` feature is properly configured in `Cargo.toml`
- Verify the `wasm32-unknown-unknown` target is installed: `rustup target add wasm32-unknown-unknown`