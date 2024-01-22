use std::fs::File;

fn main() {
    // Ques-1: Use pattern matching to match a tuple (x, y) where x is an integer and y is a character.
    // Print "Matched!" if the tuple matches, and "Not Matched!" otherwise.
    let my_struct = tuple { x: 42, y: 'A' };

    match my_struct {
        tuple { x, y } if x > 0 && y.is_alphabetic() => {
            println!("Matched!");
        }
        _ => {
            println!("Not Matched!");
        }
    }

    // Ques-2: Define a Person struct with fields for name and age. Create an instance of the struct and
    // print the person's details.
    let person = Person {
        name: String::from("Ajay"),
        age: 43,
    };
    println!(
        "The person's name is {} and their age is {}",
        person.name, person.age
    );

    // Ques-3: Create an enum named Color with variants representing different colors (e.g., Red, Green, Blue).
    let color1 = Color::red;
    let color2 = Color::green;
    let color3 = Color::blue;

    println!(
        "The first color is {:?}, second color is {:?}, third color is {:?}",
        color1, color2, color3
    );

    // Ques-4: Write a function that takes a name and an age as parameters and returns an instance of the Person struct.
    let person_1 = create_person("Rohan", 12);
    println!(
        "The person's name is {} and their age is {}.",
        person_1.name, person_1.age
    );

    // Ques-4: Implement a function that takes a Color enum variant and returns a corresponding hexadecimal color code as a String.
    let red_code = enum_func(Color::red);
    let green_code = enum_func(Color::green);
    let blue_code = enum_func(Color::blue);
    println!(
        "The red colour is {}, green colour is {}, and blue colour is {}.",
        red_code, green_code, blue_code
    );

    // Ques-5: Define a tuple struct named Point that represents a point in 2D space with x and y coordinates (both f64).
    let my_tuple = Point(23.9, 65.2);
    println!(
        "The value of x-coordinate is {}, and the value of y-coordinate is {}.",
        my_tuple.0, my_tuple.1
    );

    // Ques-6: Create a function that divides two numbers and returns a Result with the result of the division or an error message if the divisor is zero.
    let test_case_1 = division_two_nums(12.3, 87.1);
    match test_case_1 {
        Ok(result) => println!("The result is {}", result),
        Err(error) => println!("The error is {}", error),
    }
    let test_case_2 = division_two_nums(33.5, 67.5);
    match test_case_2 {
        Ok(result) => println!("The result is {}", result),
        Err(error) => println!("The error is {}", error),
    }

    // Ques-7: Write a function that takes an optional string (Option<String>) and prints "Length: <length>" if the string is present, or "No string provided" otherwise.
    let string_1 = Some(String::from("Hello world!"));
    let string_2 = None;
    string_length(string_1);
    string_length(string_2);

    // Ques-8: Define a custom error type named FileError with variants representing different file-related errors (e.g., NotFound, PermissionDenied).
    let file_path = "helllo.txt";

    match open_file(file_path) {
        Ok(()) => print!("File opened successfully"),
        Err(err) => match err {
            FileErrors::NotFound => print!("File not found!"),
            FileErrors::PermissionDenied => print!("Permission denied to open the file!"),
            FileErrors::IOError(io_err) => {
                print!("IOError: {:?}", io_err);
            }
            FileErrors::CustomError(custom_msg) => {
                print!("Custom Error: {}", custom_msg);
            }
        },
    }

    // Ques-9: Implement a function that takes an Option<i32> and prints "Some: <value>" if it contains a value, or "None" if it is empty.
    let some_value = Some(42);
    let none_value = None;

    print_option_value(some_value);
    print_option_value(none_value);

    // Ques-10: Define an enum named Direction with variants representing directions (North, South, East, West). Write a function that takes a Direction enum variant and prints a corresponding message.
    let direction = Direction::East;
    print_direction(direction);

    // Ques-11: Create a tuple (u8, u8) representing coordinates (x, y). Write a function that takes this tuple and prints "Quadrant I," "Quadrant II," etc., based on the quadrant the coordinates fall into.
    let my_tuple = (1, 2);
    print_quadrant(my_tuple);

    // Ques-12: Write a function that takes an integer and prints "Positive," "Negative," or "Zero" based on the value.
    let my_num = 7;
    print_integer_value(my_num);

    // Ques-13: Define a struct named Person with fields for name (String) and age (u32). Write a function that takes a Person and prints different messages based on the age.
    let new_person = Person_w {
        age: 32,
        name: String::from("Akshay"),
    };

    age_struct(new_person);
}

struct tuple {
    x: i32,
    y: char,
}

struct Person {
    name: String,
    age: i32,
}

#[derive(Debug)]
enum Color {
    red,
    green,
    blue,
}

fn create_person(name: &str, age: i32) -> Person {
    // Creating an instance of the Person struct
    Person {
        name: String::from(name),
        age,
    }
}

fn enum_func(color: Color) -> String {
    match color {
        Color::red => String::from("#FF0000"),
        Color::green => String::from("#00FF00"),
        Color::blue => String::from("#0000FF"),
    }
}

struct Point(f64, f64);

fn division_two_nums(num1: f64, num2: f64) -> Result<f64, String> {
    if num2 != 0.0 {
        Ok(num1 / num2)
    } else {
        Err(String::from("There's an error"))
    }
}

fn string_length(string: Option<String>) {
    match string {
        Some(s) => {
            let length = s.len();
            println!("The length of the string is: {}", length);
        }
        None => println!("No string present."),
    }
}

enum FileErrors {
    NotFound,
    PermissionDenied,
    CustomError(String),
    IOError(std::io::Error),
}

fn open_file(file_path: &str) -> Result<(), FileErrors> {
    Err(FileErrors::NotFound)
}

fn print_option_value(option_value: Option<i32>) {
    match option_value {
        Some(value) => println!("Some: {}", value),
        None => println!("None"),
    }
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn print_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
    }
}

fn print_quadrant(coordinates: (u8, u8)) {
    match coordinates {
        (x, y) if x > 0 && y > 0 => println!("Quadrant 1"),
        (x, y) if x < 0 && y > 0 => println!("Quadrant 2"),
        (x, y) if x < 0 && y < 0 => println!("Quadrant 3"),
        (x, y) if x > 0 && y < 0 => println!("Quadrant 4"),
        _ => println!("On the axis or at origin"),
    }
}

fn print_integer_value(number: i32) {
    match num {
        n if n > 0 => println!("Number is positive."),
        n if n < 0 => println!("Number is negative."),
        _ => println!("Zero it is."),
    }
}

struct Person_w {
    age: u32,
    name: String,
}

fn age_struct(person: Person_w) {
    match person.age {
        age if age < 18 => println!("{} is a minor.", person.age),
        age if age > 18 => println!("{} is a major.", person.age),
        _ => println!("{} is a senior citizen.", person.name),
    }
}
