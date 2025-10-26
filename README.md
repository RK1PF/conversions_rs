# Conversions RS - Comprehensive SI Unit Conversion Library 🔄

[![CI](https://github.com/RK1PF/conversions_rs/workflows/CI/badge.svg)](https://github.com/RK1PF/conversions_rs/actions)
[![Crates.io](https://img.shields.io/crates/v/conversions_rs.svg)](https://crates.io/crates/conversions_rs)
[![Documentation](https://docs.rs/conversions_rs/badge.svg)](https://docs.rs/conversions_rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/RK1PF/conversions_rs#license)
[![Downloads](https://img.shields.io/crates/d/conversions_rs.svg)](https://crates.io/crates/conversions_rs)

A comprehensive command-line unit conversion tool and Rust library with full SI (International System of Units) support.

## Installation

### From crates.io (Recommended)

Install the binary directly from crates.io:

```bash
cargo install conversions_rs
```

### From GitHub Releases

Download the latest binary for your platform from the [releases page](https://github.com/RK1PF/conversions_rs/releases).

### From Source

Clone the repository and build from source:

```bash
git clone https://github.com/RK1PF/conversions_rs.git
cd conversions_rs
cargo build --release
# The binary will be in target/release/conversions_rs
```

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
conversions_rs = "1.1.0"
```

## Features

✨ **Multi-Platform Support**: Command-line tool, Rust library, and WebAssembly module for web browsers

### SI Base Units
- **Length Conversions**: meters, kilometers, centimeters, millimeters, feet, inches, yards, miles
- **Mass Conversions**: kilograms, grams, pounds, ounces, tons, stones  
- **Temperature Conversions**: Celsius, Fahrenheit, Kelvin
- **Time Conversions**: seconds, minutes, hours, days, weeks, years, milliseconds, microseconds, nanoseconds
- **Electric Current Conversions**: amperes, milliamperes, microamperes, nanoamperes, kiloamperes
- **Amount of Substance Conversions**: moles, millimoles, micromoles, nanomoles, picomoles, kilomoles
- **Luminous Intensity Conversions**: candela, millicandela, kilocandela, hefnerkerze, international candle

### SI Derived Units
- **Volume Conversions**: liters, milliliters, gallons (US/UK), fluid ounces (US/UK), cups, pints, quarts
- **Area Conversions**: square meters, square centimeters, square kilometers, square feet, square inches, acres, hectares

## Usage

### Command-Line Mode (Non-Interactive)

You can use the app directly from the command line for quick conversions:

```bash
# SI Base Units
conversions_rs length 100 ft m          # 100 feet to meters
conversions_rs weight 10 kg lb          # 10 kilograms to pounds  
conversions_rs temperature 32 F C       # 32°F to Celsius
conversions_rs time 3600 s min          # 3600 seconds to minutes
conversions_rs current 1500 mA A        # 1500 milliamperes to amperes
conversions_rs amount 0.5 mol mmol      # 0.5 moles to millimoles
conversions_rs luminosity 2.5 cd mcd    # 2.5 candela to millicandela

# SI Derived Units  
conversions_rs volume 1 gal l           # 1 gallon to liters
conversions_rs area 10000 "m²" ha       # 10000 square meters to hectares
```

**Get help:**
```bash
conversions_rs --help                   # General help
conversions_rs length --help            # Help for length conversions
conversions_rs time --help              # Help for time conversions
conversions_rs current --help           # Help for current conversions
# ... and all other conversion types
```

### Interactive Mode

Run without arguments for the interactive menu:

```bash
cargo run
```

The app provides an interactive menu where you can:

1. Choose the conversion type (Length, Weight, Temperature, or Volume)
2. Enter the value to convert
3. Specify the source unit
4. Specify the target unit
5. Get the converted result

### Example Sessions

**Command-Line Mode:**
```bash
$ conversions_rs length 100 ft m
100 ft = 30.479999 m

$ conversions_rs temperature 32 F C  
32°F = 0.00°C

$ conversions_rs weight 5 kg lb
5 kg = 11.023100 lb

$ conversions_rs volume 1 gal l
1 gal = 3.785410 l
```

**Interactive Mode:**

```
🔄 Unit Conversion App
======================

Choose conversion type:
1. 📏 Length
2. ⚖️  Weight/Mass
3. 🌡️  Temperature
4. 🧪 Volume
5. 🚪 Exit

Enter your choice (1-5): 1

📏 Length Conversion
Supported units: m, km, cm, mm, ft, in, yd, mi
Enter the value to convert: 100
From unit: ft
To unit: m
✅ 100 ft = 30.480000 m
```

### Using as a Library

You can use the conversion functions directly in your Rust code in multiple ways:

#### General Conversion Functions (String-based)
```rust
use conversions_rs::*;

// Length conversion
let meters = convert_length(100.0, "ft", "m").unwrap();
println!("{} meters", meters); // 30.48 meters

// Temperature conversion
let fahrenheit = convert_temperature(0.0, "C", "F").unwrap();
println!("{}°F", fahrenheit); // 32°F

// Weight conversion
let pounds = convert_weight(1.0, "kg", "lb").unwrap();
println!("{} lbs", pounds); // 2.20462 lbs

// Volume conversion
let milliliters = convert_volume(1.0, "gal", "ml").unwrap();
println!("{} ml", milliliters); // 3785.41 ml
```

#### Modular API (Type-safe, organized by unit)
```rust
use conversions_rs::conversions::length::*;

// Using the modular API - more organized and discoverable
let feet = meters::to_feet(10.0);           // 32.8084 feet
let inches = feet::to_inches(5.0);          // 60.0 inches
let cm = inches::to_centimeters(12.0);      // 30.48 cm
let km = miles::to_kilometers(5.0);         // 8.0467 km

// Chain conversions easily
let result = meters::to_feet(kilometers::to_meters(1.0)); // 1 km to feet
```

#### Legacy Functions (Backward compatibility)
```rust
use conversions_rs::*;

// Traditional function names still work
let feet = meters_to_feet(10.0);
let kg = pounds_to_kilograms(22.0);
let fahrenheit = celsius_to_fahrenheit(25.0);
let ml = liters_to_milliliters(2.5);
```

### WebAssembly (WASM) Usage 🌐

The library can be compiled to WebAssembly for use in web browsers and JavaScript environments.

#### Prerequisites

First, install `wasm-pack`:
```bash
cargo install wasm-pack
```

#### Building for WebAssembly

Use the provided build scripts to compile for different WASM targets:

**Windows:**
```bash
./build-wasm.bat
```

**Unix/Linux/macOS:**
```bash
./build-wasm.sh
```

This will generate WASM packages in the `pkg/` directory for different targets:
- `pkg/web/` - For direct browser usage
- `pkg/nodejs/` - For Node.js applications  
- `pkg/bundler/` - For webpack, rollup, etc.

#### JavaScript/TypeScript Usage

```javascript
import init, { 
    convert_length_wasm, 
    convert_weight_wasm, 
    convert_temperature_wasm,
    convert_volume_wasm,
    convert_time_wasm,
    convert_area_wasm,
    get_supported_units
} from './pkg/web/conversions_rs.js';

// Initialize the WASM module
await init();

// Perform conversions
const lengthResult = convert_length_wasm(100, "ft", "m");
if (lengthResult.success) {
    console.log(`100 feet = ${lengthResult.value} meters`);
} else {
    console.error("Conversion failed:", lengthResult.error);
}

// Temperature conversion
const tempResult = convert_temperature_wasm(25, "C", "F");
console.log(`25°C = ${tempResult.value}°F`); // 25°C = 77°F

// Get supported units for a conversion type
const lengthUnits = get_supported_units("length");
console.log("Length units:", lengthUnits);
// Output: ["m", "km", "cm", "mm", "ft", "in", "yd", "mi", ...]
```

#### WASM Result Type

All WASM conversion functions return a `ConversionResult` object:

```typescript
interface ConversionResult {
    success: boolean;    // Whether the conversion succeeded
    value: number;       // The converted value (0 if failed)
    error?: string;      // Error message if conversion failed
}
```

#### HTML Demo

A complete HTML demo is provided in `demo.html` that showcases all WASM functionality. 
Open it in a web browser (must be served over HTTP/HTTPS) to try the conversions interactively.

#### Integration Examples

**React/Next.js:**
```jsx
import { useEffect, useState } from 'react';
import init, { convert_length_wasm } from './pkg/web/conversions_rs.js';

function Converter() {
    const [wasmReady, setWasmReady] = useState(false);

    useEffect(() => {
        init().then(() => setWasmReady(true));
    }, []);

    const handleConvert = () => {
        if (!wasmReady) return;
        
        const result = convert_length_wasm(100, "ft", "m");
        if (result.success) {
            console.log("Converted:", result.value);
        }
    };

    return wasmReady ? (
        <button onClick={handleConvert}>Convert 100ft to meters</button>
    ) : (
        <div>Loading WASM...</div>
    );
}
```

**Node.js:**
```javascript
const { convert_length_wasm } = require('./pkg/nodejs/conversions_rs.js');

const result = convert_length_wasm(100, "ft", "m");
console.log(`100 feet = ${result.value} meters`);
```

#### Browser Support

The WASM module supports all modern browsers with WebAssembly support:
- Chrome 57+
- Firefox 52+  
- Safari 11+
- Edge 16+

For older browsers, consider using a WebAssembly polyfill.

## Supported Units

### Length
- `m`, `meter`, `meters` - Meters
- `km`, `kilometer`, `kilometers` - Kilometers
- `cm`, `centimeter`, `centimeters` - Centimeters
- `mm`, `millimeter`, `millimeters` - Millimeters
- `ft`, `foot`, `feet` - Feet
- `in`, `inch`, `inches` - Inches
- `yd`, `yard`, `yards` - Yards
- `mi`, `mile`, `miles` - Miles

### Weight/Mass
- `kg`, `kilogram`, `kilograms` - Kilograms
- `g`, `gram`, `grams` - Grams
- `lb`, `lbs`, `pound`, `pounds` - Pounds
- `oz`, `ounce`, `ounces` - Ounces
- `t`, `ton`, `tons` - Metric Tons
- `st`, `stone`, `stones` - Stones

### Temperature
- `C`, `celsius` - Celsius
- `F`, `fahrenheit` - Fahrenheit
- `K`, `kelvin` - Kelvin

### Volume
- `l`, `liter`, `liters`, `litre`, `litres` - Liters
- `ml`, `milliliter`, `milliliters` - Milliliters
- `gal`, `gallon`, `gal_us` - US Gallons
- `gal_uk`, `gallon_uk` - UK Gallons
- `fl_oz`, `fl_oz_us`, `fluid_ounce` - US Fluid Ounces
- `fl_oz_uk`, `fluid_ounce_uk` - UK Fluid Ounces
- `cup`, `cups`, `cup_us` - US Cups
- `pt`, `pint`, `pints`, `pt_us` - US Pints
- `qt`, `quart`, `quarts`, `qt_us` - US Quarts

## Building

```bash
# Build the project
cargo build

# Build for release (optimized)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check
```

### Installing for System-Wide Use

To use the app from anywhere on your system:

```bash
# Build release version
cargo build --release

# The executable will be at: target/release/conversions_rs.exe (Windows) or target/release/conversions_rs (Unix)

# On Windows, you can add the target/release directory to your PATH
# Or copy conversions_rs.exe to a directory that's already in your PATH

# On Unix/Linux/Mac:
# sudo cp target/release/conversions_rs /usr/local/bin/
```

## Library Usage

You can use Conversions RS as a library in your Rust projects:

```rust
use conversions_rs::conversions::length;

fn main() {
    let meters = length::feet_to_meters(100.0);
    println!("100 feet = {} meters", meters);
    
    let feet = length::meters_to_feet(30.48);
    println!("30.48 meters = {} feet", feet);
}
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

### Development

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes to this project.

---

Made with ❤️ by [Raihau GRAFFE](mailto:graffe.raihau@gmail.com)

Then you can use it directly:
```bash
converions length 100 ft m
converions temperature 25 C F
```

## Testing

The project includes comprehensive unit tests covering:
- All conversion functions
- Error handling for invalid units
- Edge cases and precision
- Same-unit conversions

Run tests with:
```bash
cargo test
```

## Project Structure

```
src/
├── main.rs              # CLI application entry point
├── lib.rs               # Library entry point and tests
└── conversions/
    ├── mod.rs           # Module declarations
    ├── length.rs        # Length conversion functions
    ├── weight.rs        # Weight/mass conversion functions
    ├── temperature.rs   # Temperature conversion functions
    └── volume.rs        # Volume conversion functions
```

## License

This project is open source and available under the MIT License.

## Dependencies

- `clap` - Command-line argument parsing for non-interactive mode