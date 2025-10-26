//! Luminous intensity conversion functions organized by unit type
//! All conversions use candela as the base unit for accuracy and consistency

/// Candela conversion functions
pub mod candela {
    /// Converts candela to millicandela.
    /// # Arguments
    /// * `value` - The luminous intensity in candela to convert
    pub fn to_millicandela(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts candela to kilocandela.
    /// # Arguments
    /// * `value` - The luminous intensity in candela to convert
    pub fn to_kilocandela(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts candela to hefnerkerze (approximation).
    /// # Arguments
    /// * `value` - The luminous intensity in candela to convert
    ///
    /// Note: 1 candela ≈ 1.11 hefnerkerze
    pub fn to_hefnerkerze(value: f64) -> f64 {
        value * 1.11
    }

    /// Converts candela to international candle (approximation).
    /// # Arguments
    /// * `value` - The luminous intensity in candela to convert
    ///
    /// Note: 1 candela ≈ 0.98 international candle
    pub fn to_international_candle(value: f64) -> f64 {
        value * 0.98
    }

    /// Converts candela to decimal candle.
    /// # Arguments
    /// * `value` - The luminous intensity in candela to convert
    ///
    /// Note: 1 candela = 1 decimal candle (by definition)
    pub fn to_decimal_candle(value: f64) -> f64 {
        value
    }
}

/// Millicandela conversion functions
pub mod millicandela {
    /// Converts millicandela to candela.
    /// # Arguments
    /// * `value` - The luminous intensity in millicandela to convert
    pub fn to_candela(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts millicandela to kilocandela.
    /// # Arguments
    /// * `value` - The luminous intensity in millicandela to convert
    pub fn to_kilocandela(value: f64) -> f64 {
        super::candela::to_kilocandela(to_candela(value))
    }

    /// Converts millicandela to hefnerkerze.
    /// # Arguments
    /// * `value` - The luminous intensity in millicandela to convert
    pub fn to_hefnerkerze(value: f64) -> f64 {
        super::candela::to_hefnerkerze(to_candela(value))
    }

    /// Converts millicandela to international candle.
    /// # Arguments
    /// * `value` - The luminous intensity in millicandela to convert
    pub fn to_international_candle(value: f64) -> f64 {
        super::candela::to_international_candle(to_candela(value))
    }

    /// Converts millicandela to decimal candle.
    /// # Arguments
    /// * `value` - The luminous intensity in millicandela to convert
    pub fn to_decimal_candle(value: f64) -> f64 {
        super::candela::to_decimal_candle(to_candela(value))
    }
}

/// Kilocandela conversion functions
pub mod kilocandela {
    /// Converts kilocandela to candela.
    /// # Arguments
    /// * `value` - The luminous intensity in kilocandela to convert
    pub fn to_candela(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts kilocandela to millicandela.
    /// # Arguments
    /// * `value` - The luminous intensity in kilocandela to convert
    pub fn to_millicandela(value: f64) -> f64 {
        super::candela::to_millicandela(to_candela(value))
    }

    /// Converts kilocandela to hefnerkerze.
    /// # Arguments
    /// * `value` - The luminous intensity in kilocandela to convert
    pub fn to_hefnerkerze(value: f64) -> f64 {
        super::candela::to_hefnerkerze(to_candela(value))
    }

    /// Converts kilocandela to international candle.
    /// # Arguments
    /// * `value` - The luminous intensity in kilocandela to convert
    pub fn to_international_candle(value: f64) -> f64 {
        super::candela::to_international_candle(to_candela(value))
    }

    /// Converts kilocandela to decimal candle.
    /// # Arguments
    /// * `value` - The luminous intensity in kilocandela to convert
    pub fn to_decimal_candle(value: f64) -> f64 {
        super::candela::to_decimal_candle(to_candela(value))
    }
}

/// Hefnerkerze conversion functions
pub mod hefnerkerze {
    /// Converts hefnerkerze to candela.
    /// # Arguments
    /// * `value` - The luminous intensity in hefnerkerze to convert
    ///
    /// Note: 1 hefnerkerze ≈ 0.901 candela
    pub fn to_candela(value: f64) -> f64 {
        value / 1.11
    }

    /// Converts hefnerkerze to millicandela.
    /// # Arguments
    /// * `value` - The luminous intensity in hefnerkerze to convert
    pub fn to_millicandela(value: f64) -> f64 {
        super::candela::to_millicandela(to_candela(value))
    }

    /// Converts hefnerkerze to kilocandela.
    /// # Arguments
    /// * `value` - The luminous intensity in hefnerkerze to convert
    pub fn to_kilocandela(value: f64) -> f64 {
        super::candela::to_kilocandela(to_candela(value))
    }

    /// Converts hefnerkerze to international candle.
    /// # Arguments
    /// * `value` - The luminous intensity in hefnerkerze to convert
    pub fn to_international_candle(value: f64) -> f64 {
        super::candela::to_international_candle(to_candela(value))
    }

    /// Converts hefnerkerze to decimal candle.
    /// # Arguments
    /// * `value` - The luminous intensity in hefnerkerze to convert
    pub fn to_decimal_candle(value: f64) -> f64 {
        super::candela::to_decimal_candle(to_candela(value))
    }
}

/// International candle conversion functions
pub mod international_candle {
    /// Converts international candle to candela.
    /// # Arguments
    /// * `value` - The luminous intensity in international candle to convert
    ///
    /// Note: 1 international candle ≈ 1.02 candela
    pub fn to_candela(value: f64) -> f64 {
        value / 0.98
    }

    /// Converts international candle to millicandela.
    /// # Arguments
    /// * `value` - The luminous intensity in international candle to convert
    pub fn to_millicandela(value: f64) -> f64 {
        super::candela::to_millicandela(to_candela(value))
    }

    /// Converts international candle to kilocandela.
    /// # Arguments
    /// * `value` - The luminous intensity in international candle to convert
    pub fn to_kilocandela(value: f64) -> f64 {
        super::candela::to_kilocandela(to_candela(value))
    }

    /// Converts international candle to hefnerkerze.
    /// # Arguments
    /// * `value` - The luminous intensity in international candle to convert
    pub fn to_hefnerkerze(value: f64) -> f64 {
        super::candela::to_hefnerkerze(to_candela(value))
    }

    /// Converts international candle to decimal candle.
    /// # Arguments
    /// * `value` - The luminous intensity in international candle to convert
    pub fn to_decimal_candle(value: f64) -> f64 {
        super::candela::to_decimal_candle(to_candela(value))
    }
}

/// Decimal candle conversion functions
pub mod decimal_candle {
    /// Converts decimal candle to candela.
    /// # Arguments
    /// * `value` - The luminous intensity in decimal candle to convert
    ///
    /// Note: 1 decimal candle = 1 candela (by definition)
    pub fn to_candela(value: f64) -> f64 {
        value
    }

    /// Converts decimal candle to millicandela.
    /// # Arguments
    /// * `value` - The luminous intensity in decimal candle to convert
    pub fn to_millicandela(value: f64) -> f64 {
        super::candela::to_millicandela(to_candela(value))
    }

    /// Converts decimal candle to kilocandela.
    /// # Arguments
    /// * `value` - The luminous intensity in decimal candle to convert
    pub fn to_kilocandela(value: f64) -> f64 {
        super::candela::to_kilocandela(to_candela(value))
    }

    /// Converts decimal candle to hefnerkerze.
    /// # Arguments
    /// * `value` - The luminous intensity in decimal candle to convert
    pub fn to_hefnerkerze(value: f64) -> f64 {
        super::candela::to_hefnerkerze(to_candela(value))
    }

    /// Converts decimal candle to international candle.
    /// # Arguments
    /// * `value` - The luminous intensity in decimal candle to convert
    pub fn to_international_candle(value: f64) -> f64 {
        super::candela::to_international_candle(to_candela(value))
    }
}

// Legacy function wrappers for backward compatibility
pub fn candela_to_millicandela(candela: f64) -> f64 {
    candela::to_millicandela(candela)
}

pub fn millicandela_to_candela(millicandela: f64) -> f64 {
    millicandela::to_candela(millicandela)
}

pub fn candela_to_kilocandela(candela: f64) -> f64 {
    candela::to_kilocandela(candela)
}

pub fn kilocandela_to_candela(kilocandela: f64) -> f64 {
    kilocandela::to_candela(kilocandela)
}

pub fn candela_to_hefnerkerze(candela: f64) -> f64 {
    candela::to_hefnerkerze(candela)
}

pub fn hefnerkerze_to_candela(hefnerkerze: f64) -> f64 {
    hefnerkerze::to_candela(hefnerkerze)
}

/// General luminous intensity conversion function that accepts string unit names
///
/// Converts a luminous intensity value from one unit to another using string identifiers.
/// This function is case-insensitive and supports common abbreviations.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from_unit` - The source unit (e.g., "cd", "mcd", "kcd", "hk", "ic", "dc")
/// * `to_unit` - The target unit using the same abbreviations
///
/// # Returns
/// * `Ok(f64)` - The converted value
/// * `Err(String)` - Error message if the conversion is not supported
///
/// # Examples
///
/// ```rust
/// use conversions_rs::convert_luminous_intensity;
///
/// let millicandela = convert_luminous_intensity(2.5, "cd", "mcd").unwrap();
/// assert_eq!(millicandela, 2500.0);
///
/// let candela = convert_luminous_intensity(500.0, "mcd", "cd").unwrap();
/// assert_eq!(candela, 0.5);
/// ```
pub fn convert_luminous_intensity(
    value: f64,
    from_unit: &str,
    to_unit: &str,
) -> Result<f64, String> {
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();

    // Convert input to candela first
    let candela = match from_unit.as_str() {
        "cd" | "candela" => value,
        "mcd" | "millicandela" => millicandela::to_candela(value),
        "kcd" | "kilocandela" => kilocandela::to_candela(value),
        "hk" | "hefnerkerze" => hefnerkerze::to_candela(value),
        "ic" | "international_candle" | "intl_candle" => international_candle::to_candela(value),
        "dc" | "decimal_candle" => decimal_candle::to_candela(value),
        _ => {
            return Err(format!(
                "Unsupported luminous intensity unit: {}",
                from_unit
            ))
        }
    };

    // Convert candela to target unit
    let result = match to_unit.as_str() {
        "cd" | "candela" => candela,
        "mcd" | "millicandela" => candela::to_millicandela(candela),
        "kcd" | "kilocandela" => candela::to_kilocandela(candela),
        "hk" | "hefnerkerze" => candela::to_hefnerkerze(candela),
        "ic" | "international_candle" | "intl_candle" => candela::to_international_candle(candela),
        "dc" | "decimal_candle" => candela::to_decimal_candle(candela),
        _ => return Err(format!("Unsupported luminous intensity unit: {}", to_unit)),
    };

    Ok(result)
}
