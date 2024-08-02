/*
Primitive data types - int, float, bool, char

Integer
Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
Signed integers (- or +): i8, i16, i32, i64, i128
Unsigned integers (+): u8, u16, u32, u64, u128
*/

fn main() {
    let x: i32 = -42;
    let y: u32 = 100;

    println!("Signed integer: {}", x);
    println!("Signed integer: {}", y);
}
