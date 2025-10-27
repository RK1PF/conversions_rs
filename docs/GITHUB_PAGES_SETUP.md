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

## Accessing the Demo

The demo is available at: `https://rk1pf.github.io/conversions_rs/`

## How the Demo Works

The `docs/index.html` file loads the WASM module from `docs/pkg/web/` and provides an interactive interface for testing unit conversions directly in the browser.