use std::fs;
fn main() {
    let my_vec = vec![1, 2];
    let my_variable = my_vec[5];

    let veggies = ["carrot", "tomato", "potato"];

    chooseVeggie(veggies[0]);
    chooseVeggie(veggies[1]);
    chooseVeggie(veggies[2]);
    chooseVeggie("egg plant");

    let my_content = getFileContent("myfile.txt");

    match my_content {
        Ok(item) => println!("The result is {}", item),
        Error(err) => println!("We got an error"),
    }
}

fn chooseVeggie(veggie: &str) {
    match veggie {
        "carrot" => println!("Not that good one"),
        "tomato" => println!("Burgersss"),
        "potato" => println!("Fav veggie"),
        _ => panic!("Not accepted veggie."),
    }
}

fn divide(num: f64, deno: f64) -> Option<f64> {
    if deno == 0.0 {
        return None;
    } else {
        return Some(num / deno);
    }
}

fn getFileContent(file_name: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}
