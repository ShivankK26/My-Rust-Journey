use std::collections::HashMap;

fn main() {
    let mut my_map = HashMap::new();

    my_map.insert("Alice".to_string(), 10);
    my_map.insert("Bob".to_string(), 20);

    for (key, value) in my_map.iter() {
        println!("{} has {}", key, value);
    }

    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let nums = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", even_numbers);

    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);

    let chained: Vec<i32> = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * 2)
        .collect();
    println!("{:?}", chained);

    let squared_nums: HashMap<_, _> = numbers.iter().map(|n| (n, n * 2)).collect();
    println!("{:?}", squared_nums);
}
