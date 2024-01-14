fn main() {
    // Using the enum Weather
    let current_wearher = Weather::Sunny;

    // Using the enum Message
    let msg = Message::Write(String::from("Hello, Rust!"));

    // Using the process_message function
    process_message(msg);

    // Calling the animal enum
    let my_pet = Animal::Cat("Melo".to_string());

    if let Animal::Cat(name) = my_pet {
        println!("My cat name is: {}", name);
    } else {
        println!("My pet is not a cat")
    }

    // Calling the impl message
    let msg = Message::Write(String::from("Melo is sleeping"));
    msg.call();
}

enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => {
                println!("change colour to r: {}, g: {}, b: {}", r, g, b);
            }
        }
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit vaiant has no data.");
        }
        Message::Move { x, y } => {
            println!("Move to coordinates x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text Nessage: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
        }
    }
}

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}
