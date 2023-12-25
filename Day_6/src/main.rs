fn main() {
    let original_string = String::from("Hello, world!");
    let cloned_string = original_string.clone(); // it clones the same string

    println!("Original string: {}", original_string);
    println!("Cloned string: {}", cloned_string);

    // Calling the modify_String function here
    let original_string = String::from("String");
    let modified_string = modify_String(&original_string);

    println!("Original string: {}", original_string);
    println!("Modified string: {}", modified_string)
}

fn modify_String(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str(" modified");
    return cloned_string;
}
