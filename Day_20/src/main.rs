fn main() {
    let even_numbers = |x: i32| -> bool { x % 2 == 0 };

    let even = even_numbers(4);
    let odd = even_numbers(5);

    println!("Is the first number an even number: {}", even);
    println!("Is the second number an even number: {}", odd);

    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    println!("Even numbers are: {:?}", even_numbers);

    let print_data = |data: &str| {
        println!("Received data {}", data);
    };

    download_data("patika.dev", print_data);
}

fn is_even(numbers: Vec<i32>) -> Vec<i32> {
    let mut even_numbers_vec = Vec::new();

    for number in numbers {
        if number % 2 == 0 {
            even_numbers_vec.push(number);
        }
    }

    even_numbers_vec
}

fn download_data(url: &str, callback: impl FnOnce(&str)) {
    println!("Getting data from {}", url);

    std::thread::sleep(std::time::Duration::from_secs(1));
    let data = format!("Some data from {}", url);
    callback(&data);
}
