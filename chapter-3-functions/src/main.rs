// Functions

fn main() {
    hello_world();
    tell_height(180);
    human("Mehedi", 21, 180.01);

    let x: u8 = {
        let price: u8 = 5;
        let qty: u8 = 10;

        price * qty
    };

    println!("Result is: {}", x);

    let y: u8 = add(4, 6);

    println!("Value from variable 'y' is: {}.", y);
    println!("Value from function 'add' is: {}.", add(40, 60));

    let weight: f64 = 70.0;
    let height: f64 = 1.80;
    let bmi: f64 = calculate_bmi(weight, height);

    println!("Your BMI is: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, Rust 🦀!");
}

fn tell_height(height: u8) {
    println!("My height is {}cm.", height);
}

fn human(name: &str, age: u8, height: f32) {
    println!(
        "My name is {}, I'm {} years old, and my height is {}cm.",
        name, age, height
    );
}

/*
= = = = = = EXPRESSIONS = = = = = =
Expressions: Anything that returns a value.
Like: -
5
true & false
add(3, 4)
if condition {value1} else {value2}
{()}
*/

// Function returning values
fn add(a: u8, b: u8) -> u8 {
    a + b
}

/*
= = = = = = STATEMENTS = = = = = =
Statement: Anything that does not return a value.
Almost all statements in Rust end with ;

let y = let x = 10;

1. Variable declarations: let x = 5;
2. Function definitions: fn foo() {}
3. Control flow statements: if condition { /* code */} else { /* code */}, while condition { /* code */}, etc.
*/

// Final example

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
    // Formula: BMI = height(kg)/height(m)^2
}
