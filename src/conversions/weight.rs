//! Weight/Mass conversion functions organized by unit type
//! All conversions use kilograms as the base unit for accuracy and consistency

/// Kilograms conversion functions
pub mod kilograms {
    /// Converts kilograms to pounds.
    /// # Arguments
    /// * `value` - The weight in kilograms to convert
    pub fn to_pounds(value: f64) -> f64 {
        value * 2.20462
    }

    /// Converts kilograms to grams.
    /// # Arguments
    /// * `value` - The weight in kilograms to convert
    pub fn to_grams(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts kilograms to ounces.
    /// # Arguments
    /// * `value` - The weight in kilograms to convert
    pub fn to_ounces(value: f64) -> f64 {
        value * 35.274
    }

    /// Converts kilograms to tons (metric).
    /// # Arguments
    /// * `value` - The weight in kilograms to convert
    pub fn to_tons(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts kilograms to stones.
    /// # Arguments
    /// * `value` - The weight in kilograms to convert
    pub fn to_stones(value: f64) -> f64 {
        value / 6.35029
    }
}

/// Pounds conversion functions
pub mod pounds {
    /// Converts pounds to kilograms.
    /// # Arguments
    /// * `value` - The weight in pounds to convert
    pub fn to_kilograms(value: f64) -> f64 {
        value / 2.20462
    }

    /// Converts pounds to grams.
    /// # Arguments
    /// * `value` - The weight in pounds to convert
    pub fn to_grams(value: f64) -> f64 {
        super::kilograms::to_grams(to_kilograms(value))
    }

    /// Converts pounds to ounces.
    /// # Arguments
    /// * `value` - The weight in pounds to convert
    pub fn to_ounces(value: f64) -> f64 {
        value * 16.0
    }

    /// Converts pounds to tons (metric).
    /// # Arguments
    /// * `value` - The weight in pounds to convert
    pub fn to_tons(value: f64) -> f64 {
        super::kilograms::to_tons(to_kilograms(value))
    }

    /// Converts pounds to stones.
    /// # Arguments
    /// * `value` - The weight in pounds to convert
    pub fn to_stones(value: f64) -> f64 {
        value / 14.0
    }
}

/// Grams conversion functions
pub mod grams {
    /// Converts grams to kilograms.
    /// # Arguments
    /// * `value` - The weight in grams to convert
    pub fn to_kilograms(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts grams to pounds.
    /// # Arguments
    /// * `value` - The weight in grams to convert
    pub fn to_pounds(value: f64) -> f64 {
        super::kilograms::to_pounds(to_kilograms(value))
    }

    /// Converts grams to ounces.
    /// # Arguments
    /// * `value` - The weight in grams to convert
    pub fn to_ounces(value: f64) -> f64 {
        value / 28.3495
    }

    /// Converts grams to tons (metric).
    /// # Arguments
    /// * `value` - The weight in grams to convert
    pub fn to_tons(value: f64) -> f64 {
        value / 1000000.0
    }

    /// Converts grams to stones.
    /// # Arguments
    /// * `value` - The weight in grams to convert
    pub fn to_stones(value: f64) -> f64 {
        super::kilograms::to_stones(to_kilograms(value))
    }
}

/// Ounces conversion functions
pub mod ounces {
    /// Converts ounces to kilograms.
    /// # Arguments
    /// * `value` - The weight in ounces to convert
    pub fn to_kilograms(value: f64) -> f64 {
        value / 35.274
    }

    /// Converts ounces to pounds.
    /// # Arguments
    /// * `value` - The weight in ounces to convert
    pub fn to_pounds(value: f64) -> f64 {
        value / 16.0
    }

    /// Converts ounces to grams.
    /// # Arguments
    /// * `value` - The weight in ounces to convert
    pub fn to_grams(value: f64) -> f64 {
        value * 28.3495
    }

    /// Converts ounces to tons (metric).
    /// # Arguments
    /// * `value` - The weight in ounces to convert
    pub fn to_tons(value: f64) -> f64 {
        super::kilograms::to_tons(to_kilograms(value))
    }

    /// Converts ounces to stones.
    /// # Arguments
    /// * `value` - The weight in ounces to convert
    pub fn to_stones(value: f64) -> f64 {
        super::kilograms::to_stones(to_kilograms(value))
    }
}

/// Tons (metric) conversion functions
pub mod tons {
    /// Converts tons to kilograms.
    /// # Arguments
    /// * `value` - The weight in tons to convert
    pub fn to_kilograms(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts tons to pounds.
    /// # Arguments
    /// * `value` - The weight in tons to convert
    pub fn to_pounds(value: f64) -> f64 {
        super::kilograms::to_pounds(to_kilograms(value))
    }

    /// Converts tons to grams.
    /// # Arguments
    /// * `value` - The weight in tons to convert
    pub fn to_grams(value: f64) -> f64 {
        value * 1000000.0
    }

    /// Converts tons to ounces.
    /// # Arguments
    /// * `value` - The weight in tons to convert
    pub fn to_ounces(value: f64) -> f64 {
        super::kilograms::to_ounces(to_kilograms(value))
    }

    /// Converts tons to stones.
    /// # Arguments
    /// * `value` - The weight in tons to convert
    pub fn to_stones(value: f64) -> f64 {
        super::kilograms::to_stones(to_kilograms(value))
    }
}

/// Stones conversion functions
pub mod stones {
    /// Converts stones to kilograms.
    /// # Arguments
    /// * `value` - The weight in stones to convert
    pub fn to_kilograms(value: f64) -> f64 {
        value * 6.35029
    }

    /// Converts stones to pounds.
    /// # Arguments
    /// * `value` - The weight in stones to convert
    pub fn to_pounds(value: f64) -> f64 {
        value * 14.0
    }

    /// Converts stones to grams.
    /// # Arguments
    /// * `value` - The weight in stones to convert
    pub fn to_grams(value: f64) -> f64 {
        super::kilograms::to_grams(to_kilograms(value))
    }

    /// Converts stones to ounces.
    /// # Arguments
    /// * `value` - The weight in stones to convert
    pub fn to_ounces(value: f64) -> f64 {
        super::kilograms::to_ounces(to_kilograms(value))
    }

    /// Converts stones to tons (metric).
    /// # Arguments
    /// * `value` - The weight in stones to convert
    pub fn to_tons(value: f64) -> f64 {
        super::kilograms::to_tons(to_kilograms(value))
    }
}

// Legacy function wrappers for backward compatibility
/// Converts kilograms to pounds (legacy function).
///
/// **Note:** Consider using `kilograms::to_pounds()` for better organization.
///
/// Uses the conversion factor: 1 kilogram = 2.20462 pounds
///
/// # Arguments
///
/// * `kg` - The weight in kilograms to convert
///
/// # Returns
///
/// The equivalent weight in pounds
///
/// # Examples
///
/// ```
/// use conversions_rs::kilograms_to_pounds;
///
/// let lbs = kilograms_to_pounds(1.0);
/// assert!((lbs - 2.20462).abs() < 0.0001);
/// ```
pub fn kilograms_to_pounds(kg: f64) -> f64 {
    kilograms::to_pounds(kg)
}

/// Converts pounds to kilograms (legacy function).
///
/// **Note:** Consider using `pounds::to_kilograms()` for better organization.
///
/// Uses the conversion factor: 1 pound = 0.453592 kilograms
///
/// # Arguments
///
/// * `lbs` - The weight in pounds to convert
///
/// # Returns
///
/// The equivalent weight in kilograms
///
/// # Examples
///
/// ```
/// use conversions_rs::pounds_to_kilograms;
///
/// let kg = pounds_to_kilograms(2.20462);
/// assert!((kg - 1.0).abs() < 0.0001);
/// ```
pub fn pounds_to_kilograms(lbs: f64) -> f64 {
    pounds::to_kilograms(lbs)
}

pub fn kilograms_to_grams(kg: f64) -> f64 {
    kilograms::to_grams(kg)
}

pub fn grams_to_kilograms(grams: f64) -> f64 {
    grams::to_kilograms(grams)
}

pub fn kilograms_to_ounces(kg: f64) -> f64 {
    kilograms::to_ounces(kg)
}

pub fn ounces_to_kilograms(oz: f64) -> f64 {
    ounces::to_kilograms(oz)
}

pub fn kilograms_to_tons(kg: f64) -> f64 {
    kilograms::to_tons(kg)
}

pub fn tons_to_kilograms(tons: f64) -> f64 {
    tons::to_kilograms(tons)
}

pub fn kilograms_to_stones(kg: f64) -> f64 {
    kilograms::to_stones(kg)
}

pub fn stones_to_kilograms(stones: f64) -> f64 {
    stones::to_kilograms(stones)
}

/// Converts between any two supported weight/mass units.
///
/// This is the main conversion function that can handle conversions between any
/// combination of supported weight/mass units. All conversions are done through
/// kilograms as an intermediate base unit to ensure accuracy and consistency.
///
/// # Supported Units
///
/// * **Metric:** `kg`, `kilogram`, `kilograms`, `g`, `gram`, `grams`,
///   `t`, `ton`, `tons` (metric tons)
/// * **Imperial:** `lb`, `lbs`, `pound`, `pounds`, `oz`, `ounce`, `ounces`,
///   `st`, `stone`, `stones`
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
/// * `Ok(f64)` - The converted value if both units are recognized
/// * `Err(String)` - An error message if either unit is not supported
///
/// # Examples
///
/// ```
/// use conversions_rs::convert_weight;
///
/// // Convert 10 kilograms to pounds
/// let result = convert_weight(10.0, "kg", "lb").unwrap();
/// assert!((result - 22.0462).abs() < 0.001);
///
/// // Convert 1000 grams to kilograms
/// let result = convert_weight(1000.0, "g", "kg").unwrap();
/// assert_eq!(result, 1.0);
///
/// // Convert 16 ounces to pounds
/// let result = convert_weight(16.0, "oz", "lb").unwrap();
/// assert!((result - 1.0).abs() < 0.01);
///
/// // Case-insensitive and plural forms work
/// let result = convert_weight(1.0, "KILOGRAM", "pounds").unwrap();
/// assert!((result - 2.20462).abs() < 0.0001);
///
/// // Error handling for unknown units
/// assert!(convert_weight(1.0, "invalid", "kg").is_err());
/// ```
///
/// # Conversion Accuracy
///
/// All conversions use standard international conversion factors:
/// - 1 kilogram = 2.20462 pounds
/// - 1 kilogram = 1000 grams
/// - 1 stone = 6.35029 kilograms
/// - etc.
pub fn convert_weight(value: f64, from: &str, to: &str) -> Result<f64, String> {
    // First convert to kilograms (base unit)
    let kilograms = match from.to_lowercase().as_str() {
        "kg" | "kilogram" | "kilograms" => value,
        "g" | "gram" | "grams" => grams_to_kilograms(value),
        "lb" | "lbs" | "pound" | "pounds" => pounds_to_kilograms(value),
        "oz" | "ounce" | "ounces" => ounces_to_kilograms(value),
        "t" | "ton" | "tons" => tons_to_kilograms(value),
        "st" | "stone" | "stones" => stones_to_kilograms(value),
        _ => return Err(format!("Unknown weight unit: {}", from)),
    };

    // Then convert from kilograms to target unit
    let result = match to.to_lowercase().as_str() {
        "kg" | "kilogram" | "kilograms" => kilograms,
        "g" | "gram" | "grams" => kilograms_to_grams(kilograms),
        "lb" | "lbs" | "pound" | "pounds" => kilograms_to_pounds(kilograms),
        "oz" | "ounce" | "ounces" => kilograms_to_ounces(kilograms),
        "t" | "ton" | "tons" => kilograms_to_tons(kilograms),
        "st" | "stone" | "stones" => kilograms_to_stones(kilograms),
        _ => return Err(format!("Unknown weight unit: {}", to)),
    };

    Ok(result)
}
