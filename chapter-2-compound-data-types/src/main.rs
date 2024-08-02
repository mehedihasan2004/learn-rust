// Compound Data Types - arrays, tuples, slices, and strings (slice string)

fn main() {
    // = = = = = = ARRAY = = = = = =
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];

    println!("Numbers: {:?}", numbers);
    println!("Fruits: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Third fruit: {}", fruits[2]);

    // = = = = = = TUPLE = = = = = =
    let student: (String, i8, bool) = ("Sabbir".to_string(), 3, true);
    let my_mix_tuple = ("Shad", 23, true, [1, 2, 3, 4, 5]);

    println!("Student tuple: {:?}", student);
    println!("My mix tuple: {:?}", my_mix_tuple);
}
