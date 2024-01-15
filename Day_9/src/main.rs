fn main() {
    // enum Option<T>{
    //     Some(T),
    //     None
    // }

    // Using find_square_root function
    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is {}", number, value),
        None => println!("The square root of {} is not a real number.", number),
    }

    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E),
    // }

    // Using divide function
    let a = 10.0;
    let b = 0.0;
    let divison_result = divide(a, b);

    match division_result {
        Ok(value) => println!("{} divided by {} is {}", a, b, value),
        Err(error_message) => println!("Error {}", error_message),
    }

    // Using the calculate_triangle_area function
    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_triangle_area(base, height);

    match area_result {
        Ok(area) => println!("The area of the triangle is {} square units.", area),
        Err(error_message) => println!("Error {}", error_message),
    }
}

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by 0 is no allowed.".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_from_database(key: &str) -> Option<f64> {
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(6.0))];

    for (k, v) in database {
        if k == key {
            return v;
        }
    }
    None
}

fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive.".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        (None, _) => Err("The base is missing".to_string()),
        (_, None) => Err("The height is missing".to_string()),
    }
}
