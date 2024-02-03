use std::fs;
use std::io;
use std::num;
fn main() {
    let my_error = RocketError::AlienInvasion;
    handle_error(my_error);

    match read_and_parse("test.txt") {
        Ok(num) => println!("Good to go."),
        Err(err) => println!("An error occured {}", err),
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("You can't divide this by 0.")
    } else {
        Ok(numerator / denominator)
    }
}

enum RocketError {
    OutOfFuel,
    NavigationSystemError,
    AlienInvasion,
}

fn handle_error(error: RocketError) {
    match error {
        RocketError::OutOfFuel => {
            println!("Get Fuel")
        }
        RocketError::AlienInvasion => {
            println!("Alient Invasion")
        }
        RocketError::NavigationSystemError => {
            println!("There's some navigation issue")
        }
    }
}

enum MyCustomError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Other(String),
}

use std::fmt;

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f, "I/O Error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse Error: {}", err),
            MyCustomError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

impl std::error::Error for MyCustomError {}

fn read_and_parse(filename: &str) -> Result<i32, MyCustomError> {
    let content = fs::read_to_string(filename).map_err(MyCustomError::Io)?;
    let num: i32 = content.trim().parse().map_err(MyCustomError::Parse)?;
    Ok(num)
}
