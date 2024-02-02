// use sports::football;
mod sports;
// use sports::football;
// use sports::footballPlayer;

// Another way of importing
use sports::{football, footballPlayer};

fn main() {
    // sports::football();
    // football();

    // let messi = sports::footballPlayer {
    //     name: "messi".to_string(),
    //     age: 19,
    // };

    let my_player = footballPlayer {
        name: "messi".to_string(),
        age: 34,
    };

    football();
}

// mod sports {
//     pub fn football() {
//         println!("We play football...");
//     }

//     pub struct footballPlayer {
//         pub name: String,
//         pub age: i32,
//     }
// }
