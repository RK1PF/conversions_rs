//! Electric current conversion functions organized by unit type
//! All conversions use amperes as the base unit for accuracy and consistency

/// Amperes conversion functions
pub mod amperes {
    /// Converts amperes to milliamperes.
    /// # Arguments
    /// * `value` - The current in amperes to convert
    pub fn to_milliamperes(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts amperes to microamperes.
    /// # Arguments
    /// * `value` - The current in amperes to convert
    pub fn to_microamperes(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts amperes to nanoamperes.
    /// # Arguments
    /// * `value` - The current in amperes to convert
    pub fn to_nanoamperes(value: f64) -> f64 {
        value * 1_000_000_000.0
    }

    /// Converts amperes to kiloamperes.
    /// # Arguments
    /// * `value` - The current in amperes to convert
    pub fn to_kiloamperes(value: f64) -> f64 {
        value / 1000.0
    }
}

/// Milliamperes conversion functions
pub mod milliamperes {
    /// Converts milliamperes to amperes.
    /// # Arguments
    /// * `value` - The current in milliamperes to convert
    pub fn to_amperes(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts milliamperes to microamperes.
    /// # Arguments
    /// * `value` - The current in milliamperes to convert
    pub fn to_microamperes(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts milliamperes to nanoamperes.
    /// # Arguments
    /// * `value` - The current in milliamperes to convert
    pub fn to_nanoamperes(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts milliamperes to kiloamperes.
    /// # Arguments
    /// * `value` - The current in milliamperes to convert
    pub fn to_kiloamperes(value: f64) -> f64 {
        super::amperes::to_kiloamperes(to_amperes(value))
    }
}

/// Microamperes conversion functions
pub mod microamperes {
    /// Converts microamperes to amperes.
    /// # Arguments
    /// * `value` - The current in microamperes to convert
    pub fn to_amperes(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts microamperes to milliamperes.
    /// # Arguments
    /// * `value` - The current in microamperes to convert
    pub fn to_milliamperes(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts microamperes to nanoamperes.
    /// # Arguments
    /// * `value` - The current in microamperes to convert
    pub fn to_nanoamperes(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts microamperes to kiloamperes.
    /// # Arguments
    /// * `value` - The current in microamperes to convert
    pub fn to_kiloamperes(value: f64) -> f64 {
        super::amperes::to_kiloamperes(to_amperes(value))
    }
}

/// Nanoamperes conversion functions
pub mod nanoamperes {
    /// Converts nanoamperes to amperes.
    /// # Arguments
    /// * `value` - The current in nanoamperes to convert
    pub fn to_amperes(value: f64) -> f64 {
        value / 1_000_000_000.0
    }

    /// Converts nanoamperes to milliamperes.
    /// # Arguments
    /// * `value` - The current in nanoamperes to convert
    pub fn to_milliamperes(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts nanoamperes to microamperes.
    /// # Arguments
    /// * `value` - The current in nanoamperes to convert
    pub fn to_microamperes(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts nanoamperes to kiloamperes.
    /// # Arguments
    /// * `value` - The current in nanoamperes to convert
    pub fn to_kiloamperes(value: f64) -> f64 {
        super::amperes::to_kiloamperes(to_amperes(value))
    }
}

/// Kiloamperes conversion functions
pub mod kiloamperes {
    /// Converts kiloamperes to amperes.
    /// # Arguments
    /// * `value` - The current in kiloamperes to convert
    pub fn to_amperes(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts kiloamperes to milliamperes.
    /// # Arguments
    /// * `value` - The current in kiloamperes to convert
    pub fn to_milliamperes(value: f64) -> f64 {
        super::amperes::to_milliamperes(to_amperes(value))
    }

    /// Converts kiloamperes to microamperes.
    /// # Arguments
    /// * `value` - The current in kiloamperes to convert
    pub fn to_microamperes(value: f64) -> f64 {
        super::amperes::to_microamperes(to_amperes(value))
    }

    /// Converts kiloamperes to nanoamperes.
    /// # Arguments
    /// * `value` - The current in kiloamperes to convert
    pub fn to_nanoamperes(value: f64) -> f64 {
        super::amperes::to_nanoamperes(to_amperes(value))
    }
}

// Legacy function wrappers for backward compatibility
pub fn amperes_to_milliamperes(amperes: f64) -> f64 {
    amperes::to_milliamperes(amperes)
}

pub fn milliamperes_to_amperes(milliamperes: f64) -> f64 {
    milliamperes::to_amperes(milliamperes)
}

pub fn amperes_to_microamperes(amperes: f64) -> f64 {
    amperes::to_microamperes(amperes)
}

pub fn microamperes_to_amperes(microamperes: f64) -> f64 {
    microamperes::to_amperes(microamperes)
}

pub fn amperes_to_kiloamperes(amperes: f64) -> f64 {
    amperes::to_kiloamperes(amperes)
}

pub fn kiloamperes_to_amperes(kiloamperes: f64) -> f64 {
    kiloamperes::to_amperes(kiloamperes)
}

/// General electric current conversion function that accepts string unit names
/// 
/// Converts an electric current value from one unit to another using string identifiers.
/// This function is case-insensitive and supports common abbreviations.
///
/// # Arguments
/// 
/// * `value` - The numeric value to convert
/// * `from_unit` - The source unit (e.g., "A", "mA", "μA", "nA", "kA")
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
/// use conversions_rs::convert_current;
/// 
/// let milliamps = convert_current(2.5, "A", "mA").unwrap();
/// assert_eq!(milliamps, 2500.0);
/// 
/// let amps = convert_current(500.0, "mA", "A").unwrap();
/// assert_eq!(amps, 0.5);
/// ```
pub fn convert_current(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();
    
    // Convert input to amperes first
    let amperes = match from_unit.as_str() {
        "a" | "amp" | "ampere" | "amperes" => value,
        "ma" | "milliamp" | "milliampere" | "milliamperes" => milliamperes::to_amperes(value),
        "μa" | "ua" | "microamp" | "microampere" | "microamperes" => microamperes::to_amperes(value),
        "na" | "nanoamp" | "nanoampere" | "nanoamperes" => nanoamperes::to_amperes(value),
        "ka" | "kiloamp" | "kiloampere" | "kiloamperes" => kiloamperes::to_amperes(value),
        _ => return Err(format!("Unsupported current unit: {}", from_unit)),
    };
    
    // Convert amperes to target unit
    let result = match to_unit.as_str() {
        "a" | "amp" | "ampere" | "amperes" => amperes,
        "ma" | "milliamp" | "milliampere" | "milliamperes" => amperes::to_milliamperes(amperes),
        "μa" | "ua" | "microamp" | "microampere" | "microamperes" => amperes::to_microamperes(amperes),
        "na" | "nanoamp" | "nanoampere" | "nanoamperes" => amperes::to_nanoamperes(amperes),
        "ka" | "kiloamp" | "kiloampere" | "kiloamperes" => amperes::to_kiloamperes(amperes),
        _ => return Err(format!("Unsupported current unit: {}", to_unit)),
    };
    
    Ok(result)
}
