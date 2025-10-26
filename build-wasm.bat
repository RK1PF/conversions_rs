@echo off
REM Build script for compiling conversions_rs to WebAssembly (Windows)

echo Building conversions_rs for WebAssembly...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: wasm-pack is not installed. Please install it first:
    echo cargo install wasm-pack
    exit /b 1
)

REM Create output directory if it doesn't exist
if not exist pkg mkdir pkg

REM Build for web target
echo Building for web target...
wasm-pack build --target web --out-dir pkg/web --features wasm

REM Build for node target
echo Building for Node.js target...
wasm-pack build --target nodejs --out-dir pkg/nodejs --features wasm

REM Build for bundler target (webpack, rollup, etc.)
echo Building for bundler target...
wasm-pack build --target bundler --out-dir pkg/bundler --features wasm

echo Build complete! Generated packages:
echo   - Web: pkg/web/
echo   - Node.js: pkg/nodejs/
echo   - Bundler: pkg/bundler/
echo.
echo Example usage:
echo   import init, { convert_length_wasm } from './pkg/web/conversions_rs.js';
echo.
echo For more examples, see the demo.html file.