# GitHub Pages Setup for WASM Demo

This document explains how the `docs/pkg/web` folder is set up for GitHub Pages to showcase the WASM functionality.

## What's Included

The `docs/pkg/web/` directory contains the compiled WASM artifacts needed for the GitHub Pages demo:

- `conversions_rs.js` - Main JavaScript bindings
- `conversions_rs_bg.wasm` - WebAssembly binary
- `conversions_rs.d.ts` - TypeScript definitions
- `conversions_rs_bg.wasm.d.ts` - WASM-specific TypeScript definitions
- `package.json` - npm package configuration
- `README.md` - Package documentation
- `LICENSE-APACHE` & `LICENSE-MIT` - License files

## GitHub Pages Configuration

1. **Repository Settings**: Go to Settings → Pages
2. **Source**: Deploy from a branch
3. **Branch**: Select `main` 
4. **Folder**: Select `/docs`

The demo will be available at: `https://rk1pf.github.io/conversions_rs/`

## How the Demo Works

The `docs/index.html` file loads the WASM module from `docs/pkg/web/` and provides an interactive interface for testing unit conversions directly in the browser.

## Git Configuration

The `.gitignore` has been updated to:
- Keep ignoring the root `/pkg/` directory (build artifacts)
- Allow the `docs/pkg/` directory to be tracked
- Include `*.wasm` files specifically for the docs directory

```gitignore
# WebAssembly build artifacts
/pkg/
!docs/pkg/
*.wasm
!docs/pkg/**/*.wasm
```

## Updating the Demo

To update the GitHub Pages demo with new WASM builds:

1. Build the WASM package: `./build-wasm.sh` or `./build-wasm.bat`
2. Copy the web target to docs: `cp -r pkg/web/* docs/pkg/web/`
3. Commit and push: `git add docs/pkg/web/ && git commit -m "Update WASM demo" && git push`

The GitHub Pages site will automatically update within a few minutes.

## Automation Possibility

You could also automate this by adding a GitHub Action that:
1. Builds the WASM on every release
2. Updates the `docs/pkg/web` directory  
3. Commits and pushes the changes

This would ensure the demo is always up-to-date with the latest release.