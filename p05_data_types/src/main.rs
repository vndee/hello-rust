fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // addition with variables
    let num1 = 5;
    let num2 = 10;
    let sum = num1 + num2;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // multiplication with variables
    let num1 = 4;
    let num2 = 30;
    let product = num1 * num2;
    println!("product: {product}");

    // division
    let truncated = -5 / 3; // Results in -1
    println!("truncated: {truncated}");

    // division with variables
    let num1 = 56.7;
    let num2 = 32.2;
    let quotient = num1 / num2;
    println!("quotient: {quotient}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {t}, f: {f}");

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tuple;
    println!("The value of y is: {y}");

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tuple;
    println!("The value of x is: {x}");

    let array: [i32; 3] = [1, 2, 3];
    println!("The value of array is: {array:?}");
}
