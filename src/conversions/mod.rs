//! Unit conversion modules for the conversions_rs library.
//! 
//! This module provides comprehensive unit conversion functionality across
//! SI base units and derived units including: length, weight/mass, temperature,
//! volume, time, electric current, amount of substance, luminous intensity, and area.
//! 
//! # Usage
//! 
//! Each conversion category has both specific conversion functions and a
//! general conversion function that accepts string unit names:
//! 
//! ```rust
//! use conversions_rs::*;
//! 
//! // Using specific functions
//! let feet = meters_to_feet(10.0);
//! let pounds = kilograms_to_pounds(5.0);
//! 
//! // Using general conversion functions
//! let result = convert_length(100.0, "ft", "m").unwrap();
//! let result = convert_weight(10.0, "kg", "lb").unwrap();
//! let result = convert_temperature(32.0, "F", "C").unwrap();
//! let result = convert_volume(1.0, "gal", "l").unwrap();
//! let result = convert_time(60.0, "min", "s").unwrap();
//! let result = convert_current(1000.0, "mA", "A").unwrap();
//! let result = convert_amount(0.5, "mol", "mmol").unwrap();
//! let result = convert_luminous_intensity(2.5, "cd", "mcd").unwrap();
//! let result = convert_area(10000.0, "m²", "ha").unwrap();
//! ```
//! 
//! # Modules
//! 
//! ## SI Base Units
//! * [`length`] - Length and distance conversions (meters, feet, inches, etc.)
//! * [`weight`] - Weight and mass conversions (kilograms, pounds, ounces, etc.)
//! * [`temperature`] - Temperature conversions (Celsius, Fahrenheit, Kelvin)
//! * [`time`] - Time conversions (seconds, minutes, hours, days, etc.)
//! * [`current`] - Electric current conversions (amperes and SI multiples)
//! * [`substance`] - Amount of substance conversions (moles and SI multiples)
//! * [`luminous_intensity`] - Luminous intensity conversions (candela and related units)
//! 
//! ## SI Derived Units
//! * [`volume`] - Volume conversions (liters, gallons, cups, etc.)
//! * [`area`] - Area conversions (square meters, acres, hectares, etc.)

pub mod length;
pub mod weight;
pub mod temperature;
pub mod volume;
pub mod time;
pub mod current;
pub mod substance;
pub mod luminous_intensity;
pub mod area;

pub use length::*;
pub use weight::*;
pub use temperature::*;
pub use volume::*;
pub use time::*;
pub use current::*;
pub use substance::*;
pub use luminous_intensity::*;
pub use area::*;
