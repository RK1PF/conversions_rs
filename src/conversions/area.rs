//! Area conversion functions organized by unit type
//! All conversions use square meters as the base unit for accuracy and consistency

/// Square meters conversion functions
pub mod square_meters {
    /// Converts square meters to square millimeters.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts square meters to square centimeters.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        value * 10_000.0
    }

    /// Converts square meters to square kilometers.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts square meters to square inches.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 1550.0031
    }

    /// Converts square meters to square feet.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 10.763910417
    }

    /// Converts square meters to square yards.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value * 1.1959900463
    }

    /// Converts square meters to acres.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 4046.8564224
    }

    /// Converts square meters to hectares.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_hectares(value: f64) -> f64 {
        value / 10_000.0
    }

    /// Converts square meters to square miles.
    /// # Arguments
    /// * `value` - The area in square meters to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 2_589_988.110336
    }
}

/// Square millimeters conversion functions
pub mod square_millimeters {
    /// Converts square millimeters to square meters.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts square millimeters to square centimeters.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        value / 100.0
    }

    /// Converts square millimeters to square kilometers.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        super::square_meters::to_square_kilometers(to_square_meters(value))
    }

    /// Converts square millimeters to square inches.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_meters::to_square_inches(to_square_meters(value))
    }

    /// Converts square millimeters to square feet.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_meters::to_square_feet(to_square_meters(value))
    }

    /// Converts square millimeters to square yards.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_meters::to_square_yards(to_square_meters(value))
    }

    /// Converts square millimeters to acres.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_acres(value: f64) -> f64 {
        super::square_meters::to_acres(to_square_meters(value))
    }

    /// Converts square millimeters to hectares.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_meters::to_hectares(to_square_meters(value))
    }

    /// Converts square millimeters to square miles.
    /// # Arguments
    /// * `value` - The area in square millimeters to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_meters::to_square_miles(to_square_meters(value))
    }
}

/// Square centimeters conversion functions
pub mod square_centimeters {
    /// Converts square centimeters to square meters.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value / 10_000.0
    }

    /// Converts square centimeters to square millimeters.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        value * 100.0
    }

    /// Converts square centimeters to square kilometers.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        super::square_meters::to_square_kilometers(to_square_meters(value))
    }

    /// Converts square centimeters to square inches.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 0.15500031
    }

    /// Converts square centimeters to square feet.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_meters::to_square_feet(to_square_meters(value))
    }

    /// Converts square centimeters to square yards.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_meters::to_square_yards(to_square_meters(value))
    }

    /// Converts square centimeters to acres.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_acres(value: f64) -> f64 {
        super::square_meters::to_acres(to_square_meters(value))
    }

    /// Converts square centimeters to hectares.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_meters::to_hectares(to_square_meters(value))
    }

    /// Converts square centimeters to square miles.
    /// # Arguments
    /// * `value` - The area in square centimeters to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_meters::to_square_miles(to_square_meters(value))
    }
}

/// Square kilometers conversion functions
pub mod square_kilometers {
    /// Converts square kilometers to square meters.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts square kilometers to square millimeters.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts square kilometers to square centimeters.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        super::square_meters::to_square_centimeters(to_square_meters(value))
    }

    /// Converts square kilometers to square inches.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_meters::to_square_inches(to_square_meters(value))
    }

    /// Converts square kilometers to square feet.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_meters::to_square_feet(to_square_meters(value))
    }

    /// Converts square kilometers to square yards.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_meters::to_square_yards(to_square_meters(value))
    }

    /// Converts square kilometers to acres.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_acres(value: f64) -> f64 {
        value * 247.10538147
    }

    /// Converts square kilometers to hectares.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_hectares(value: f64) -> f64 {
        value * 100.0
    }

    /// Converts square kilometers to square miles.
    /// # Arguments
    /// * `value` - The area in square kilometers to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value * 0.3861021585
    }
}

/// Square inches conversion functions
pub mod square_inches {
    /// Converts square inches to square meters.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value / 1550.0031
    }

    /// Converts square inches to square millimeters.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts square inches to square centimeters.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        value / 0.15500031
    }

    /// Converts square inches to square kilometers.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        super::square_meters::to_square_kilometers(to_square_meters(value))
    }

    /// Converts square inches to square feet.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value / 144.0
    }

    /// Converts square inches to square yards.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value / 1296.0
    }

    /// Converts square inches to acres.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_acres(value: f64) -> f64 {
        super::square_meters::to_acres(to_square_meters(value))
    }

    /// Converts square inches to hectares.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_meters::to_hectares(to_square_meters(value))
    }

    /// Converts square inches to square miles.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_meters::to_square_miles(to_square_meters(value))
    }
}

/// Square feet conversion functions
pub mod square_feet {
    /// Converts square feet to square meters.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value / 10.763910417
    }

    /// Converts square feet to square millimeters.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts square feet to square centimeters.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        super::square_meters::to_square_centimeters(to_square_meters(value))
    }

    /// Converts square feet to square kilometers.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        super::square_meters::to_square_kilometers(to_square_meters(value))
    }

    /// Converts square feet to square inches.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 144.0
    }

    /// Converts square feet to square yards.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value / 9.0
    }

    /// Converts square feet to acres.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 43560.0
    }

    /// Converts square feet to hectares.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_meters::to_hectares(to_square_meters(value))
    }

    /// Converts square feet to square miles.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 27878400.0
    }
}

/// Square yards conversion functions
pub mod square_yards {
    /// Converts square yards to square meters.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value / 1.1959900463
    }

    /// Converts square yards to square millimeters.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts square yards to square centimeters.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        super::square_meters::to_square_centimeters(to_square_meters(value))
    }

    /// Converts square yards to square kilometers.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        super::square_meters::to_square_kilometers(to_square_meters(value))
    }

    /// Converts square yards to square inches.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 1296.0
    }

    /// Converts square yards to square feet.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 9.0
    }

    /// Converts square yards to acres.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 4840.0
    }

    /// Converts square yards to hectares.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_meters::to_hectares(to_square_meters(value))
    }

    /// Converts square yards to square miles.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 3097600.0
    }
}

/// Acres conversion functions
pub mod acres {
    /// Converts acres to square meters.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value * 4046.8564224
    }

    /// Converts acres to square millimeters.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts acres to square centimeters.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        super::square_meters::to_square_centimeters(to_square_meters(value))
    }

    /// Converts acres to square kilometers.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        value / 247.10538147
    }

    /// Converts acres to square inches.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_meters::to_square_inches(to_square_meters(value))
    }

    /// Converts acres to square feet.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 43560.0
    }

    /// Converts acres to square yards.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value * 4840.0
    }

    /// Converts acres to hectares.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_hectares(value: f64) -> f64 {
        value * 0.40468564224
    }

    /// Converts acres to square miles.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 640.0
    }
}

/// Hectares conversion functions
pub mod hectares {
    /// Converts hectares to square meters.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value * 10_000.0
    }

    /// Converts hectares to square millimeters.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts hectares to square centimeters.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        super::square_meters::to_square_centimeters(to_square_meters(value))
    }

    /// Converts hectares to square kilometers.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        value / 100.0
    }

    /// Converts hectares to square inches.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_meters::to_square_inches(to_square_meters(value))
    }

    /// Converts hectares to square feet.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_meters::to_square_feet(to_square_meters(value))
    }

    /// Converts hectares to square yards.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_meters::to_square_yards(to_square_meters(value))
    }

    /// Converts hectares to acres.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 0.40468564224
    }

    /// Converts hectares to square miles.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_meters::to_square_miles(to_square_meters(value))
    }
}

/// Square miles conversion functions
pub mod square_miles {
    /// Converts square miles to square meters.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_meters(value: f64) -> f64 {
        value * 2_589_988.110336
    }

    /// Converts square miles to square millimeters.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_millimeters(value: f64) -> f64 {
        super::square_meters::to_square_millimeters(to_square_meters(value))
    }

    /// Converts square miles to square centimeters.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_centimeters(value: f64) -> f64 {
        super::square_meters::to_square_centimeters(to_square_meters(value))
    }

    /// Converts square miles to square kilometers.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_kilometers(value: f64) -> f64 {
        value / 0.3861021585
    }

    /// Converts square miles to square inches.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_meters::to_square_inches(to_square_meters(value))
    }

    /// Converts square miles to square feet.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 27878400.0
    }

    /// Converts square miles to square yards.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value * 3097600.0
    }

    /// Converts square miles to acres.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_acres(value: f64) -> f64 {
        value * 640.0
    }

    /// Converts square miles to hectares.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_meters::to_hectares(to_square_meters(value))
    }
}

// Legacy function wrappers for backward compatibility
pub fn square_meters_to_square_millimeters(square_meters: f64) -> f64 {
    square_meters::to_square_millimeters(square_meters)
}

pub fn square_millimeters_to_square_meters(square_millimeters: f64) -> f64 {
    square_millimeters::to_square_meters(square_millimeters)
}

pub fn square_meters_to_square_centimeters(square_meters: f64) -> f64 {
    square_meters::to_square_centimeters(square_meters)
}

pub fn square_centimeters_to_square_meters(square_centimeters: f64) -> f64 {
    square_centimeters::to_square_meters(square_centimeters)
}

pub fn square_meters_to_square_kilometers(square_meters: f64) -> f64 {
    square_meters::to_square_kilometers(square_meters)
}

pub fn square_kilometers_to_square_meters(square_kilometers: f64) -> f64 {
    square_kilometers::to_square_meters(square_kilometers)
}

pub fn square_meters_to_square_feet(square_meters: f64) -> f64 {
    square_meters::to_square_feet(square_meters)
}

pub fn square_feet_to_square_meters(square_feet: f64) -> f64 {
    square_feet::to_square_meters(square_feet)
}

pub fn square_meters_to_acres(square_meters: f64) -> f64 {
    square_meters::to_acres(square_meters)
}

pub fn acres_to_square_meters(acres: f64) -> f64 {
    acres::to_square_meters(acres)
}

/// General area conversion function that accepts string unit names
/// 
/// Converts an area value from one unit to another using string identifiers.
/// This function is case-insensitive and supports common abbreviations.
///
/// # Arguments
/// 
/// * `value` - The numeric value to convert
/// * `from_unit` - The source unit (e.g., "m²", "cm²", "km²", "ft²", "in²", "ac", "ha")
/// * `to_unit` - The target unit using the same abbreviations
///
/// # Returns
/// 
/// * `Ok(f64)` - The converted value
/// * `Err(String)` - Error message if the conversion is not supported
///
/// # Examples
/// 
/// ```rust
/// use conversions_rs::convert_area;
/// 
/// let square_centimeters = convert_area(2.5, "m²", "cm²").unwrap();
/// assert_eq!(square_centimeters, 25000.0);
/// 
/// let acres = convert_area(10000.0, "m²", "ac").unwrap();
/// assert!((acres - 2.471).abs() < 0.001);
/// ```
pub fn convert_area(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();
    
    // Convert input to square meters first
    let square_meters = match from_unit.as_str() {
        "m²" | "m2" | "sq_m" | "square_meters" => value,
        "mm²" | "mm2" | "sq_mm" | "square_millimeters" => square_millimeters::to_square_meters(value),
        "cm²" | "cm2" | "sq_cm" | "square_centimeters" => square_centimeters::to_square_meters(value),
        "km²" | "km2" | "sq_km" | "square_kilometers" => square_kilometers::to_square_meters(value),
        "in²" | "in2" | "sq_in" | "square_inches" => square_inches::to_square_meters(value),
        "ft²" | "ft2" | "sq_ft" | "square_feet" => square_feet::to_square_meters(value),
        "yd²" | "yd2" | "sq_yd" | "square_yards" => square_yards::to_square_meters(value),
        "ac" | "acre" | "acres" => acres::to_square_meters(value),
        "ha" | "hectare" | "hectares" => hectares::to_square_meters(value),
        "mi²" | "mi2" | "sq_mi" | "square_miles" => square_miles::to_square_meters(value),
        _ => return Err(format!("Unsupported area unit: {}", from_unit)),
    };
    
    // Convert square meters to target unit
    let result = match to_unit.as_str() {
        "m²" | "m2" | "sq_m" | "square_meters" => square_meters,
        "mm²" | "mm2" | "sq_mm" | "square_millimeters" => square_meters::to_square_millimeters(square_meters),
        "cm²" | "cm2" | "sq_cm" | "square_centimeters" => square_meters::to_square_centimeters(square_meters),
        "km²" | "km2" | "sq_km" | "square_kilometers" => square_meters::to_square_kilometers(square_meters),
        "in²" | "in2" | "sq_in" | "square_inches" => square_meters::to_square_inches(square_meters),
        "ft²" | "ft2" | "sq_ft" | "square_feet" => square_meters::to_square_feet(square_meters),
        "yd²" | "yd2" | "sq_yd" | "square_yards" => square_meters::to_square_yards(square_meters),
        "ac" | "acre" | "acres" => square_meters::to_acres(square_meters),
        "ha" | "hectare" | "hectares" => square_meters::to_hectares(square_meters),
        "mi²" | "mi2" | "sq_mi" | "square_miles" => square_meters::to_square_miles(square_meters),
        _ => return Err(format!("Unsupported area unit: {}", to_unit)),
    };
    
    Ok(result)
}
