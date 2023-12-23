fn main() {
    let my_string = String::from("Hello, world!");

    let my_ref = &my_string; // points to the reference of the variable my_string

    println!("My reference is: {}", my_ref);


    // Calling print_string function
    let my_string = String::from("Hello, world!");
    print_string(&my_string);

    println!("I still got my string {}", my_string);


    // Calling the change_string function
    let mut my_string = String::from("Hello");

    change_string(&mut my_string);

    println!("{}", my_string);

    let first_immutable_Reference = &my_string;
    let second_immutable_reference = &my_string;

    println!(
        "First immutable string is {}, second immutable string is {}",
        first_immutable_Reference, second_immutable_reference
    );

    let first_mutable_Reference = &mut my_string;
    println!("First mutable reference value {}", first_mutable_Reference);


    // Calling the return_reference function
    let new_string = String::from("new string");
    let new_string_ref = return_reference(&new_string);

    println!("new string is {}", new_string);

    // Again calling the new_string function
    let newer_string = new_string;

    println!("new string reference is {}", new_string_ref);
}

fn print_string(s: &String) {
    println!("{}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" world"); // used for adding strings or combining them
}

fn return_reference(some_string: &String) -> {
    some_string
}