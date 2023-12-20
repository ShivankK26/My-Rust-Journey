fn main() {
    println!("Hello, world!");

    let sum = add(3, 5);
    println!("The sum is: {}", sum);

    let day_of_the_week = "Sunday";

    if day_of_the_week == "Sunday" {
        println!("The race day");
    } else if day_of_the_week == "Saturday" {
        println!("Qualifying today!");
    } else {
        println!("Patiently wait for the race day!")
    }

    // while loop
    let mut counter = 0;

    while counter <= 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }

    // for loop
    for number in 1..5 {
        println!("Number is {}", number);
    }

    // another way of writing for loops
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for number in numbers {
        println!("Number is {}", number);
    }

    println!("--------------------------");

    // loop
    counter = 0;
    loop {
        println!("Counter value is {}", counter);
        counter += 1;

        if counter == 6 {
            break;
        }
    }

    // match- it is kinda switch case statement
    let num = 5;

    match num {
        1 => {
            println!("The number is one");
            println!("This is the first match arm");
        }
        2 => println!("The number is two."),
        3 => println!("The number is three."),
        _ => println!("The number is something else"),
    }

    let result = match num {
        1 => "The number is one",
        2 => "The number is two",
        3 => "The number is three",
        // 4 => println!("The number is four"), will not work
        _ => "The number is something else",
    };

    println!("The result is {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}

// fn no_param() -> i32 {
//     println!("This just works!");
// }
