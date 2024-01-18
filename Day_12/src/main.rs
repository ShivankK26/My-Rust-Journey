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
