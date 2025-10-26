mod conversions;

use std::io::{self, Write};
use clap::{Parser, Subcommand};
use conversions::*;

#[derive(Parser)]
#[command(name = "conversions_rs")]
#[command(about = "A comprehensive unit conversion tool")]
#[command(version = "0.1.0")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert length units
    Length {
        /// Value to convert
        value: f64,
        /// Source unit (m, km, cm, mm, ft, in, yd, mi)
        from: String,
        /// Target unit (m, km, cm, mm, ft, in, yd, mi)
        to: String,
    },
    /// Convert weight/mass units
    Weight {
        /// Value to convert
        value: f64,
        /// Source unit (kg, g, lb, oz, t, st)
        from: String,
        /// Target unit (kg, g, lb, oz, t, st)
        to: String,
    },
    /// Convert temperature units
    Temperature {
        /// Value to convert
        value: f64,
        /// Source unit (C, F, K)
        from: String,
        /// Target unit (C, F, K)
        to: String,
    },
    /// Convert volume units
    Volume {
        /// Value to convert
        value: f64,
        /// Source unit (l, ml, gal, gal_uk, fl_oz, fl_oz_uk, cup, pt, qt)
        from: String,
        /// Target unit (l, ml, gal, gal_uk, fl_oz, fl_oz_uk, cup, pt, qt)
        to: String,
    },
    /// Convert time units
    Time {
        /// Value to convert
        value: f64,
        /// Source unit (s, min, h, day, week, year, ms, μs, ns)
        from: String,
        /// Target unit (s, min, h, day, week, year, ms, μs, ns)
        to: String,
    },
    /// Convert electric current units
    Current {
        /// Value to convert
        value: f64,
        /// Source unit (A, mA, μA, nA, kA)
        from: String,
        /// Target unit (A, mA, μA, nA, kA)
        to: String,
    },
    /// Convert amount of substance units
    Amount {
        /// Value to convert
        value: f64,
        /// Source unit (mol, mmol, μmol, nmol, pmol, kmol)
        from: String,
        /// Target unit (mol, mmol, μmol, nmol, pmol, kmol)
        to: String,
    },
    /// Convert luminous intensity units
    Luminosity {
        /// Value to convert
        value: f64,
        /// Source unit (cd, mcd, kcd, hk, ic, dc)
        from: String,
        /// Target unit (cd, mcd, kcd, hk, ic, dc)
        to: String,
    },
    /// Convert area units
    Area {
        /// Value to convert
        value: f64,
        /// Source unit (m², cm², km², ft², in², ac, ha, mi²)
        from: String,
        /// Target unit (m², cm², km², ft², in², ac, ha, mi²)
        to: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(command) => handle_cli_command(command),
        None => run_interactive_mode(),
    }
}

fn handle_cli_command(command: Commands) {
    let result = match command {
        Commands::Length { value, from, to } => {
            match convert_length(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Weight { value, from, to } => {
            match convert_weight(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Temperature { value, from, to } => {
            match convert_temperature(value, &from, &to) {
                Ok(result) => {
                    println!("{}°{} = {:.2}°{}", value, from.to_uppercase(), result, to.to_uppercase());
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Volume { value, from, to } => {
            match convert_volume(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Time { value, from, to } => {
            match convert_time(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Current { value, from, to } => {
            match convert_current(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Amount { value, from, to } => {
            match convert_amount(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Luminosity { value, from, to } => {
            match convert_luminous_intensity(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
        Commands::Area { value, from, to } => {
            match convert_area(value, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.6} {}", value, from, result, to);
                    return;
                }
                Err(error) => error,
            }
        }
    };

    eprintln!("Error: {}", result);
    std::process::exit(1);
}

fn run_interactive_mode() {
    println!("🔄 Unit Conversion App");
    println!("======================");
    
    loop {
        display_menu();
        
        print!("\nEnter your choice (1-9): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => handle_length_conversion(),
            "2" => handle_weight_conversion(),
            "3" => handle_temperature_conversion(),
            "4" => handle_volume_conversion(),
            "5" => handle_time_conversion(),
            "6" => handle_current_conversion(),
            "7" => handle_amount_conversion(),
            "8" => handle_luminosity_conversion(),
            "9" => handle_area_conversion(),
            "0" => {
                println!("Thanks for using the Unit Conversion App! 👋");
                break;
            }
            _ => println!("❌ Invalid choice. Please select 0-9."),
        }
        
        println!("\n{}", "-".repeat(50));
    }
}

fn display_menu() {
    println!("\nChoose conversion type:");
    println!("1. 📏 Length");
    println!("2. ⚖️  Weight/Mass");
    println!("3. 🌡️  Temperature");
    println!("4. 🧪 Volume");
    println!("5. ⏱️  Time");
    println!("6. ⚡ Electric Current");
    println!("7. 🧬 Amount of Substance");
    println!("8. 💡 Luminous Intensity");
    println!("9. 📐 Area");
    println!("0. 🚪 Exit");
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_number(prompt: &str) -> f64 {
    loop {
        let input = get_input(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("❌ Please enter a valid number."),
        }
    }
}

fn handle_length_conversion() {
    println!("\n📏 Length Conversion");
    println!("Supported units: m, km, cm, mm, ft, in, yd, mi");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_length(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_weight_conversion() {
    println!("\n⚖️  Weight/Mass Conversion");
    println!("Supported units: kg, g, lb, oz, t, st");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_weight(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_temperature_conversion() {
    println!("\n🌡️  Temperature Conversion");
    println!("Supported units: C (Celsius), F (Fahrenheit), K (Kelvin)");
    
    let value = get_number("Enter the temperature to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_temperature(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {}°{} = {:.2}°{}", value, from_unit.to_uppercase(), result, to_unit.to_uppercase());
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_volume_conversion() {
    println!("\n🧪 Volume Conversion");
    println!("Supported units: l, ml, gal, gal_uk, fl_oz, fl_oz_uk, cup, pt, qt");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_volume(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_time_conversion() {
    println!("\n⏱️  Time Conversion");
    println!("Supported units: s, min, h, day, week, year, ms, μs, ns");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_time(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_current_conversion() {
    println!("\n⚡ Electric Current Conversion");
    println!("Supported units: A, mA, μA, nA, kA");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_current(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_amount_conversion() {
    println!("\n🧬 Amount of Substance Conversion");
    println!("Supported units: mol, mmol, μmol, nmol, pmol, kmol");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_amount(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_luminosity_conversion() {
    println!("\n💡 Luminous Intensity Conversion");
    println!("Supported units: cd, mcd, kcd, hk, ic, dc");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_luminous_intensity(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}

fn handle_area_conversion() {
    println!("\n📐 Area Conversion");
    println!("Supported units: m², cm², km², ft², in², ac, ha, mi²");
    
    let value = get_number("Enter the value to convert: ");
    let from_unit = get_input("From unit: ");
    let to_unit = get_input("To unit: ");
    
    match convert_area(value, &from_unit, &to_unit) {
        Ok(result) => {
            println!("✅ {} {} = {:.6} {}", value, from_unit, result, to_unit);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }
}
