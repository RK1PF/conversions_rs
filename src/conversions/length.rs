//! Length conversion functions organized by unit type
//! All conversions use meters as the base unit for accuracy and consistency

/// Meter conversion functions
pub mod meters {
    /// Converts meters to feet.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_feet(value: f64) -> f64 {
        value * 3.28084
    }

    /// Converts meters to inches.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 39.3701
    }

    /// Converts meters to kilometers.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_kilometers(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts meters to centimeters.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_centimeters(value: f64) -> f64 {
        value * 100.0
    }

    /// Converts meters to millimeters.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_millimeters(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts meters to yards.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_yards(value: f64) -> f64 {
        value * 1.09361
    }

    /// Converts meters to miles.
    /// # Arguments
    /// * `value` - The length in meters to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 1609.34
    }
}

/// Feet conversion functions
pub mod feet {
    /// Converts feet to meters.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_meters(value: f64) -> f64 {
        value / 3.28084
    }

    /// Converts feet to inches.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 12.0
    }

    /// Converts feet to yards.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_yards(value: f64) -> f64 {
        value / 3.0
    }

    /// Converts feet to miles.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 5280.0
    }

    /// Converts feet to centimeters.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_centimeters(value: f64) -> f64 {
        super::meters::to_centimeters(to_meters(value))
    }

    /// Converts feet to millimeters.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_millimeters(value: f64) -> f64 {
        super::meters::to_millimeters(to_meters(value))
    }

    /// Converts feet to kilometers.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_kilometers(value: f64) -> f64 {
        super::meters::to_kilometers(to_meters(value))
    }
}

/// Inches conversion functions
pub mod inches {
    /// Converts inches to meters.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_meters(value: f64) -> f64 {
        value / 39.3701
    }

    /// Converts inches to feet.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_feet(value: f64) -> f64 {
        value / 12.0
    }

    /// Converts inches to yards.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_yards(value: f64) -> f64 {
        value / 36.0
    }

    /// Converts inches to centimeters.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_centimeters(value: f64) -> f64 {
        value * 2.54
    }

    /// Converts inches to millimeters.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_millimeters(value: f64) -> f64 {
        value * 25.4
    }

    /// Converts inches to kilometers.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_kilometers(value: f64) -> f64 {
        super::meters::to_kilometers(to_meters(value))
    }

    /// Converts inches to miles.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_miles(value: f64) -> f64 {
        super::meters::to_miles(to_meters(value))
    }
}

/// Kilometers conversion functions
pub mod kilometers {
    /// Converts kilometers to meters.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_meters(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts kilometers to feet.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_feet(value: f64) -> f64 {
        super::meters::to_feet(to_meters(value))
    }

    /// Converts kilometers to inches.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_inches(value: f64) -> f64 {
        super::meters::to_inches(to_meters(value))
    }

    /// Converts kilometers to yards.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_yards(value: f64) -> f64 {
        super::meters::to_yards(to_meters(value))
    }

    /// Converts kilometers to miles.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 1.60934
    }

    /// Converts kilometers to centimeters.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_centimeters(value: f64) -> f64 {
        value * 100000.0
    }

    /// Converts kilometers to millimeters.
    /// # Arguments
    /// * `value` - The length in kilometers to convert
    pub fn to_millimeters(value: f64) -> f64 {
        value * 1000000.0
    }
}

/// Centimeters conversion functions
pub mod centimeters {
    /// Converts centimeters to meters.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_meters(value: f64) -> f64 {
        value / 100.0
    }

    /// Converts centimeters to feet.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_feet(value: f64) -> f64 {
        super::meters::to_feet(to_meters(value))
    }

    /// Converts centimeters to inches.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_inches(value: f64) -> f64 {
        value / 2.54
    }

    /// Converts centimeters to yards.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_yards(value: f64) -> f64 {
        super::meters::to_yards(to_meters(value))
    }

    /// Converts centimeters to miles.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_miles(value: f64) -> f64 {
        super::meters::to_miles(to_meters(value))
    }

    /// Converts centimeters to kilometers.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_kilometers(value: f64) -> f64 {
        value / 100000.0
    }

    /// Converts centimeters to millimeters.
    /// # Arguments
    /// * `value` - The length in centimeters to convert
    pub fn to_millimeters(value: f64) -> f64 {
        value * 10.0
    }
}

/// Millimeters conversion functions
pub mod millimeters {
    /// Converts millimeters to meters.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_meters(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts millimeters to feet.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_feet(value: f64) -> f64 {
        super::meters::to_feet(to_meters(value))
    }

    /// Converts millimeters to inches.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_inches(value: f64) -> f64 {
        value / 25.4
    }

    /// Converts millimeters to yards.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_yards(value: f64) -> f64 {
        super::meters::to_yards(to_meters(value))
    }

    /// Converts millimeters to miles.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_miles(value: f64) -> f64 {
        super::meters::to_miles(to_meters(value))
    }

    /// Converts millimeters to kilometers.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_kilometers(value: f64) -> f64 {
        value / 1000000.0
    }

    /// Converts millimeters to centimeters.
    /// # Arguments
    /// * `value` - The length in millimeters to convert
    pub fn to_centimeters(value: f64) -> f64 {
        value / 10.0
    }
}

/// Yards conversion functions
pub mod yards {
    /// Converts yards to meters.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_meters(value: f64) -> f64 {
        value / 1.09361
    }

    /// Converts yards to feet.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_feet(value: f64) -> f64 {
        value * 3.0
    }

    /// Converts yards to inches.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 36.0
    }

    /// Converts yards to miles.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 1760.0
    }

    /// Converts yards to kilometers.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_kilometers(value: f64) -> f64 {
        super::meters::to_kilometers(to_meters(value))
    }

    /// Converts yards to centimeters.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_centimeters(value: f64) -> f64 {
        super::meters::to_centimeters(to_meters(value))
    }

    /// Converts yards to millimeters.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_millimeters(value: f64) -> f64 {
        super::meters::to_millimeters(to_meters(value))
    }
}

/// Miles conversion functions
pub mod miles {
    /// Converts miles to meters.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_meters(value: f64) -> f64 {
        value * 1609.34
    }

    /// Converts miles to feet.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_feet(value: f64) -> f64 {
        value * 5280.0
    }

    /// Converts miles to inches.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 63360.0
    }

    /// Converts miles to yards.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_yards(value: f64) -> f64 {
        value * 1760.0
    }

    /// Converts miles to kilometers.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_kilometers(value: f64) -> f64 {
        value * 1.60934
    }

    /// Converts miles to centimeters.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_centimeters(value: f64) -> f64 {
        super::meters::to_centimeters(to_meters(value))
    }

    /// Converts miles to millimeters.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_millimeters(value: f64) -> f64 {
        super::meters::to_millimeters(to_meters(value))
    }
}

// Legacy function wrappers for backward compatibility
/// Converts meters to feet (legacy function).
/// 
/// **Note:** Consider using `meters::to_feet()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_feet(meters: f64) -> f64 {
    meters::to_feet(meters)
}

/// Converts feet to meters (legacy function).
/// 
/// **Note:** Consider using `feet::to_meters()` for better organization.
/// 
/// # Arguments
/// * `feet` - The length in feet to convert
pub fn feet_to_meters(feet: f64) -> f64 {
    feet::to_meters(feet)
}

/// Converts meters to inches (legacy function).
/// 
/// **Note:** Consider using `meters::to_inches()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_inches(meters: f64) -> f64 {
    meters::to_inches(meters)
}

/// Converts inches to meters (legacy function).
/// 
/// **Note:** Consider using `inches::to_meters()` for better organization.
/// 
/// # Arguments
/// * `inches` - The length in inches to convert
pub fn inches_to_meters(inches: f64) -> f64 {
    inches::to_meters(inches)
}

/// Converts meters to kilometers (legacy function).
/// 
/// **Note:** Consider using `meters::to_kilometers()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_kilometers(meters: f64) -> f64 {
    meters::to_kilometers(meters)
}

/// Converts kilometers to meters (legacy function).
/// 
/// **Note:** Consider using `kilometers::to_meters()` for better organization.
/// 
/// # Arguments
/// * `kilometers` - The length in kilometers to convert
pub fn kilometers_to_meters(kilometers: f64) -> f64 {
    kilometers::to_meters(kilometers)
}

/// Converts meters to centimeters (legacy function).
/// 
/// **Note:** Consider using `meters::to_centimeters()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_centimeters(meters: f64) -> f64 {
    meters::to_centimeters(meters)
}

/// Converts centimeters to meters (legacy function).
/// 
/// **Note:** Consider using `centimeters::to_meters()` for better organization.
/// 
/// # Arguments
/// * `centimeters` - The length in centimeters to convert
pub fn centimeters_to_meters(centimeters: f64) -> f64 {
    centimeters::to_meters(centimeters)
}

/// Converts meters to millimeters (legacy function).
/// 
/// **Note:** Consider using `meters::to_millimeters()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_millimeters(meters: f64) -> f64 {
    meters::to_millimeters(meters)
}

/// Converts millimeters to meters (legacy function).
/// 
/// **Note:** Consider using `millimeters::to_meters()` for better organization.
/// 
/// # Arguments
/// * `millimeters` - The length in millimeters to convert
pub fn millimeters_to_meters(millimeters: f64) -> f64 {
    millimeters::to_meters(millimeters)
}

/// Converts meters to yards (legacy function).
/// 
/// **Note:** Consider using `meters::to_yards()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_yards(meters: f64) -> f64 {
    meters::to_yards(meters)
}

/// Converts yards to meters (legacy function).
/// 
/// **Note:** Consider using `yards::to_meters()` for better organization.
/// 
/// # Arguments
/// * `yards` - The length in yards to convert
pub fn yards_to_meters(yards: f64) -> f64 {
    yards::to_meters(yards)
}

/// Converts meters to miles (legacy function).
/// 
/// **Note:** Consider using `meters::to_miles()` for better organization.
/// 
/// # Arguments
/// * `meters` - The length in meters to convert
pub fn meters_to_miles(meters: f64) -> f64 {
    meters::to_miles(meters)
}

/// Converts miles to meters (legacy function).
/// 
/// **Note:** Consider using `miles::to_meters()` for better organization.
/// 
/// # Arguments
/// * `miles` - The length in miles to convert
pub fn miles_to_meters(miles: f64) -> f64 {
    miles::to_meters(miles)
}

/// Converts between any two supported length units.
/// 
/// This is the main conversion function that can handle conversions between any
/// combination of supported length units. All conversions are done through meters
/// as an intermediate base unit to ensure accuracy and consistency.
/// 
/// # Supported Units
/// 
/// * **Metric:** `m`, `meter`, `meters`, `km`, `kilometer`, `kilometers`, 
///   `cm`, `centimeter`, `centimeters`, `mm`, `millimeter`, `millimeters`
/// * **Imperial:** `ft`, `foot`, `feet`, `in`, `inch`, `inches`, 
///   `yd`, `yard`, `yards`, `mi`, `mile`, `miles`
/// 
/// Unit names are case-insensitive and support both singular and plural forms.
/// 
/// # Arguments
/// 
/// * `value` - The numeric value to convert
/// * `from` - The source unit (case-insensitive)
/// * `to` - The target unit (case-insensitive)
/// 
/// # Returns
/// 
/// * `Ok(f64)` - The converted value if both units are recognized
/// * `Err(String)` - An error message if either unit is not supported
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::convert_length;
/// 
/// // Convert 100 feet to meters
/// let result = convert_length(100.0, "ft", "m").unwrap();
/// assert!((result - 30.48).abs() < 0.01);
/// 
/// // Convert 5 kilometers to miles
/// let result = convert_length(5.0, "km", "mi").unwrap();
/// assert!((result - 3.10686).abs() < 0.001);
/// 
/// // Convert 42 inches to centimeters
/// let result = convert_length(42.0, "in", "cm").unwrap();
/// assert!((result - 106.68).abs() < 0.01);
/// 
/// // Case-insensitive and plural forms work
/// let result = convert_length(1.0, "METER", "feet").unwrap();
/// assert!((result - 3.28084).abs() < 0.0001);
/// 
/// // Error handling for unknown units
/// assert!(convert_length(1.0, "invalid", "m").is_err());
/// ```
/// 
/// # Conversion Accuracy
/// 
/// All conversions use standard international conversion factors:
/// - 1 meter = 3.28084 feet
/// - 1 meter = 39.3701 inches  
/// - 1 mile = 1609.34 meters
/// - etc.
/// 
/// Results maintain high precision suitable for most applications.
pub fn convert_length(value: f64, from: &str, to: &str) -> Result<f64, String> {
    // First convert to meters (base unit)
    let meters = match from.to_lowercase().as_str() {
        "m" | "meter" | "meters" => value,
        "km" | "kilometer" | "kilometers" => kilometers_to_meters(value),
        "cm" | "centimeter" | "centimeters" => centimeters_to_meters(value),
        "mm" | "millimeter" | "millimeters" => millimeters_to_meters(value),
        "ft" | "foot" | "feet" => feet_to_meters(value),
        "in" | "inch" | "inches" => inches_to_meters(value),
        "yd" | "yard" | "yards" => yards_to_meters(value),
        "mi" | "mile" | "miles" => miles_to_meters(value),
        _ => return Err(format!("Unknown length unit: {}", from)),
    };

    // Then convert from meters to target unit
    let result = match to.to_lowercase().as_str() {
        "m" | "meter" | "meters" => meters,
        "km" | "kilometer" | "kilometers" => meters_to_kilometers(meters),
        "cm" | "centimeter" | "centimeters" => meters_to_centimeters(meters),
        "mm" | "millimeter" | "millimeters" => meters_to_millimeters(meters),
        "ft" | "foot" | "feet" => meters_to_feet(meters),
        "in" | "inch" | "inches" => meters_to_inches(meters),
        "yd" | "yard" | "yards" => meters_to_yards(meters),
        "mi" | "mile" | "miles" => meters_to_miles(meters),
        _ => return Err(format!("Unknown length unit: {}", to)),
    };

    Ok(result)
}
