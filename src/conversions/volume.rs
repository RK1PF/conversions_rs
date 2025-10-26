//! Volume conversion functions organized by unit type
//! All conversions use liters as the base unit for accuracy and consistency

/// Liters conversion functions
pub mod liters {
    /// Converts liters to US gallons.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        value / 3.78541
    }

    /// Converts liters to UK gallons.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        value / 4.54609
    }

    /// Converts liters to milliliters.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_milliliters(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts liters to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        value * 33.814
    }

    /// Converts liters to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        value * 35.1951
    }

    /// Converts liters to US cups.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_cups_us(value: f64) -> f64 {
        value * 4.22675
    }

    /// Converts liters to US pints.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_pints_us(value: f64) -> f64 {
        value * 2.11338
    }

    /// Converts liters to US quarts.
    /// # Arguments
    /// * `value` - The volume in liters to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        value * 1.05669
    }
}

/// US gallons conversion functions
pub mod gallons_us {
    /// Converts US gallons to liters.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_liters(value: f64) -> f64 {
        value * 3.78541
    }

    /// Converts US gallons to UK gallons.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        super::liters::to_gallons_uk(to_liters(value))
    }

    /// Converts US gallons to milliliters.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_milliliters(value: f64) -> f64 {
        super::liters::to_milliliters(to_liters(value))
    }

    /// Converts US gallons to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        value * 128.0
    }

    /// Converts US gallons to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        super::liters::to_fluid_ounces_uk(to_liters(value))
    }

    /// Converts US gallons to US cups.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_cups_us(value: f64) -> f64 {
        value * 16.0
    }

    /// Converts US gallons to US pints.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_pints_us(value: f64) -> f64 {
        value * 8.0
    }

    /// Converts US gallons to US quarts.
    /// # Arguments
    /// * `value` - The volume in US gallons to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        value * 4.0
    }
}

/// UK gallons conversion functions
pub mod gallons_uk {
    /// Converts UK gallons to liters.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_liters(value: f64) -> f64 {
        value * 4.54609
    }

    /// Converts UK gallons to US gallons.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        super::liters::to_gallons_us(to_liters(value))
    }

    /// Converts UK gallons to milliliters.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_milliliters(value: f64) -> f64 {
        super::liters::to_milliliters(to_liters(value))
    }

    /// Converts UK gallons to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        super::liters::to_fluid_ounces_us(to_liters(value))
    }

    /// Converts UK gallons to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        value * 160.0
    }

    /// Converts UK gallons to US cups.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_cups_us(value: f64) -> f64 {
        super::liters::to_cups_us(to_liters(value))
    }

    /// Converts UK gallons to US pints.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_pints_us(value: f64) -> f64 {
        super::liters::to_pints_us(to_liters(value))
    }

    /// Converts UK gallons to US quarts.
    /// # Arguments
    /// * `value` - The volume in UK gallons to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        super::liters::to_quarts_us(to_liters(value))
    }
}

/// Milliliters conversion functions
pub mod milliliters {
    /// Converts milliliters to liters.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_liters(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts milliliters to US gallons.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        super::liters::to_gallons_us(to_liters(value))
    }

    /// Converts milliliters to UK gallons.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        super::liters::to_gallons_uk(to_liters(value))
    }

    /// Converts milliliters to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        value / 29.5735
    }

    /// Converts milliliters to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        value / 28.4131
    }

    /// Converts milliliters to US cups.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_cups_us(value: f64) -> f64 {
        value / 236.588
    }

    /// Converts milliliters to US pints.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_pints_us(value: f64) -> f64 {
        value / 473.176
    }

    /// Converts milliliters to US quarts.
    /// # Arguments
    /// * `value` - The volume in milliliters to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        value / 946.353
    }
}

/// US fluid ounces conversion functions
pub mod fluid_ounces_us {
    /// Converts US fluid ounces to liters.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_liters(value: f64) -> f64 {
        value / 33.814
    }

    /// Converts US fluid ounces to US gallons.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        value / 128.0
    }

    /// Converts US fluid ounces to UK gallons.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        super::liters::to_gallons_uk(to_liters(value))
    }

    /// Converts US fluid ounces to milliliters.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_milliliters(value: f64) -> f64 {
        value * 29.5735
    }

    /// Converts US fluid ounces to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        super::liters::to_fluid_ounces_uk(to_liters(value))
    }

    /// Converts US fluid ounces to US cups.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_cups_us(value: f64) -> f64 {
        value / 8.0
    }

    /// Converts US fluid ounces to US pints.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_pints_us(value: f64) -> f64 {
        value / 16.0
    }

    /// Converts US fluid ounces to US quarts.
    /// # Arguments
    /// * `value` - The volume in US fluid ounces to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        value / 32.0
    }
}

/// UK fluid ounces conversion functions
pub mod fluid_ounces_uk {
    /// Converts UK fluid ounces to liters.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_liters(value: f64) -> f64 {
        value / 35.1951
    }

    /// Converts UK fluid ounces to US gallons.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        super::liters::to_gallons_us(to_liters(value))
    }

    /// Converts UK fluid ounces to UK gallons.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        value / 160.0
    }

    /// Converts UK fluid ounces to milliliters.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_milliliters(value: f64) -> f64 {
        value * 28.4131
    }

    /// Converts UK fluid ounces to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        super::liters::to_fluid_ounces_us(to_liters(value))
    }

    /// Converts UK fluid ounces to US cups.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_cups_us(value: f64) -> f64 {
        super::liters::to_cups_us(to_liters(value))
    }

    /// Converts UK fluid ounces to US pints.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_pints_us(value: f64) -> f64 {
        super::liters::to_pints_us(to_liters(value))
    }

    /// Converts UK fluid ounces to US quarts.
    /// # Arguments
    /// * `value` - The volume in UK fluid ounces to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        super::liters::to_quarts_us(to_liters(value))
    }
}

/// US cups conversion functions
pub mod cups_us {
    /// Converts US cups to liters.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_liters(value: f64) -> f64 {
        value / 4.22675
    }

    /// Converts US cups to US gallons.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        value / 16.0
    }

    /// Converts US cups to UK gallons.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        super::liters::to_gallons_uk(to_liters(value))
    }

    /// Converts US cups to milliliters.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_milliliters(value: f64) -> f64 {
        value * 236.588
    }

    /// Converts US cups to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        value * 8.0
    }

    /// Converts US cups to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        super::liters::to_fluid_ounces_uk(to_liters(value))
    }

    /// Converts US cups to US pints.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_pints_us(value: f64) -> f64 {
        value / 2.0
    }

    /// Converts US cups to US quarts.
    /// # Arguments
    /// * `value` - The volume in US cups to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        value / 4.0
    }
}

/// US pints conversion functions
pub mod pints_us {
    /// Converts US pints to liters.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_liters(value: f64) -> f64 {
        value / 2.11338
    }

    /// Converts US pints to US gallons.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        value / 8.0
    }

    /// Converts US pints to UK gallons.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        super::liters::to_gallons_uk(to_liters(value))
    }

    /// Converts US pints to milliliters.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_milliliters(value: f64) -> f64 {
        value * 473.176
    }

    /// Converts US pints to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        value * 16.0
    }

    /// Converts US pints to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        super::liters::to_fluid_ounces_uk(to_liters(value))
    }

    /// Converts US pints to US cups.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_cups_us(value: f64) -> f64 {
        value * 2.0
    }

    /// Converts US pints to US quarts.
    /// # Arguments
    /// * `value` - The volume in US pints to convert
    pub fn to_quarts_us(value: f64) -> f64 {
        value / 2.0
    }
}

/// US quarts conversion functions
pub mod quarts_us {
    /// Converts US quarts to liters.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_liters(value: f64) -> f64 {
        value / 1.05669
    }

    /// Converts US quarts to US gallons.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_gallons_us(value: f64) -> f64 {
        value / 4.0
    }

    /// Converts US quarts to UK gallons.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_gallons_uk(value: f64) -> f64 {
        super::liters::to_gallons_uk(to_liters(value))
    }

    /// Converts US quarts to milliliters.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_milliliters(value: f64) -> f64 {
        value * 946.353
    }

    /// Converts US quarts to US fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_fluid_ounces_us(value: f64) -> f64 {
        value * 32.0
    }

    /// Converts US quarts to UK fluid ounces.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_fluid_ounces_uk(value: f64) -> f64 {
        super::liters::to_fluid_ounces_uk(to_liters(value))
    }

    /// Converts US quarts to US cups.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_cups_us(value: f64) -> f64 {
        value * 4.0
    }

    /// Converts US quarts to US pints.
    /// # Arguments
    /// * `value` - The volume in US quarts to convert
    pub fn to_pints_us(value: f64) -> f64 {
        value * 2.0
    }
}

// Legacy function wrappers for backward compatibility
pub fn liters_to_gallons_us(liters: f64) -> f64 {
    liters::to_gallons_us(liters)
}

pub fn gallons_us_to_liters(gallons: f64) -> f64 {
    gallons_us::to_liters(gallons)
}

pub fn liters_to_gallons_uk(liters: f64) -> f64 {
    liters::to_gallons_uk(liters)
}

pub fn gallons_uk_to_liters(gallons: f64) -> f64 {
    gallons_uk::to_liters(gallons)
}

pub fn liters_to_milliliters(liters: f64) -> f64 {
    liters::to_milliliters(liters)
}

pub fn milliliters_to_liters(ml: f64) -> f64 {
    milliliters::to_liters(ml)
}

pub fn liters_to_fluid_ounces_us(liters: f64) -> f64 {
    liters::to_fluid_ounces_us(liters)
}

pub fn fluid_ounces_us_to_liters(fl_oz: f64) -> f64 {
    fluid_ounces_us::to_liters(fl_oz)
}

pub fn liters_to_fluid_ounces_uk(liters: f64) -> f64 {
    liters::to_fluid_ounces_uk(liters)
}

pub fn fluid_ounces_uk_to_liters(fl_oz: f64) -> f64 {
    fluid_ounces_uk::to_liters(fl_oz)
}

pub fn liters_to_cups_us(liters: f64) -> f64 {
    liters::to_cups_us(liters)
}

pub fn cups_us_to_liters(cups: f64) -> f64 {
    cups_us::to_liters(cups)
}

pub fn liters_to_pints_us(liters: f64) -> f64 {
    liters::to_pints_us(liters)
}

pub fn pints_us_to_liters(pints: f64) -> f64 {
    pints_us::to_liters(pints)
}

pub fn liters_to_quarts_us(liters: f64) -> f64 {
    liters::to_quarts_us(liters)
}

pub fn quarts_us_to_liters(quarts: f64) -> f64 {
    quarts_us::to_liters(quarts)
}

/// Convert between any two volume units
pub fn convert_volume(value: f64, from: &str, to: &str) -> Result<f64, String> {
    // First convert to liters (base unit)
    let liters = match from.to_lowercase().as_str() {
        "l" | "liter" | "liters" | "litre" | "litres" => value,
        "ml" | "milliliter" | "milliliters" | "millilitre" | "millilitres" => {
            milliliters_to_liters(value)
        }
        "gal" | "gallon" | "gallons" | "gal_us" => gallons_us_to_liters(value),
        "gal_uk" | "gallon_uk" | "gallons_uk" => gallons_uk_to_liters(value),
        "fl_oz" | "fl_oz_us" | "fluid_ounce" | "fluid_ounces" => fluid_ounces_us_to_liters(value),
        "fl_oz_uk" | "fluid_ounce_uk" | "fluid_ounces_uk" => fluid_ounces_uk_to_liters(value),
        "cup" | "cups" | "cup_us" => cups_us_to_liters(value),
        "pt" | "pint" | "pints" | "pt_us" => pints_us_to_liters(value),
        "qt" | "quart" | "quarts" | "qt_us" => quarts_us_to_liters(value),
        _ => return Err(format!("Unknown volume unit: {}", from)),
    };

    // Then convert from liters to target unit
    let result = match to.to_lowercase().as_str() {
        "l" | "liter" | "liters" | "litre" | "litres" => liters,
        "ml" | "milliliter" | "milliliters" | "millilitre" | "millilitres" => {
            liters_to_milliliters(liters)
        }
        "gal" | "gallon" | "gallons" | "gal_us" => liters_to_gallons_us(liters),
        "gal_uk" | "gallon_uk" | "gallons_uk" => liters_to_gallons_uk(liters),
        "fl_oz" | "fl_oz_us" | "fluid_ounce" | "fluid_ounces" => liters_to_fluid_ounces_us(liters),
        "fl_oz_uk" | "fluid_ounce_uk" | "fluid_ounces_uk" => liters_to_fluid_ounces_uk(liters),
        "cup" | "cups" | "cup_us" => liters_to_cups_us(liters),
        "pt" | "pint" | "pints" | "pt_us" => liters_to_pints_us(liters),
        "qt" | "quart" | "quarts" | "qt_us" => liters_to_quarts_us(liters),
        _ => return Err(format!("Unknown volume unit: {}", to)),
    };

    Ok(result)
}
