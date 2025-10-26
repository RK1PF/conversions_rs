/// Example demonstrating the comprehensive modular API structure
///
/// This example shows how to use the unit conversion library with the new
/// modular API that organizes functions by unit type, including all SI base
/// and derived units for complete scientific unit conversion support.
use conversions_rs::{
    area, current, length, luminous_intensity, substance, temperature, time, volume, weight,
};

fn main() {
    println!("🔧 Unit Conversion Library - Comprehensive SI Units Demo\n");

    // Length conversions using the modular API
    println!("📏 Length Conversions:");
    println!("100 meters = {:.2} feet", length::meters::to_feet(100.0));
    println!("6 feet = {:.2} meters", length::feet::to_meters(6.0));
    println!(
        "1 mile = {:.2} kilometers",
        length::miles::to_kilometers(1.0)
    );
    println!(
        "50 centimeters = {:.1} inches",
        length::centimeters::to_inches(50.0)
    );
    println!();

    // Weight conversions using the modular API
    println!("⚖️  Weight Conversions:");
    println!(
        "10 kilograms = {:.2} pounds",
        weight::kilograms::to_pounds(10.0)
    );
    println!(
        "150 pounds = {:.2} kilograms",
        weight::pounds::to_kilograms(150.0)
    );
    println!("1 ton = {:.0} kilograms", weight::tons::to_kilograms(1.0));
    println!("500 grams = {:.1} ounces", weight::grams::to_ounces(500.0));
    println!();

    // Temperature conversions using the modular API
    println!("🌡️  Temperature Conversions:");
    println!("25°C = {:.1}°F", temperature::celsius::to_fahrenheit(25.0));
    println!("77°F = {:.1}°C", temperature::fahrenheit::to_celsius(77.0));
    println!("0°C = {:.1}K", temperature::celsius::to_kelvin(0.0));
    println!("298.15K = {:.1}°C", temperature::kelvin::to_celsius(298.15));
    println!();

    // Volume conversions using the modular API
    println!("🧪 Volume Conversions:");
    println!(
        "100 liters = {:.2} US gallons",
        volume::liters::to_gallons_us(100.0)
    );
    println!(
        "5 US gallons = {:.2} liters",
        volume::gallons_us::to_liters(5.0)
    );
    println!(
        "1 liter = {:.0} milliliters",
        volume::liters::to_milliliters(1.0)
    );
    println!(
        "2 cups = {:.0} milliliters",
        volume::cups_us::to_milliliters(2.0)
    );
    println!();

    // Time conversions using the modular API (NEW SI UNIT)
    println!("⏱️  Time Conversions:");
    println!(
        "3600 seconds = {:.0} minutes",
        time::seconds::to_minutes(3600.0)
    );
    println!("2 hours = {:.0} minutes", time::hours::to_minutes(2.0));
    println!("7 days = {:.0} hours", time::days::to_hours(7.0));
    println!(
        "1000 milliseconds = {:.0} seconds",
        time::milliseconds::to_seconds(1000.0)
    );
    println!("2 weeks = {:.0} days", time::weeks::to_days(2.0));
    println!();

    // Electric current conversions using the modular API (NEW SI UNIT)
    println!("⚡ Electric Current Conversions:");
    println!(
        "1.5 amperes = {:.0} milliamperes",
        current::amperes::to_milliamperes(1.5)
    );
    println!(
        "500 milliamperes = {:.1} amperes",
        current::milliamperes::to_amperes(500.0)
    );
    println!(
        "1 ampere = {:.0} microamperes",
        current::amperes::to_microamperes(1.0)
    );
    println!(
        "2 kiloamperes = {:.0} amperes",
        current::kiloamperes::to_amperes(2.0)
    );
    println!();

    // Amount of substance conversions using the modular API (NEW SI UNIT)
    println!("🧬 Amount of Substance Conversions:");
    println!(
        "0.5 moles = {:.0} millimoles",
        substance::moles::to_millimoles(0.5)
    );
    println!(
        "250 millimoles = {:.2} moles",
        substance::millimoles::to_moles(250.0)
    );
    println!(
        "1 mole = {:.0} micromoles",
        substance::moles::to_micromoles(1.0)
    );
    println!(
        "2 kilomoles = {:.0} moles",
        substance::kilomoles::to_moles(2.0)
    );
    println!();

    // Luminous intensity conversions using the modular API (NEW SI UNIT)
    println!("💡 Luminous Intensity Conversions:");
    println!(
        "2.5 candela = {:.0} millicandela",
        luminous_intensity::candela::to_millicandela(2.5)
    );
    println!(
        "1500 millicandela = {:.1} candela",
        luminous_intensity::millicandela::to_candela(1500.0)
    );
    println!(
        "1 candela = {:.2} hefnerkerze",
        luminous_intensity::candela::to_hefnerkerze(1.0)
    );
    println!(
        "0.5 kilocandela = {:.0} candela",
        luminous_intensity::kilocandela::to_candela(0.5)
    );
    println!();

    // Area conversions using the modular API (NEW SI DERIVED UNIT)
    println!("📐 Area Conversions:");
    println!(
        "10000 square meters = {:.1} hectares",
        area::square_meters::to_hectares(10000.0)
    );
    println!("1 hectare = {:.2} acres", area::hectares::to_acres(1.0));
    println!(
        "1 square kilometer = {:.0} square meters",
        area::square_kilometers::to_square_meters(1.0)
    );
    println!(
        "144 square inches = {:.0} square feet",
        area::square_inches::to_square_feet(144.0)
    );
    println!(
        "1 acre = {:.0} square meters",
        area::acres::to_square_meters(1.0)
    );
    println!();

    println!("✨ Benefits of the comprehensive modular API:");
    println!("• Complete SI unit system support (base + derived units)");
    println!("• Better code organization by physical quantity");
    println!("• Improved discoverability with IDE autocomplete");
    println!("• Clearer function names (unit::to_other_unit)");
    println!("• Scientific precision with proper conversion factors");
    println!("• Backwards compatible with legacy functions");
    println!("• Supports both metric and imperial unit systems");
}
