fn main() {
    println!("Hello, world!");

    let message = "Hey, there!";

    let x: i32 = 42; // i32 is the integer type
    let pi: f64 = 3.14; // this is a float type integer

    let is_rust_fun: bool = true; // boolean type

    let letter_a: char = 'a'; // character type

    // Now applying the concept of shadowing, i.e overwriting the value which we defined perviously
    let x = 4;
    if x >= 0 {
        println!("x is non-negative")
    } else {
        println!("x is negative")
    }

    // defining a mutable data type, all the data types which we defined previously were immutable ones
    let mut i = 1;

    while i <= 5 {
        println!("{}", i);
        i += 1;
    }
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
