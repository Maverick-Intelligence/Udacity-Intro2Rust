mod generics;

fn main() {
    let list_i32 = [1234, 2345, 3456, 4567, 5678];
    let list_char = ['a', 'b', 'c', 'd', 'e'];

    let largest_i32 = generics::largest_i32(&list_i32);
    let largest_char = generics::largest_char(&list_char);

    println!("The largest i32 in 'list_32' is {}", largest_i32);
    println!("The largest cahr in 'list_char' is {}", largest_char);
    println!("#####################################");

    let list_f32 = [0.12345, 1.23456, 2.34567, 3.45678, 4.56789];

    let largest_temp_i32 = generics::largest(&list_i32);
    let largest_temp_char = generics::largest(&list_char);
    let largest_temp_f32 = generics::largest(&list_f32);

    println!("The largest i32 in 'list_temp_i32' is {}", largest_temp_i32);
    println!(
        "The largest char in 'list_temp_char' is {}",
        largest_temp_char
    );
    println!("The largest f32 in 'list_temp_f32' is {}", largest_temp_f32);
}
