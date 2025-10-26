//! # Conversions RS - A Comprehensive Unit Conversion Library
//!
//! `conversions_rs` is a Rust library and command-line tool for converting between
//! different units of measurement. It supports comprehensive SI (International
//! System of Units) base and derived units including: length, weight/mass,
//! temperature, volume, time, electric current, amount of substance, luminous
//! intensity, and area with high precision and extensive unit support.
//!
//! ## Features
//!
//! ### SI Base Units
//! - **Length conversions**: meters, kilometers, feet, inches, yards, miles, etc.
//! - **Mass conversions**: kilograms, grams, pounds, ounces, tons, stones
//! - **Temperature conversions**: Celsius, Fahrenheit, Kelvin
//! - **Time conversions**: seconds, minutes, hours, days, weeks, years, etc.
//! - **Electric current conversions**: amperes and SI prefixes (mA, µA, kA, etc.)
//! - **Amount of substance conversions**: moles and SI prefixes (mmol, µmol, etc.)
//! - **Luminous intensity conversions**: candela and related historical units
//!
//! ### SI Derived Units
//! - **Volume conversions**: liters, gallons (US/UK), cups, pints, quarts, etc.
//! - **Area conversions**: square meters, acres, hectares, square feet, etc.
//!
//! ### Additional Features
//! - **Case-insensitive unit names** with support for singular/plural forms
//! - **High precision** calculations using standard conversion factors
//! - **Comprehensive error handling** for invalid units
//! - **Modular API** organized by unit type for better discoverability
//!
//! ## Usage Patterns
//!
//! ### 1. Modular API (Recommended - Type-safe & Discoverable)
//!
//! The modular API organizes conversions by unit type, making it easy to discover
//! available conversions with IDE autocomplete:
//!
//! ```rust
//! use conversions_rs::conversions::*;
//!
//! // Length conversions - organized by source unit
//! let feet = length::meters::to_feet(10.0);           // 32.8084 feet
//! let inches = length::feet::to_inches(5.0);          // 60.0 inches  
//! let cm = length::inches::to_centimeters(12.0);      // 30.48 cm
//! let km = length::miles::to_kilometers(5.0);         // 8.0467 km
//!
//! // Weight conversions
//! let pounds = weight::kilograms::to_pounds(5.0);     // 11.0231 pounds
//! let grams = weight::pounds::to_grams(1.0);          // 453.592 grams
//!
//! // Temperature conversions
//! let fahrenheit = temperature::celsius::to_fahrenheit(25.0);  // 77.0°F
//! let kelvin = temperature::celsius::to_kelvin(0.0);           // 273.15K
//!
//! // Volume conversions
//! let gallons = volume::liters::to_gallons_us(10.0);          // 2.64 gallons
//! let milliliters = volume::gallons_us::to_milliliters(1.0);  // 3785.41 ml
//!
//! // Time conversions (NEW SI BASE UNIT)
//! let minutes = time::seconds::to_minutes(3600.0);            // 60.0 minutes
//! let hours = time::minutes::to_hours(120.0);                 // 2.0 hours
//! let milliseconds = time::seconds::to_milliseconds(1.5);     // 1500.0 ms
//!
//! // Electric current conversions (NEW SI BASE UNIT)
//! let milliamperes = current::amperes::to_milliamperes(1.5);  // 1500.0 mA
//! let amperes = current::milliamperes::to_amperes(500.0);     // 0.5 A
//! let microamperes = current::amperes::to_microamperes(0.001); // 1000.0 μA
//!
//! // Amount of substance conversions (NEW SI BASE UNIT)
//! let millimoles = substance::moles::to_millimoles(0.5);      // 500.0 mmol
//! let moles = substance::millimoles::to_moles(250.0);         // 0.25 mol
//! let micromoles = substance::moles::to_micromoles(0.001);    // 1000.0 μmol
//!
//! // Luminous intensity conversions (NEW SI BASE UNIT)
//! let millicandela = luminous_intensity::candela::to_millicandela(2.5);  // 2500.0 mcd
//! let candela = luminous_intensity::millicandela::to_candela(1500.0);    // 1.5 cd
//! let hefnerkerze = luminous_intensity::candela::to_hefnerkerze(1.0);    // 1.11 hk
//!
//! // Area conversions (NEW SI DERIVED UNIT)
//! let hectares = area::square_meters::to_hectares(10000.0);   // 1.0 ha
//! let acres = area::hectares::to_acres(1.0);                  // 2.471 acres
//! let sq_feet = area::square_meters::to_square_feet(1.0);     // 10.764 ft²
//!
//! // Easy to chain conversions
//! let result = length::meters::to_feet(length::kilometers::to_meters(1.0)); // 1 km to feet
//! ```
//!
//! ### 2. General Conversion Functions (String-based)
//!
//! Best for user input and flexible conversions:
//!
//! ```rust
//! use conversions_rs::*;
//!
//! // Length conversions
//! let meters = convert_length(100.0, "ft", "m").unwrap();
//! println!("100 feet = {:.2} meters", meters);
//!
//! // Weight conversions
//! let pounds = convert_weight(5.0, "kg", "lb").unwrap();
//! println!("5 kg = {:.2} pounds", pounds);
//!
//! // Temperature conversions  
//! let celsius = convert_temperature(32.0, "F", "C").unwrap();
//! println!("32°F = {:.1}°C", celsius);
//!
//! // Volume conversions
//! let liters = convert_volume(1.0, "gal", "l").unwrap();
//! println!("1 gallon = {:.2} liters", liters);
//!
//! // Time conversions
//! let seconds = convert_time(5.0, "min", "s").unwrap();
//! println!("5 minutes = {:.0} seconds", seconds);
//!
//! // Electric current conversions
//! let amperes = convert_current(1500.0, "mA", "A").unwrap();
//! println!("1500 mA = {:.2} A", amperes);
//!
//! // Amount of substance conversions
//! let millimoles = convert_amount(0.5, "mol", "mmol").unwrap();
//! println!("0.5 mol = {:.0} mmol", millimoles);
//!
//! // Luminous intensity conversions
//! let millicandela = convert_luminous_intensity(2.5, "cd", "mcd").unwrap();
//! println!("2.5 cd = {:.0} mcd", millicandela);
//!
//! // Area conversions
//! let hectares = convert_area(10000.0, "m²", "ha").unwrap();
//! println!("10000 m² = {:.1} ha", hectares);
//! ```
//!
//! ### 3. Legacy Functions (Backward compatible)
//!
//! ```rust
//! use conversions_rs::*;
//!
//! let feet = meters_to_feet(10.0);
//! let kg = pounds_to_kilograms(22.0);
//! let fahrenheit = celsius_to_fahrenheit(25.0);
//! ```
//!
//! ## Error Handling
//!
//! The general conversion functions return `Result<f64, String>` for proper
//! error handling:
//!
//! ```rust
//! use conversions_rs::convert_length;
//!
//! match convert_length(100.0, "ft", "invalid_unit") {
//!     Ok(result) => println!("Converted: {}", result),
//!     Err(error) => eprintln!("Conversion error: {}", error),
//! }
//! ```
//!
//! ## Command-Line Usage
//!
//! This library can also be used as a command-line tool with support for all SI units:
//!
//! ```bash
//! # SI Base Units
//! conversions_rs length 100 ft m              # Length conversions
//! conversions_rs weight 5 kg lb               # Mass conversions  
//! conversions_rs temperature 32 F C           # Temperature conversions
//! conversions_rs time 3600 s min              # Time conversions
//! conversions_rs current 1500 mA A            # Electric current conversions
//! conversions_rs amount 0.5 mol mmol          # Amount of substance conversions
//! conversions_rs luminosity 2.5 cd mcd        # Luminous intensity conversions
//!
//! # SI Derived Units
//! conversions_rs volume 5 gal l               # Volume conversions
//! conversions_rs area 10000 "m²" ha           # Area conversions
//!
//! # Interactive mode with full menu
//! conversions_rs
//! ```

pub mod conversions;

pub use conversions::*;

#[cfg(test)]
mod tests {
    use crate::conversions::*;

    #[test]
    fn test_length_conversions() {
        // Test meter to feet
        assert!((convert_length(1.0, "m", "ft").unwrap() - 3.28084).abs() < 0.0001);

        // Test kilometer to meter
        assert_eq!(convert_length(1.0, "km", "m").unwrap(), 1000.0);

        // Test inch to centimeter
        assert!((convert_length(1.0, "in", "cm").unwrap() - 2.54).abs() < 0.01);

        // Test mile to kilometer
        assert!((convert_length(1.0, "mi", "km").unwrap() - 1.60934).abs() < 0.001);
    }

    #[test]
    fn test_weight_conversions() {
        // Test kilogram to pound
        assert!((convert_weight(1.0, "kg", "lb").unwrap() - 2.20462).abs() < 0.0001);

        // Test gram to kilogram
        assert_eq!(convert_weight(1000.0, "g", "kg").unwrap(), 1.0);

        // Test pound to ounces
        assert!((convert_weight(1.0, "lb", "oz").unwrap() - 16.0).abs() < 0.01);
    }

    #[test]
    fn test_temperature_conversions() {
        // Test Celsius to Fahrenheit
        assert_eq!(convert_temperature(0.0, "C", "F").unwrap(), 32.0);
        assert_eq!(convert_temperature(100.0, "C", "F").unwrap(), 212.0);

        // Test Fahrenheit to Celsius
        assert_eq!(convert_temperature(32.0, "F", "C").unwrap(), 0.0);
        assert_eq!(convert_temperature(212.0, "F", "C").unwrap(), 100.0);

        // Test Celsius to Kelvin
        assert_eq!(convert_temperature(0.0, "C", "K").unwrap(), 273.15);

        // Test Kelvin to Celsius
        assert_eq!(convert_temperature(273.15, "K", "C").unwrap(), 0.0);
    }

    #[test]
    fn test_volume_conversions() {
        // Test liter to US gallon
        assert!((convert_volume(1.0, "l", "gal").unwrap() - 0.264172).abs() < 0.0001);

        // Test US gallon to liter
        assert!((convert_volume(1.0, "gal", "l").unwrap() - 3.78541).abs() < 0.0001);

        // Test cup to milliliter (US)
        assert!((convert_volume(1.0, "cup", "ml").unwrap() - 236.588).abs() < 0.1);
    }

    #[test]
    fn test_modular_length_api() {
        use crate::conversions::length::*;

        // Test meters module
        assert!((meters::to_feet(1.0) - 3.28084).abs() < 0.0001);
        assert!((meters::to_inches(1.0) - 39.3701).abs() < 0.001);

        // Test feet module
        assert!((feet::to_meters(1.0) - 0.3048).abs() < 0.0001);
        assert_eq!(feet::to_inches(1.0), 12.0);

        // Test chaining conversions
        let km_to_feet = meters::to_feet(kilometers::to_meters(1.0));
        assert!((km_to_feet - 3280.84).abs() < 0.1);
    }

    #[test]
    fn test_modular_volume_api() {
        use crate::conversions::volume::*;

        // Test liters module
        assert!((liters::to_gallons_us(1.0) - 0.264172).abs() < 0.0001);
        assert_eq!(liters::to_milliliters(1.0), 1000.0);

        // Test gallons_us module
        assert!((gallons_us::to_liters(1.0) - 3.78541).abs() < 0.0001);
        assert_eq!(gallons_us::to_fluid_ounces_us(1.0), 128.0);
    }

    #[test]
    fn test_time_conversions() {
        // Test seconds to minutes
        assert_eq!(convert_time(60.0, "s", "min").unwrap(), 1.0);

        // Test minutes to hours
        assert_eq!(convert_time(120.0, "min", "h").unwrap(), 2.0);

        // Test hours to days
        assert_eq!(convert_time(48.0, "h", "day").unwrap(), 2.0);

        // Test milliseconds to seconds
        assert_eq!(convert_time(1000.0, "ms", "s").unwrap(), 1.0);
    }

    #[test]
    fn test_current_conversions() {
        // Test amperes to milliamperes
        assert_eq!(convert_current(1.0, "A", "mA").unwrap(), 1000.0);

        // Test milliamperes to amperes
        assert_eq!(convert_current(500.0, "mA", "A").unwrap(), 0.5);

        // Test amperes to microamperes
        assert_eq!(convert_current(1.0, "A", "μA").unwrap(), 1_000_000.0);

        // Test kiloamperes to amperes
        assert_eq!(convert_current(1.0, "kA", "A").unwrap(), 1000.0);
    }

    #[test]
    fn test_amount_conversions() {
        // Test moles to millimoles
        assert_eq!(convert_amount(1.0, "mol", "mmol").unwrap(), 1000.0);

        // Test millimoles to moles
        assert_eq!(convert_amount(500.0, "mmol", "mol").unwrap(), 0.5);

        // Test moles to micromoles
        assert_eq!(convert_amount(1.0, "mol", "μmol").unwrap(), 1_000_000.0);

        // Test kilomoles to moles
        assert_eq!(convert_amount(1.0, "kmol", "mol").unwrap(), 1000.0);
    }

    #[test]
    fn test_luminous_intensity_conversions() {
        // Test candela to millicandela
        assert_eq!(
            convert_luminous_intensity(1.0, "cd", "mcd").unwrap(),
            1000.0
        );

        // Test millicandela to candela
        assert_eq!(convert_luminous_intensity(500.0, "mcd", "cd").unwrap(), 0.5);

        // Test candela to kilocandela
        assert_eq!(
            convert_luminous_intensity(1000.0, "cd", "kcd").unwrap(),
            1.0
        );

        // Test decimal candle to candela (should be equal)
        assert_eq!(convert_luminous_intensity(1.0, "dc", "cd").unwrap(), 1.0);
    }

    #[test]
    fn test_area_conversions() {
        // Test square meters to square centimeters
        assert_eq!(convert_area(1.0, "m²", "cm²").unwrap(), 10_000.0);

        // Test square kilometers to square meters
        assert_eq!(convert_area(1.0, "km²", "m²").unwrap(), 1_000_000.0);

        // Test square meters to hectares
        assert_eq!(convert_area(10_000.0, "m²", "ha").unwrap(), 1.0);

        // Test acres to square meters
        assert!((convert_area(1.0, "ac", "m²").unwrap() - 4046.8564224).abs() < 0.001);

        // Test square feet to square inches (with floating-point tolerance)
        assert!((convert_area(1.0, "ft²", "in²").unwrap() - 144.0).abs() < 0.001);
    }

    #[test]
    fn test_same_unit_conversions() {
        // Test that converting to the same unit returns the original value
        assert_eq!(convert_length(5.0, "m", "m").unwrap(), 5.0);
        assert_eq!(convert_weight(10.0, "kg", "kg").unwrap(), 10.0);
        assert_eq!(convert_temperature(25.0, "C", "C").unwrap(), 25.0);
        assert_eq!(convert_volume(3.0, "l", "l").unwrap(), 3.0);
        assert_eq!(convert_time(60.0, "s", "s").unwrap(), 60.0);
        assert_eq!(convert_current(1.5, "A", "A").unwrap(), 1.5);
        assert_eq!(convert_amount(0.5, "mol", "mol").unwrap(), 0.5);
        assert_eq!(convert_luminous_intensity(2.0, "cd", "cd").unwrap(), 2.0);
        assert_eq!(convert_area(100.0, "m²", "m²").unwrap(), 100.0);
    }

    #[test]
    fn test_invalid_units() {
        // Test error handling for invalid units
        assert!(convert_length(100.0, "invalid", "m").is_err());
        assert!(convert_weight(50.0, "kg", "invalid").is_err());
        assert!(convert_temperature(25.0, "C", "invalid").is_err());
        assert!(convert_volume(10.0, "invalid", "l").is_err());
        assert!(convert_time(60.0, "invalid", "s").is_err());
        assert!(convert_current(1.0, "A", "invalid").is_err());
        assert!(convert_amount(0.5, "mol", "invalid").is_err());
        assert!(convert_luminous_intensity(1.0, "cd", "invalid").is_err());
        assert!(convert_area(100.0, "m²", "invalid").is_err());
    }
}
