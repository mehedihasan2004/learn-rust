// Primitive data types - int, float, bool, char

fn main() {
    // = = = = = = INTEGER = = = = = =
    /*
    Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
    Signed integers (- or +): i8, i16, i32, i64, i128
    Unsigned integers (+): u8, u16, u32, u64, u128
    */

    let x: i32 = -42;
    let y: u32 = 100;

    println!("Signed integer: {}", x);
    println!("Signed integer: {}", y);

    /*
    Range
    difference between i32(32 bits) and i64(64 bits)
    i32 - 2147483647
    i64 - 9223372036854775807
    */

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
}
