//! Amount of substance conversion functions organized by unit type
//! All conversions use moles as the base unit for accuracy and consistency

/// Moles conversion functions
pub mod moles {
    /// Converts moles to millimoles.
    /// # Arguments
    /// * `value` - The amount in moles to convert
    pub fn to_millimoles(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts moles to micromoles.
    /// # Arguments
    /// * `value` - The amount in moles to convert
    pub fn to_micromoles(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts moles to nanomoles.
    /// # Arguments
    /// * `value` - The amount in moles to convert
    pub fn to_nanomoles(value: f64) -> f64 {
        value * 1_000_000_000.0
    }

    /// Converts moles to picomoles.
    /// # Arguments
    /// * `value` - The amount in moles to convert
    pub fn to_picomoles(value: f64) -> f64 {
        value * 1_000_000_000_000.0
    }

    /// Converts moles to kilomoles.
    /// # Arguments
    /// * `value` - The amount in moles to convert
    pub fn to_kilomoles(value: f64) -> f64 {
        value / 1000.0
    }
}

/// Millimoles conversion functions
pub mod millimoles {
    /// Converts millimoles to moles.
    /// # Arguments
    /// * `value` - The amount in millimoles to convert
    pub fn to_moles(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts millimoles to micromoles.
    /// # Arguments
    /// * `value` - The amount in millimoles to convert
    pub fn to_micromoles(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts millimoles to nanomoles.
    /// # Arguments
    /// * `value` - The amount in millimoles to convert
    pub fn to_nanomoles(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts millimoles to picomoles.
    /// # Arguments
    /// * `value` - The amount in millimoles to convert
    pub fn to_picomoles(value: f64) -> f64 {
        value * 1_000_000_000.0
    }

    /// Converts millimoles to kilomoles.
    /// # Arguments
    /// * `value` - The amount in millimoles to convert
    pub fn to_kilomoles(value: f64) -> f64 {
        super::moles::to_kilomoles(to_moles(value))
    }
}

/// Micromoles conversion functions
pub mod micromoles {
    /// Converts micromoles to moles.
    /// # Arguments
    /// * `value` - The amount in micromoles to convert
    pub fn to_moles(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts micromoles to millimoles.
    /// # Arguments
    /// * `value` - The amount in micromoles to convert
    pub fn to_millimoles(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts micromoles to nanomoles.
    /// # Arguments
    /// * `value` - The amount in micromoles to convert
    pub fn to_nanomoles(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts micromoles to picomoles.
    /// # Arguments
    /// * `value` - The amount in micromoles to convert
    pub fn to_picomoles(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts micromoles to kilomoles.
    /// # Arguments
    /// * `value` - The amount in micromoles to convert
    pub fn to_kilomoles(value: f64) -> f64 {
        super::moles::to_kilomoles(to_moles(value))
    }
}

/// Nanomoles conversion functions
pub mod nanomoles {
    /// Converts nanomoles to moles.
    /// # Arguments
    /// * `value` - The amount in nanomoles to convert
    pub fn to_moles(value: f64) -> f64 {
        value / 1_000_000_000.0
    }

    /// Converts nanomoles to millimoles.
    /// # Arguments
    /// * `value` - The amount in nanomoles to convert
    pub fn to_millimoles(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts nanomoles to micromoles.
    /// # Arguments
    /// * `value` - The amount in nanomoles to convert
    pub fn to_micromoles(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts nanomoles to picomoles.
    /// # Arguments
    /// * `value` - The amount in nanomoles to convert
    pub fn to_picomoles(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts nanomoles to kilomoles.
    /// # Arguments
    /// * `value` - The amount in nanomoles to convert
    pub fn to_kilomoles(value: f64) -> f64 {
        super::moles::to_kilomoles(to_moles(value))
    }
}

/// Picomoles conversion functions
pub mod picomoles {
    /// Converts picomoles to moles.
    /// # Arguments
    /// * `value` - The amount in picomoles to convert
    pub fn to_moles(value: f64) -> f64 {
        value / 1_000_000_000_000.0
    }

    /// Converts picomoles to millimoles.
    /// # Arguments
    /// * `value` - The amount in picomoles to convert
    pub fn to_millimoles(value: f64) -> f64 {
        value / 1_000_000_000.0
    }

    /// Converts picomoles to micromoles.
    /// # Arguments
    /// * `value` - The amount in picomoles to convert
    pub fn to_micromoles(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts picomoles to nanomoles.
    /// # Arguments
    /// * `value` - The amount in picomoles to convert
    pub fn to_nanomoles(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts picomoles to kilomoles.
    /// # Arguments
    /// * `value` - The amount in picomoles to convert
    pub fn to_kilomoles(value: f64) -> f64 {
        super::moles::to_kilomoles(to_moles(value))
    }
}

/// Kilomoles conversion functions
pub mod kilomoles {
    /// Converts kilomoles to moles.
    /// # Arguments
    /// * `value` - The amount in kilomoles to convert
    pub fn to_moles(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts kilomoles to millimoles.
    /// # Arguments
    /// * `value` - The amount in kilomoles to convert
    pub fn to_millimoles(value: f64) -> f64 {
        super::moles::to_millimoles(to_moles(value))
    }

    /// Converts kilomoles to micromoles.
    /// # Arguments
    /// * `value` - The amount in kilomoles to convert
    pub fn to_micromoles(value: f64) -> f64 {
        super::moles::to_micromoles(to_moles(value))
    }

    /// Converts kilomoles to nanomoles.
    /// # Arguments
    /// * `value` - The amount in kilomoles to convert
    pub fn to_nanomoles(value: f64) -> f64 {
        super::moles::to_nanomoles(to_moles(value))
    }

    /// Converts kilomoles to picomoles.
    /// # Arguments
    /// * `value` - The amount in kilomoles to convert
    pub fn to_picomoles(value: f64) -> f64 {
        super::moles::to_picomoles(to_moles(value))
    }
}

// Legacy function wrappers for backward compatibility
pub fn moles_to_millimoles(moles: f64) -> f64 {
    moles::to_millimoles(moles)
}

pub fn millimoles_to_moles(millimoles: f64) -> f64 {
    millimoles::to_moles(millimoles)
}

pub fn moles_to_micromoles(moles: f64) -> f64 {
    moles::to_micromoles(moles)
}

pub fn micromoles_to_moles(micromoles: f64) -> f64 {
    micromoles::to_moles(micromoles)
}

pub fn moles_to_nanomoles(moles: f64) -> f64 {
    moles::to_nanomoles(moles)
}

pub fn nanomoles_to_moles(nanomoles: f64) -> f64 {
    nanomoles::to_moles(nanomoles)
}

pub fn moles_to_kilomoles(moles: f64) -> f64 {
    moles::to_kilomoles(moles)
}

pub fn kilomoles_to_moles(kilomoles: f64) -> f64 {
    kilomoles::to_moles(kilomoles)
}

/// General amount of substance conversion function that accepts string unit names
/// 
/// Converts an amount of substance value from one unit to another using string identifiers.
/// This function is case-insensitive and supports common abbreviations.
///
/// # Arguments
/// 
/// * `value` - The numeric value to convert
/// * `from_unit` - The source unit (e.g., "mol", "mmol", "μmol", "nmol", "pmol", "kmol")
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
/// use conversions_rs::convert_amount;
/// 
/// let millimoles = convert_amount(2.5, "mol", "mmol").unwrap();
/// assert_eq!(millimoles, 2500.0);
/// 
/// let moles = convert_amount(500.0, "mmol", "mol").unwrap();
/// assert_eq!(moles, 0.5);
/// ```
pub fn convert_amount(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();
    
    // Convert input to moles first
    let moles = match from_unit.as_str() {
        "mol" | "mole" | "moles" => value,
        "mmol" | "millimol" | "millimole" | "millimoles" => millimoles::to_moles(value),
        "μmol" | "umol" | "micromol" | "micromole" | "micromoles" => micromoles::to_moles(value),
        "nmol" | "nanomol" | "nanomole" | "nanomoles" => nanomoles::to_moles(value),
        "pmol" | "picomol" | "picomole" | "picomoles" => picomoles::to_moles(value),
        "kmol" | "kilomol" | "kilomole" | "kilomoles" => kilomoles::to_moles(value),
        _ => return Err(format!("Unsupported amount unit: {}", from_unit)),
    };
    
    // Convert moles to target unit
    let result = match to_unit.as_str() {
        "mol" | "mole" | "moles" => moles,
        "mmol" | "millimol" | "millimole" | "millimoles" => moles::to_millimoles(moles),
        "μmol" | "umol" | "micromol" | "micromole" | "micromoles" => moles::to_micromoles(moles),
        "nmol" | "nanomol" | "nanomole" | "nanomoles" => moles::to_nanomoles(moles),
        "pmol" | "picomol" | "picomole" | "picomoles" => moles::to_picomoles(moles),
        "kmol" | "kilomol" | "kilomole" | "kilomoles" => moles::to_kilomoles(moles),
        _ => return Err(format!("Unsupported amount unit: {}", to_unit)),
    };
    
    Ok(result)
}
