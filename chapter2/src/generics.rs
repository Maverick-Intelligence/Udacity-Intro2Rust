pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for i in 0..list.len() {
        if list[i] > largest {
            largest = list[i];
        }
    }
    largest
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for i in 0..list.len() {
        if list[i] > largest {
            largest = list[i];
        }
    }
    largest
}

pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in 0..list.len() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }
    largest
}

pub fn execute_generics_example() {
    println!("=== Chapter 2.1. Generics ===\n");

    println!("- 2.1.1 Type-Specific Largest -");
    let list_i32 = [1234, 2345, 3456, 4567, 5678];
    let list_char = ['a', 'b', 'c', 'd', 'e'];
    let largest_i32 = largest_i32(&list_i32);
    let largest_char = largest_char(&list_char);
    println!(
        "Without generics we need one function per type: `largest_i32` returned {} and `largest_char` returned '{}'.",
        largest_i32, largest_char
    );
    println!("--------------------------------------------------\n\n");

    println!("- 2.1.2 Generic Largest -");
    let list_f32 = [0.12345, 1.23456, 2.34567, 3.45678, 4.56789];
    let largest_temp_i32 = largest(&list_i32);
    let largest_temp_char = largest(&list_char);
    let largest_temp_f32 = largest(&list_f32);
    println!(
        "One generic `largest<T: PartialOrd>` handles every comparable type: i32 = {}, char = '{}', f32 = {}.",
        largest_temp_i32, largest_temp_char, largest_temp_f32
    );
    println!("--------------------------------------------------\n\n");
}
