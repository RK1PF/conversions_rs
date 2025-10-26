//! Time conversion functions organized by unit type
//! All conversions use seconds as the base unit for accuracy and consistency

/// Seconds conversion functions
pub mod seconds {
    /// Converts seconds to minutes.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_minutes(value: f64) -> f64 {
        value / 60.0
    }

    /// Converts seconds to hours.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_hours(value: f64) -> f64 {
        value / 3600.0
    }

    /// Converts seconds to days.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_days(value: f64) -> f64 {
        value / 86400.0
    }

    /// Converts seconds to weeks.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_weeks(value: f64) -> f64 {
        value / 604800.0
    }

    /// Converts seconds to years (365.25 days).
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_years(value: f64) -> f64 {
        value / 31557600.0 // 365.25 * 24 * 3600
    }

    /// Converts seconds to milliseconds.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts seconds to microseconds.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_microseconds(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts seconds to nanoseconds.
    /// # Arguments
    /// * `value` - The time in seconds to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        value * 1_000_000_000.0
    }
}

/// Minutes conversion functions
pub mod minutes {
    /// Converts minutes to seconds.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_seconds(value: f64) -> f64 {
        value * 60.0
    }

    /// Converts minutes to hours.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_hours(value: f64) -> f64 {
        value / 60.0
    }

    /// Converts minutes to days.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_days(value: f64) -> f64 {
        value / 1440.0
    }

    /// Converts minutes to weeks.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_weeks(value: f64) -> f64 {
        value / 10080.0
    }

    /// Converts minutes to years.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_years(value: f64) -> f64 {
        super::seconds::to_years(to_seconds(value))
    }

    /// Converts minutes to milliseconds.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        super::seconds::to_milliseconds(to_seconds(value))
    }

    /// Converts minutes to microseconds.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_microseconds(value: f64) -> f64 {
        super::seconds::to_microseconds(to_seconds(value))
    }

    /// Converts minutes to nanoseconds.
    /// # Arguments
    /// * `value` - The time in minutes to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        super::seconds::to_nanoseconds(to_seconds(value))
    }
}

/// Hours conversion functions
pub mod hours {
    /// Converts hours to seconds.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_seconds(value: f64) -> f64 {
        value * 3600.0
    }

    /// Converts hours to minutes.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_minutes(value: f64) -> f64 {
        value * 60.0
    }

    /// Converts hours to days.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_days(value: f64) -> f64 {
        value / 24.0
    }

    /// Converts hours to weeks.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_weeks(value: f64) -> f64 {
        value / 168.0
    }

    /// Converts hours to years.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_years(value: f64) -> f64 {
        super::seconds::to_years(to_seconds(value))
    }

    /// Converts hours to milliseconds.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        super::seconds::to_milliseconds(to_seconds(value))
    }

    /// Converts hours to microseconds.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_microseconds(value: f64) -> f64 {
        super::seconds::to_microseconds(to_seconds(value))
    }

    /// Converts hours to nanoseconds.
    /// # Arguments
    /// * `value` - The time in hours to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        super::seconds::to_nanoseconds(to_seconds(value))
    }
}

/// Days conversion functions
pub mod days {
    /// Converts days to seconds.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_seconds(value: f64) -> f64 {
        value * 86400.0
    }

    /// Converts days to minutes.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_minutes(value: f64) -> f64 {
        value * 1440.0
    }

    /// Converts days to hours.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_hours(value: f64) -> f64 {
        value * 24.0
    }

    /// Converts days to weeks.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_weeks(value: f64) -> f64 {
        value / 7.0
    }

    /// Converts days to years.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_years(value: f64) -> f64 {
        value / 365.25
    }

    /// Converts days to milliseconds.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        super::seconds::to_milliseconds(to_seconds(value))
    }

    /// Converts days to microseconds.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_microseconds(value: f64) -> f64 {
        super::seconds::to_microseconds(to_seconds(value))
    }

    /// Converts days to nanoseconds.
    /// # Arguments
    /// * `value` - The time in days to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        super::seconds::to_nanoseconds(to_seconds(value))
    }
}

/// Weeks conversion functions
pub mod weeks {
    /// Converts weeks to seconds.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_seconds(value: f64) -> f64 {
        value * 604800.0
    }

    /// Converts weeks to minutes.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_minutes(value: f64) -> f64 {
        value * 10080.0
    }

    /// Converts weeks to hours.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_hours(value: f64) -> f64 {
        value * 168.0
    }

    /// Converts weeks to days.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_days(value: f64) -> f64 {
        value * 7.0
    }

    /// Converts weeks to years.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_years(value: f64) -> f64 {
        super::days::to_years(to_days(value))
    }

    /// Converts weeks to milliseconds.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        super::seconds::to_milliseconds(to_seconds(value))
    }

    /// Converts weeks to microseconds.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_microseconds(value: f64) -> f64 {
        super::seconds::to_microseconds(to_seconds(value))
    }

    /// Converts weeks to nanoseconds.
    /// # Arguments
    /// * `value` - The time in weeks to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        super::seconds::to_nanoseconds(to_seconds(value))
    }
}

/// Years conversion functions
pub mod years {
    /// Converts years to seconds.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_seconds(value: f64) -> f64 {
        value * 31557600.0 // 365.25 * 24 * 3600
    }

    /// Converts years to minutes.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_minutes(value: f64) -> f64 {
        super::seconds::to_minutes(to_seconds(value))
    }

    /// Converts years to hours.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_hours(value: f64) -> f64 {
        super::seconds::to_hours(to_seconds(value))
    }

    /// Converts years to days.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_days(value: f64) -> f64 {
        value * 365.25
    }

    /// Converts years to weeks.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_weeks(value: f64) -> f64 {
        super::days::to_weeks(to_days(value))
    }

    /// Converts years to milliseconds.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        super::seconds::to_milliseconds(to_seconds(value))
    }

    /// Converts years to microseconds.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_microseconds(value: f64) -> f64 {
        super::seconds::to_microseconds(to_seconds(value))
    }

    /// Converts years to nanoseconds.
    /// # Arguments
    /// * `value` - The time in years to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        super::seconds::to_nanoseconds(to_seconds(value))
    }
}

/// Milliseconds conversion functions
pub mod milliseconds {
    /// Converts milliseconds to seconds.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_seconds(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts milliseconds to minutes.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_minutes(value: f64) -> f64 {
        super::seconds::to_minutes(to_seconds(value))
    }

    /// Converts milliseconds to hours.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_hours(value: f64) -> f64 {
        super::seconds::to_hours(to_seconds(value))
    }

    /// Converts milliseconds to days.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_days(value: f64) -> f64 {
        super::seconds::to_days(to_seconds(value))
    }

    /// Converts milliseconds to weeks.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_weeks(value: f64) -> f64 {
        super::seconds::to_weeks(to_seconds(value))
    }

    /// Converts milliseconds to years.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_years(value: f64) -> f64 {
        super::seconds::to_years(to_seconds(value))
    }

    /// Converts milliseconds to microseconds.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_microseconds(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts milliseconds to nanoseconds.
    /// # Arguments
    /// * `value` - The time in milliseconds to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        value * 1_000_000.0
    }
}

/// Microseconds conversion functions
pub mod microseconds {
    /// Converts microseconds to seconds.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_seconds(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts microseconds to minutes.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_minutes(value: f64) -> f64 {
        super::seconds::to_minutes(to_seconds(value))
    }

    /// Converts microseconds to hours.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_hours(value: f64) -> f64 {
        super::seconds::to_hours(to_seconds(value))
    }

    /// Converts microseconds to days.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_days(value: f64) -> f64 {
        super::seconds::to_days(to_seconds(value))
    }

    /// Converts microseconds to weeks.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_weeks(value: f64) -> f64 {
        super::seconds::to_weeks(to_seconds(value))
    }

    /// Converts microseconds to years.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_years(value: f64) -> f64 {
        super::seconds::to_years(to_seconds(value))
    }

    /// Converts microseconds to milliseconds.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts microseconds to nanoseconds.
    /// # Arguments
    /// * `value` - The time in microseconds to convert
    pub fn to_nanoseconds(value: f64) -> f64 {
        value * 1000.0
    }
}

/// Nanoseconds conversion functions
pub mod nanoseconds {
    /// Converts nanoseconds to seconds.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_seconds(value: f64) -> f64 {
        value / 1_000_000_000.0
    }

    /// Converts nanoseconds to minutes.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_minutes(value: f64) -> f64 {
        super::seconds::to_minutes(to_seconds(value))
    }

    /// Converts nanoseconds to hours.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_hours(value: f64) -> f64 {
        super::seconds::to_hours(to_seconds(value))
    }

    /// Converts nanoseconds to days.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_days(value: f64) -> f64 {
        super::seconds::to_days(to_seconds(value))
    }

    /// Converts nanoseconds to weeks.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_weeks(value: f64) -> f64 {
        super::seconds::to_weeks(to_seconds(value))
    }

    /// Converts nanoseconds to years.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_years(value: f64) -> f64 {
        super::seconds::to_years(to_seconds(value))
    }

    /// Converts nanoseconds to milliseconds.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_milliseconds(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts nanoseconds to microseconds.
    /// # Arguments
    /// * `value` - The time in nanoseconds to convert
    pub fn to_microseconds(value: f64) -> f64 {
        value / 1000.0
    }
}

// Legacy function wrappers for backward compatibility
pub fn seconds_to_minutes(seconds: f64) -> f64 {
    seconds::to_minutes(seconds)
}

pub fn minutes_to_seconds(minutes: f64) -> f64 {
    minutes::to_seconds(minutes)
}

pub fn hours_to_seconds(hours: f64) -> f64 {
    hours::to_seconds(hours)
}

pub fn seconds_to_hours(seconds: f64) -> f64 {
    seconds::to_hours(seconds)
}

pub fn days_to_hours(days: f64) -> f64 {
    days::to_hours(days)
}

pub fn hours_to_days(hours: f64) -> f64 {
    hours::to_days(hours)
}

pub fn weeks_to_days(weeks: f64) -> f64 {
    weeks::to_days(weeks)
}

pub fn days_to_weeks(days: f64) -> f64 {
    days::to_weeks(days)
}

pub fn years_to_days(years: f64) -> f64 {
    years::to_days(years)
}

pub fn days_to_years(days: f64) -> f64 {
    days::to_years(days)
}

pub fn milliseconds_to_seconds(ms: f64) -> f64 {
    milliseconds::to_seconds(ms)
}

pub fn seconds_to_milliseconds(seconds: f64) -> f64 {
    seconds::to_milliseconds(seconds)
}

pub fn microseconds_to_seconds(us: f64) -> f64 {
    microseconds::to_seconds(us)
}

pub fn seconds_to_microseconds(seconds: f64) -> f64 {
    seconds::to_microseconds(seconds)
}

pub fn nanoseconds_to_seconds(ns: f64) -> f64 {
    nanoseconds::to_seconds(ns)
}

pub fn seconds_to_nanoseconds(seconds: f64) -> f64 {
    seconds::to_nanoseconds(seconds)
}

/// General time conversion function that accepts string unit names
///
/// Converts a time value from one unit to another using string identifiers.
/// This function is case-insensitive and supports common abbreviations.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from_unit` - The source unit (e.g., "s", "min", "h", "day", "week", "year", "ms", "us", "ns")
/// * `to_unit` - The target unit using the same abbreviations
///
/// # Returns
/// * `Ok(f64)` - The converted value
/// * `Err(String)` - Error message if the conversion is not supported
///
/// # Examples
///
/// ```rust
/// use conversions_rs::convert_time;
///
/// let minutes = convert_time(3600.0, "s", "min").unwrap();
/// assert_eq!(minutes, 60.0);
///
/// let milliseconds = convert_time(2.5, "s", "ms").unwrap();
/// assert_eq!(milliseconds, 2500.0);
/// ```
pub fn convert_time(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();

    // Convert input to seconds first
    let seconds = match from_unit.as_str() {
        "s" | "sec" | "second" | "seconds" => value,
        "min" | "minute" | "minutes" => minutes::to_seconds(value),
        "h" | "hr" | "hour" | "hours" => hours::to_seconds(value),
        "d" | "day" | "days" => days::to_seconds(value),
        "w" | "week" | "weeks" => weeks::to_seconds(value),
        "y" | "year" | "years" => years::to_seconds(value),
        "ms" | "millisecond" | "milliseconds" => milliseconds::to_seconds(value),
        "us" | "μs" | "microsecond" | "microseconds" => microseconds::to_seconds(value),
        "ns" | "nanosecond" | "nanoseconds" => nanoseconds::to_seconds(value),
        _ => return Err(format!("Unsupported time unit: {}", from_unit)),
    };

    // Convert seconds to target unit
    let result = match to_unit.as_str() {
        "s" | "sec" | "second" | "seconds" => seconds,
        "min" | "minute" | "minutes" => seconds::to_minutes(seconds),
        "h" | "hr" | "hour" | "hours" => seconds::to_hours(seconds),
        "d" | "day" | "days" => seconds::to_days(seconds),
        "w" | "week" | "weeks" => seconds::to_weeks(seconds),
        "y" | "year" | "years" => seconds::to_years(seconds),
        "ms" | "millisecond" | "milliseconds" => seconds::to_milliseconds(seconds),
        "us" | "μs" | "microsecond" | "microseconds" => seconds::to_microseconds(seconds),
        "ns" | "nanosecond" | "nanoseconds" => seconds::to_nanoseconds(seconds),
        _ => return Err(format!("Unsupported time unit: {}", to_unit)),
    };

    Ok(result)
}
