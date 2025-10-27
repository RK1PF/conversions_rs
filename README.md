# Conversions RS - Comprehensive SI Unit Conversion Library 🔄

[![CI](https://github.com/RK1PF/conversions_rs/workflows/CI/badge.svg)](https://github.com/RK1PF/conversions_rs/actions)
[![Crates.io](https://img.shields.io/crates/v/conversions_rs.svg)](https://crates.io/crates/conversions_rs)
[![Documentation](https://docs.rs/conversions_rs/badge.svg)](https://docs.rs/conversions_rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/RK1PF/conversions_rs#license)
[![Downloads](https://img.shields.io/crates/d/conversions_rs.svg)](https://crates.io/crates/conversions_rs)
[![npm version](https://img.shields.io/npm/v/conversions_rs.svg)](https://www.npmjs.com/package/conversions_rs)
[![npm downloads](https://img.shields.io/npm/dm/conversions_rs.svg)](https://www.npmjs.com/package/conversions_rs)
[![Live Demo](https://img.shields.io/badge/demo-live-brightgreen.svg)](https://rk1pf.github.io/conversions_rs/)

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
conversions_rs = "1.2.1"
```

### For JavaScript/TypeScript (WebAssembly)

Install from npm for use in web browsers or Node.js:

```bash
npm install conversions_rs
```

Or download WASM packages directly from the [releases page](https://github.com/RK1PF/conversions_rs/releases):
- `conversions_rs-wasm-web.tar.gz` - For web browsers
- `conversions_rs-wasm-nodejs.tar.gz` - For Node.js applications  
- `conversions_rs-wasm-bundler.tar.gz` - For bundlers (webpack, rollup, etc.)

## Features

✨ **Multi-Platform Support**: Command-line tool, Rust library, and WebAssembly module for web browsers

### SI Base Units
- **Length Conversions**: meters, kilometers, centimeters, millimeters, feet, inches, yards, miles
- **Mass Conversions**: kilograms, grams, pounds, ounces, tons (metric), stones  
- **Temperature Conversions**: Celsius, Fahrenheit, Kelvin
- **Time Conversions**: seconds, minutes, hours, days, weeks, months, years, milliseconds, microseconds, nanoseconds
- **Electric Current Conversions**: amperes, milliamperes, microamperes, nanoamperes, kiloamperes, megaamperes
- **Amount of Substance Conversions**: moles, millimoles, micromoles, nanomoles, kilomoles
- **Luminous Intensity Conversions**: candela, millicandela, kilocandela

### SI Derived Units
- **Volume Conversions**: liters, milliliters, gallons (US/UK), fluid ounces (US/UK), cups, pints, quarts, tablespoons, teaspoons
- **Area Conversions**: square meters, square centimeters, square kilometers, square feet, square inches, acres, hectares, square yards, square miles

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
conversions_rs substance 0.5 mol mmol   # 0.5 moles to millimoles
conversions_rs luminous_intensity 2.5 cd mcd  # 2.5 candela to millicandela

# SI Derived Units  
conversions_rs volume 1 gal l           # 1 gallon to liters
conversions_rs area 10000 "m²" ha       # 10000 square meters to hectares
```

**Get help:**
```bash
conversions_rs --help                        # General help
conversions_rs length --help                 # Help for length conversions
conversions_rs weight --help                 # Help for weight conversions
conversions_rs temperature --help            # Help for temperature conversions
conversions_rs volume --help                 # Help for volume conversions
conversions_rs time --help                   # Help for time conversions
conversions_rs current --help                # Help for current conversions
conversions_rs substance --help              # Help for substance conversions
conversions_rs luminous_intensity --help     # Help for luminous intensity conversions
conversions_rs area --help                   # Help for area conversions
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
5. ⏰ Time
6. ⚡ Electric Current
7. 🧬 Amount of Substance
8. 💡 Luminous Intensity
9. 📐 Area
10. 🚪 Exit

Enter your choice (1-10): 1

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

// Time conversion
let minutes = convert_time(3600.0, "s", "min").unwrap();
println!("{} minutes", minutes); // 60.0 minutes

// Electric current conversion
let amperes = convert_current(1500.0, "mA", "A").unwrap();
println!("{} A", amperes); // 1.5 A

// Amount of substance conversion
let millimoles = convert_amount(0.5, "mol", "mmol").unwrap();
println!("{} mmol", millimoles); // 500.0 mmol

// Luminous intensity conversion
let millicandela = convert_luminous_intensity(2.5, "cd", "mcd").unwrap();
println!("{} mcd", millicandela); // 2500.0 mcd

// Area conversion
let hectares = convert_area(10000.0, "m²", "ha").unwrap();
println!("{} ha", hectares); // 1.0 ha
```

#### Modular API (Type-safe, organized by unit)
```rust
use conversions_rs::{length, weight, temperature, volume, time, current, substance, luminous_intensity, area};

// Using the modular API - more organized and discoverable
let feet = length::meters::to_feet(10.0);           // 32.8084 feet
let inches = length::feet::to_inches(5.0);          // 60.0 inches
let cm = length::inches::to_centimeters(12.0);      // 30.48 cm
let km = length::miles::to_kilometers(5.0);         // 8.0467 km

// Weight/Mass conversions
let pounds = weight::kilograms::to_pounds(10.0);    // 22.0462 pounds
let grams = weight::pounds::to_grams(1.0);          // 453.592 grams

// Temperature conversions
let celsius = temperature::fahrenheit::to_celsius(100.0);  // 37.7778 celsius
let kelvin = temperature::celsius::to_kelvin(25.0);        // 298.15 kelvin

// Volume conversions
let liters = volume::gallons::to_liters(1.0);       // 3.78541 liters
let ml = volume::cups::to_milliliters(2.0);         // 473.176 ml

// Time conversions
let seconds = time::minutes::to_seconds(5.0);       // 300.0 seconds
let hours = time::seconds::to_hours(7200.0);        // 2.0 hours

// Chain conversions easily
let result = length::meters::to_feet(length::kilometers::to_meters(1.0)); // 1 km to feet
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

#### Installation Options

**Option 1: Install from npm (Recommended)**
```bash
npm install conversions_rs
```

**Option 2: Download from GitHub Releases**
Download the appropriate WASM package from the [releases page](https://github.com/RK1PF/conversions_rs/releases):
- `conversions_rs-wasm-web.tar.gz` - For web browsers
- `conversions_rs-wasm-nodejs.tar.gz` - For Node.js applications  
- `conversions_rs-wasm-bundler.tar.gz` - For bundlers (webpack, rollup, etc.)

**Option 3: Build from source**

First, install `wasm-pack`:
```bash
cargo install wasm-pack
```

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

**Using npm package:**
```javascript
import init, { 
    convert_length_wasm, 
    convert_weight_wasm, 
    convert_temperature_wasm,
    convert_volume_wasm,
    convert_time_wasm,
    convert_area_wasm,
    get_supported_units
} from 'conversions_rs';

// Initialize the WASM module
await init();
```

**Using local build:**
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
```

**Example usage:**
```javascript
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
import init, { convert_length_wasm } from 'conversions_rs';

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
- `tbsp`, `tablespoon`, `tablespoons` - Tablespoons
- `tsp`, `teaspoon`, `teaspoons` - Teaspoons

### Time
- `s`, `second`, `seconds` - Seconds
- `min`, `minute`, `minutes` - Minutes
- `h`, `hour`, `hours` - Hours
- `d`, `day`, `days` - Days
- `week`, `weeks` - Weeks
- `month`, `months` - Months
- `year`, `years` - Years
- `ms`, `millisecond`, `milliseconds` - Milliseconds
- `μs`, `microsecond`, `microseconds` - Microseconds
- `ns`, `nanosecond`, `nanoseconds` - Nanoseconds

### Electric Current
- `A`, `ampere`, `amperes` - Amperes
- `mA`, `milliampere`, `milliamperes` - Milliamperes
- `μA`, `microampere`, `microamperes` - Microamperes
- `nA`, `nanoampere`, `nanoamperes` - Nanoamperes
- `kA`, `kiloampere`, `kiloamperes` - Kiloamperes
- `MA`, `megaampere`, `megaamperes` - Megaamperes

### Amount of Substance
- `mol`, `mole`, `moles` - Moles
- `mmol`, `millimole`, `millimoles` - Millimoles
- `μmol`, `micromole`, `micromoles` - Micromoles
- `nmol`, `nanomole`, `nanomoles` - Nanomoles
- `kmol`, `kilomole`, `kilomoles` - Kilomoles

### Luminous Intensity
- `cd`, `candela` - Candela
- `mcd`, `millicandela` - Millicandela
- `kcd`, `kilocandela` - Kilocandela

### Area
- `m²`, `square_meter`, `square_meters` - Square Meters
- `km²`, `square_kilometer`, `square_kilometers` - Square Kilometers
- `cm²`, `square_centimeter`, `square_centimeters` - Square Centimeters
- `mm²`, `square_millimeter`, `square_millimeters` - Square Millimeters
- `ha`, `hectare`, `hectares` - Hectares
- `acre`, `acres` - Acres
- `ft²`, `square_foot`, `square_feet` - Square Feet
- `in²`, `square_inch`, `square_inches` - Square Inches
- `yd²`, `square_yard`, `square_yards` - Square Yards
- `mi²`, `square_mile`, `square_miles` - Square Miles

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