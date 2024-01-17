fn main() {
    let num1 = 10.0;
    let num2 = 2.0;

    let result_add = calculate(Operation::Add, num1, num2);
    println!("{} + {} = {}", num1, num2, result_add);

    // Perform subtraction
    let result_subtract = calculate(Operation::Subtract, num1, num2);
    println!("{} - {} = {}", num1, num2, result_subtract);

    // Perform multiplication
    let result_multiply = calculate(Operation::Multiply, num1, num2);
    println!("{} * {} = {}", num1, num2, result_multiply);

    // Perform division
    let result_divide = calculate(Operation::Divide, num1, num2);
    println!("{} / {} = {}", num1, num2, result_divide);
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operation: Operation, num1: f64, num2: f64) -> f64 {
    match operation {
        Operation::Add => num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply => num1 * num2,
        Operation::Divide => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error.");
                f64::NAN
            }
        }
    }
}
