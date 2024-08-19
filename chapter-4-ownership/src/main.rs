/*
Ownership
[stopping/Resuming the programm]

Ownership intorduced by Rust to solve memory safety issues and high performance at the same time.

Q1. What is Ownership?
Ans: Every value has a single owner [every variale has one value, and it is it's sole owner].

Ownership Rules:-
1. Each value in Rust has a variable that's it's owner.
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/

// Example of rule number 1.
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);

//     println!("Length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Example of rule number 2.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;

//     println!("{}", s1); // s1 is not useable anymore
//     println!("{}", s2);
// }

// Example of rule number 3.
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);

    println!("Length of '{}' is {}.", s1, len);
} // s1 goes out of scope and its value will be dropped

fn print_lost(s: &string) {
    println!("{}", &s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
