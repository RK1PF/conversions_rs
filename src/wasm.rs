//! WebAssembly bindings for the conversions_rs library.
//!
//! This module provides JavaScript-friendly APIs for unit conversions
//! when compiled to WebAssembly.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::conversions::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    // Bind the `console.log` function from the Web API
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Result type for WASM conversion operations
#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[derive(Clone)]
pub struct ConversionResult {
    success: bool,
    value: f64,
    error: Option<String>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ConversionResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

/// Convert length units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_length_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_length(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert weight/mass units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_weight_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_weight(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert temperature units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_temperature_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_temperature(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert volume units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_volume_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_volume(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert time units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_time_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_time(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert electric current units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_current_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_current(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert amount of substance units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_substance_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_amount(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert luminous intensity units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_luminous_intensity_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_luminous_intensity(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Convert area units
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_area_wasm(value: f64, from: &str, to: &str) -> ConversionResult {
    match convert_area(value, from, to) {
        Ok(result) => ConversionResult {
            success: true,
            value: result,
            error: None,
        },
        Err(e) => ConversionResult {
            success: false,
            value: 0.0,
            error: Some(e.to_string()),
        },
    }
}

/// Initialize the WASM module
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("conversions_rs WASM module initialized");
}

/// Get supported units for a given conversion type
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_supported_units(conversion_type: &str) -> Vec<String> {
    match conversion_type.to_lowercase().as_str() {
        "length" => vec![
            "m".to_string(), "km".to_string(), "cm".to_string(), "mm".to_string(),
            "ft".to_string(), "in".to_string(), "yd".to_string(), "mi".to_string(),
            "nm".to_string(), "μm".to_string(),
        ],
        "weight" | "mass" => vec![
            "kg".to_string(), "g".to_string(), "mg".to_string(), "lb".to_string(),
            "oz".to_string(), "t".to_string(), "st".to_string(), "grain".to_string(),
            "carat".to_string(),
        ],
        "temperature" => vec![
            "C".to_string(), "F".to_string(), "K".to_string(),
        ],
        "volume" => vec![
            "l".to_string(), "ml".to_string(), "gal".to_string(), "gal_uk".to_string(),
            "fl_oz".to_string(), "fl_oz_uk".to_string(), "cup".to_string(),
            "pt".to_string(), "qt".to_string(), "tbsp".to_string(), "tsp".to_string(),
        ],
        "time" => vec![
            "s".to_string(), "min".to_string(), "h".to_string(), "d".to_string(),
            "week".to_string(), "month".to_string(), "year".to_string(),
            "ms".to_string(), "μs".to_string(), "ns".to_string(),
        ],
        "current" => vec![
            "A".to_string(), "mA".to_string(), "μA".to_string(), "nA".to_string(),
            "kA".to_string(), "MA".to_string(),
        ],
        "substance" => vec![
            "mol".to_string(), "mmol".to_string(), "μmol".to_string(), "nmol".to_string(),
            "kmol".to_string(),
        ],
        "luminous_intensity" => vec![
            "cd".to_string(), "mcd".to_string(), "kcd".to_string(),
        ],
        "area" => vec![
            "m²".to_string(), "km²".to_string(), "cm²".to_string(), "mm²".to_string(),
            "ha".to_string(), "acre".to_string(), "ft²".to_string(), "in²".to_string(),
            "yd²".to_string(), "mi²".to_string(),
        ],
        _ => vec![],
    }
}