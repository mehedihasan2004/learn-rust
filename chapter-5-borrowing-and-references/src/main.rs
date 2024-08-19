/*
References and Borrowing
Safety and Performance
Borrowing and references are powerfull concepts

Understanding References:-
References: Enable you to borrow values without taking ownership.
I. Immutable reference.
II. Mutabel reference.
Create reference by add "&".
*/

// I. Immutable reference
fn main() {
    let x: i8 = 5;
    let y: &i8 = &x;

    println!("Value of x is: {}", x);
    println!("Value of y is: {}", y);
}
