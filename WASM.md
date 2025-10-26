# WebAssembly Quick Start Guide

This guide helps you quickly get started with using the conversions_rs library in WebAssembly.

## Prerequisites

1. Install `wasm-pack`:
   ```bash
   cargo install wasm-pack
   ```

2. Install Node.js (for testing and development server)

## Quick Start

### 1. Build WASM Package

Choose your target platform:

```bash
# For web browsers (ES modules)
wasm-pack build --target web --out-dir pkg/web --features wasm

# For Node.js applications
wasm-pack build --target nodejs --out-dir pkg/nodejs --features wasm

# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler --out-dir pkg/bundler --features wasm

# Build all targets at once (Windows)
./build-wasm.bat

# Build all targets at once (Unix/Linux/macOS)
./build-wasm.sh
```

### 2. Test the WASM Module

Run the included test script:
```bash
node test-wasm.js
```

### 3. Try the HTML Demo

1. Build the web package:
   ```bash
   wasm-pack build --target web --out-dir pkg/web --features wasm
   ```

2. Start a local server:
   ```bash
   python -m http.server 8000
   # or
   npx serve .
   ```

3. Open `http://localhost:8000/demo.html` in your browser

### 4. Use in Your Project

#### Web Browser (ES Modules)

```html
<!DOCTYPE html>
<html>
<head>
    <script type="module">
        import init, { convert_length_wasm } from './pkg/web/conversions_rs.js';
        
        async function run() {
            await init();
            
            const result = convert_length_wasm(100, "ft", "m");
            if (result.success) {
                console.log(`100 feet = ${result.value} meters`);
            }
        }
        
        run();
    </script>
</head>
<body>
    <h1>Conversions RS Demo</h1>
</body>
</html>
```

#### Node.js

```javascript
const { convert_length_wasm } = require('./pkg/nodejs/conversions_rs.js');

const result = convert_length_wasm(100, "ft", "m");
if (result.success) {
    console.log(`100 feet = ${result.value} meters`);
} else {
    console.error("Error:", result.error);
}
```

#### React/Next.js

```jsx
import { useEffect, useState } from 'react';

function Converter() {
    const [wasm, setWasm] = useState(null);
    const [result, setResult] = useState('');

    useEffect(() => {
        import('./pkg/web/conversions_rs.js').then(async (wasmModule) => {
            await wasmModule.default();
            setWasm(wasmModule);
        });
    }, []);

    const handleConvert = () => {
        if (!wasm) return;
        
        const conversionResult = wasm.convert_length_wasm(100, "ft", "m");
        if (conversionResult.success) {
            setResult(`100 feet = ${conversionResult.value.toFixed(4)} meters`);
        } else {
            setResult(`Error: ${conversionResult.error}`);
        }
    };

    return (
        <div>
            <button onClick={handleConvert} disabled={!wasm}>
                Convert 100ft to meters
            </button>
            {result && <p>{result}</p>}
        </div>
    );
}
```

#### TypeScript

The WASM package includes TypeScript definitions:

```typescript
import init, { 
    convert_length_wasm, 
    ConversionResult 
} from './pkg/web/conversions_rs.js';

await init();

const result: ConversionResult = convert_length_wasm(100, "ft", "m");
if (result.success) {
    console.log(`Result: ${result.value}`);
}
```

## API Reference

### Conversion Functions

All conversion functions follow the same pattern:

```typescript
function convert_[TYPE]_wasm(value: number, from: string, to: string): ConversionResult
```

Available functions:
- `convert_length_wasm()` - Length conversions
- `convert_weight_wasm()` - Weight/mass conversions  
- `convert_temperature_wasm()` - Temperature conversions
- `convert_volume_wasm()` - Volume conversions
- `convert_time_wasm()` - Time conversions
- `convert_current_wasm()` - Electric current conversions
- `convert_substance_wasm()` - Amount of substance conversions
- `convert_luminous_intensity_wasm()` - Luminous intensity conversions
- `convert_area_wasm()` - Area conversions

### Result Type

```typescript
interface ConversionResult {
    success: boolean;   // Whether conversion succeeded
    value: number;      // Converted value (0 if failed)
    error?: string;     // Error message if failed
}
```

### Utility Functions

```typescript
function get_supported_units(conversion_type: string): string[]
```

Example:
```javascript
const lengthUnits = get_supported_units("length");
// Returns: ["m", "km", "cm", "mm", "ft", "in", "yd", "mi", ...]
```

## Supported Unit Types

- `"length"` - meters, feet, inches, etc.
- `"weight"` or `"mass"` - kilograms, pounds, etc.
- `"temperature"` - Celsius, Fahrenheit, Kelvin
- `"volume"` - liters, gallons, etc.
- `"time"` - seconds, minutes, hours, etc.
- `"current"` - amperes, milliamperes, etc.
- `"substance"` - moles, millimoles, etc.
- `"luminous_intensity"` - candela, millicandela, etc.
- `"area"` - square meters, acres, etc.

## Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## NPM Scripts

If you have a `package.json`, you can use these convenience scripts:

```bash
npm run build:wasm        # Build for web
npm run build:wasm:node   # Build for Node.js
npm run build:wasm:all    # Build all targets
npm run test:wasm         # Run tests
npm run demo              # Build and serve demo
```

## Troubleshooting

### Module not found errors
Make sure you've built the WASM package and the paths are correct.

### CORS errors in browser
Serve your files over HTTP/HTTPS, not file:// protocol.

### Large bundle size
The WASM file is optimized but still around 65KB. Consider lazy loading if size is critical.

For more examples and advanced usage, see the full documentation in the README.md file.