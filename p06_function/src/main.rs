fn main() {
    another_function();
    another_function_with_parameters(5, 6);
    let sum = add_two_numbers(5, 6);
    println!("The sum of 5 and 6 is: {sum}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameters(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}
