use std::io;

fn main() {
    hello();
    convert();
    goodbye();
}

fn hello() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("{NAME} v{VERSION}");
    println!();
}

enum Unit {
    F,
    C,
    None,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::F => write!(f, "F"),
            Unit::C => write!(f, "C"),
            Unit::None => write!(f, "None"),
        }
    }
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn report(temp: f32, unit: Unit) {
    println!();
    match unit {
        Unit::F => {
            let other_unit = Unit::C;
            let converted = f_to_c(temp);
            println!("{temp}°{unit} -> {converted}°{other_unit}");
        },
        Unit::C => {
            let other_unit = Unit::F;
            let converted = c_to_f(temp);
            println!("{temp}°{unit} -> {converted}°{other_unit}");
        },
        Unit::None => {
            println!("Invalid unit. Unable to convert.");
        },
    };
    println!();
}

fn convert() {
    loop {
        println!("Enter a temperature to convert or nothing to exit:");

        // Read the line
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // Parse and validate the input
        let words: Vec<&str> = input.split_whitespace().collect();
        const MAX_TOKENS: usize = 2;
        if words.len() > MAX_TOKENS {
            println!("Expected {MAX_TOKENS} tokens or less.");
            continue;
        } else if words.len() == 0 {
            break;
        }

        // Try to convert the temperature
        let temp = match words.get(0) {
            Some(temp) => String::from(*temp),
            None => continue,
        };
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("'{temp}' is not a number. Try again.");
                continue;
            }
        };

        // Get the unit
        let mut input = match words.get(1) {
            Some(input) => String::from(*input),
            None => {
                println!("No unit provided. Please specify Fahrenheit or Celsius.");
    
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line.");
                input
            }
        };

        // Try to convert the input to a unit
        // If the unit is 'None', keep asking for a unit
        let unit = loop {
            let unit = input.to_lowercase();
            let unit = if unit.starts_with('f') {
                Unit::F
            } else if unit.starts_with('c') {
                Unit::C
            } else {
                Unit::None
            };
            
            if !matches!(unit, Unit::None) {
                break unit;
            }

            println!("Invalid unit. Please specify Fahrenheit or Celsius.");

            input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
        };
        
        // Convert the temperature and report the results
        report(temp, unit);
    }
}

fn goodbye() {
    println!("Goodbye!");
}
