//! Temperature conversion functions organized by unit type

/// Celsius conversion functions
pub mod celsius {
    /// Converts Celsius to Fahrenheit.
    /// # Arguments
    /// * `value` - The temperature in Celsius to convert
    pub fn to_fahrenheit(value: f64) -> f64 {
        value * 9.0 / 5.0 + 32.0
    }

    /// Converts Celsius to Kelvin.
    /// # Arguments
    /// * `value` - The temperature in Celsius to convert
    pub fn to_kelvin(value: f64) -> f64 {
        value + 273.15
    }
}

/// Fahrenheit conversion functions
pub mod fahrenheit {
    /// Converts Fahrenheit to Celsius.
    /// # Arguments
    /// * `value` - The temperature in Fahrenheit to convert
    pub fn to_celsius(value: f64) -> f64 {
        (value - 32.0) * 5.0 / 9.0
    }

    /// Converts Fahrenheit to Kelvin.
    /// # Arguments
    /// * `value` - The temperature in Fahrenheit to convert
    pub fn to_kelvin(value: f64) -> f64 {
        super::celsius::to_kelvin(to_celsius(value))
    }
}

/// Kelvin conversion functions
pub mod kelvin {
    /// Converts Kelvin to Celsius.
    /// # Arguments
    /// * `value` - The temperature in Kelvin to convert
    pub fn to_celsius(value: f64) -> f64 {
        value - 273.15
    }

    /// Converts Kelvin to Fahrenheit.
    /// # Arguments
    /// * `value` - The temperature in Kelvin to convert
    pub fn to_fahrenheit(value: f64) -> f64 {
        super::celsius::to_fahrenheit(to_celsius(value))
    }
}

// Legacy function wrappers for backward compatibility
/// Converts Celsius to Fahrenheit (legacy function).
/// 
/// **Note:** Consider using `celsius::to_fahrenheit()` for better organization.
/// 
/// Uses the formula: °F = (°C × 9/5) + 32
/// 
/// # Arguments
/// 
/// * `celsius` - The temperature in Celsius to convert
/// 
/// # Returns
/// 
/// The equivalent temperature in Fahrenheit
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::celsius_to_fahrenheit;
/// 
/// let fahrenheit = celsius_to_fahrenheit(0.0);
/// assert_eq!(fahrenheit, 32.0);
/// 
/// let fahrenheit = celsius_to_fahrenheit(100.0);
/// assert_eq!(fahrenheit, 212.0);
/// ```
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius::to_fahrenheit(celsius)
}

/// Converts Fahrenheit to Celsius (legacy function).
/// 
/// **Note:** Consider using `fahrenheit::to_celsius()` for better organization.
/// 
/// Uses the formula: °C = (°F - 32) × 5/9
/// 
/// # Arguments
/// 
/// * `fahrenheit` - The temperature in Fahrenheit to convert
/// 
/// # Returns
/// 
/// The equivalent temperature in Celsius
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::fahrenheit_to_celsius;
/// 
/// let celsius = fahrenheit_to_celsius(32.0);
/// assert_eq!(celsius, 0.0);
/// 
/// let celsius = fahrenheit_to_celsius(212.0);
/// assert_eq!(celsius, 100.0);
/// ```
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    fahrenheit::to_celsius(fahrenheit)
}

/// Converts Celsius to Kelvin (legacy function).
/// 
/// **Note:** Consider using `celsius::to_kelvin()` for better organization.
/// 
/// Uses the formula: K = °C + 273.15
/// 
/// # Arguments
/// 
/// * `celsius` - The temperature in Celsius to convert
/// 
/// # Returns
/// 
/// The equivalent temperature in Kelvin
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::celsius_to_kelvin;
/// 
/// let kelvin = celsius_to_kelvin(0.0);
/// assert_eq!(kelvin, 273.15);
/// 
/// let kelvin = celsius_to_kelvin(-273.15);
/// assert_eq!(kelvin, 0.0);
/// ```
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius::to_kelvin(celsius)
}

/// Converts Kelvin to Celsius (legacy function).
/// 
/// **Note:** Consider using `kelvin::to_celsius()` for better organization.
/// 
/// Uses the formula: °C = K - 273.15
/// 
/// # Arguments
/// 
/// * `kelvin` - The temperature in Kelvin to convert
/// 
/// # Returns
/// 
/// The equivalent temperature in Celsius
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::kelvin_to_celsius;
/// 
/// let celsius = kelvin_to_celsius(273.15);
/// assert_eq!(celsius, 0.0);
/// 
/// let celsius = kelvin_to_celsius(373.15);
/// assert_eq!(celsius, 100.0);
/// ```
pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin::to_celsius(kelvin)
}

/// Converts Fahrenheit to Kelvin (legacy function).
/// 
/// **Note:** Consider using `fahrenheit::to_kelvin()` for better organization.
/// 
/// Converts through Celsius as an intermediate step for accuracy.
/// 
/// # Arguments
/// 
/// * `fahrenheit` - The temperature in Fahrenheit to convert
/// 
/// # Returns
/// 
/// The equivalent temperature in Kelvin
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::fahrenheit_to_kelvin;
/// 
/// let kelvin = fahrenheit_to_kelvin(32.0);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    fahrenheit::to_kelvin(fahrenheit)
}

/// Converts Kelvin to Fahrenheit (legacy function).
/// 
/// **Note:** Consider using `kelvin::to_fahrenheit()` for better organization.
/// 
/// Converts through Celsius as an intermediate step for accuracy.
/// 
/// # Arguments
/// 
/// * `kelvin` - The temperature in Kelvin to convert
/// 
/// # Returns
/// 
/// The equivalent temperature in Fahrenheit
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::kelvin_to_fahrenheit;
/// 
/// let fahrenheit = kelvin_to_fahrenheit(273.15);
/// assert_eq!(fahrenheit, 32.0);
/// ```
pub fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    kelvin::to_fahrenheit(kelvin)
}

/// Converts between any two supported temperature units.
/// 
/// This function handles conversions between Celsius, Fahrenheit, and Kelvin.
/// All conversions use the standard formulas and maintain high precision.
/// 
/// # Supported Units
/// 
/// * **Celsius:** `c`, `celsius`
/// * **Fahrenheit:** `f`, `fahrenheit`  
/// * **Kelvin:** `k`, `kelvin`
/// 
/// Unit names are case-insensitive.
/// 
/// # Arguments
/// 
/// * `value` - The temperature value to convert
/// * `from` - The source temperature unit (case-insensitive)
/// * `to` - The target temperature unit (case-insensitive)
/// 
/// # Returns
/// 
/// * `Ok(f64)` - The converted temperature if both units are recognized
/// * `Err(String)` - An error message if either unit is not supported
/// 
/// # Examples
/// 
/// ```
/// use conversions_rs::convert_temperature;
/// 
/// // Convert freezing point of water
/// let result = convert_temperature(0.0, "C", "F").unwrap();
/// assert_eq!(result, 32.0);
/// 
/// // Convert boiling point of water
/// let result = convert_temperature(212.0, "F", "C").unwrap();
/// assert_eq!(result, 100.0);
/// 
/// // Convert absolute zero
/// let result = convert_temperature(0.0, "K", "C").unwrap();
/// assert_eq!(result, -273.15);
/// 
/// // Case-insensitive units work
/// let result = convert_temperature(25.0, "CELSIUS", "fahrenheit").unwrap();
/// assert_eq!(result, 77.0);
/// 
/// // Error handling for unknown units
/// assert!(convert_temperature(100.0, "invalid", "C").is_err());
/// ```
/// 
/// # Temperature Scale Information
/// 
/// * **Celsius (°C):** Water freezes at 0°C, boils at 100°C at standard pressure
/// * **Fahrenheit (°F):** Water freezes at 32°F, boils at 212°F at standard pressure
/// * **Kelvin (K):** Absolute temperature scale, 0K = absolute zero (-273.15°C)
pub fn convert_temperature(value: f64, from: &str, to: &str) -> Result<f64, String> {
    let result = match (from.to_lowercase().as_str(), to.to_lowercase().as_str()) {
        // Celsius conversions
        ("c" | "celsius", "f" | "fahrenheit") => celsius_to_fahrenheit(value),
        ("c" | "celsius", "k" | "kelvin") => celsius_to_kelvin(value),
        ("c" | "celsius", "c" | "celsius") => value,
        
        // Fahrenheit conversions
        ("f" | "fahrenheit", "c" | "celsius") => fahrenheit_to_celsius(value),
        ("f" | "fahrenheit", "k" | "kelvin") => fahrenheit_to_kelvin(value),
        ("f" | "fahrenheit", "f" | "fahrenheit") => value,
        
        // Kelvin conversions
        ("k" | "kelvin", "c" | "celsius") => kelvin_to_celsius(value),
        ("k" | "kelvin", "f" | "fahrenheit") => kelvin_to_fahrenheit(value),
        ("k" | "kelvin", "k" | "kelvin") => value,
        
        _ => return Err(format!("Unknown temperature conversion: {} to {}", from, to)),
    };

    Ok(result)
}
