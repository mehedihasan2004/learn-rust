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
    let my_mix_tuple: (&str, i8, bool, [i8; 5]) = ("Shad", 23, true, [1, 2, 3, 4, 5]);

    println!("Student tuple: {:?}", student);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // = = = = = = SLICE = = = = = =
    let number_slices: &[i8] = &[1, 2, 3, 4, 5];
    let animal_slices: &[&str] = &["Lion", "Tiger", "Crocodile"];
    let book_slices: &[&String] = &[&"IT".to_string(), &"HP".to_string(), &"ZEN".to_string()];

    println!("Number slice: {:?}", number_slices);
    println!("Animal slice: {:?}", animal_slices);
    println!("Book slice: {:?}", book_slices);

    // = = = = = = STRING = = = = = =
    // Strings: growable, mutable, owned string type
    let mut stone_cold: String = String::from("Hell, ");

    stone_cold.push_str("Yeah!");

    println!("Stone Cold Says: {}", stone_cold);

    // = = = = = = STRING SLICE = = = = = =
    let string: String = String::from("Hello, World!");
    let string_slice: &str = &string[0..6];

    println!("Slice Value: {}", string_slice);
}
