// Functions

fn main() {
    hello_world();
    tell_height(180);
    human("Mehedi", 21, 180.01);
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
