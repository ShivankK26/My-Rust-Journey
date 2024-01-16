use std::collections::HashMap;

fn main() {
    // Creating vector collections in Rust
    let mut _numbers = vec![1, 2, 3, 4];

    let mut names: Vec<String> = Vec::new();

    names.push(String::from("Alice")); // [Alice]
    names.push(String::from("Bob")); // [Alice, Bob]

    let first_name = &names[0];
    let second_name = &names[1];
    println!(
        "First name is {}, second name is {}",
        first_name, second_name
    );

    names.pop(); // [Alice]
    println!("");

    for number in _numbers {
        println!("The number is {}", number);
    }

    let slice = &_numbers[1..3]; // [2,3]

    // Creating string collections in Rust
    let mut my_string = String::from("my");
    let mut second_string = "Second string".to_string();

    my_string.push_str(" String");
    println!("{}", my_string);

    for c in my_string.chars() {
        println!("{}", c);
    }

    for b in my_string.bytes() {
        println!("{}", b);
    }

    // Creating Hashmap collections in Rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10); // [Alice: 10]
    scores.insert(String::from("Bob"), 20); // [Alice: 10, Bob: 20]

    let alice_score = scores.get(&String::from("Alice"));
    println!("{:?}", alice_score);
    println!("{:?}", scores);

    scores.remove(&String::from("Bob"));
    println!("{:?}", scores);

    // Iterating through a HashMap
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
}
