fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for i in 0..list.len() {
        if list[i] > largest {
            largest = list[i];
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for i in 0..list.len() {
        if list[i] > largest {
            largest = list[i];
        }
    }
    largest
}

fn main() {
    let list_i32 = [1234, 2345, 3456, 4567, 5678];
    let list_char = ['a', 'b', 'c', 'd', 'e'];

    let largest_i32 = largest_i32(&list_i32);
    let largest_char = largest_char(&list_char);

    println!("The largest i32 in 'list_32' is {}", largest_i32);
    println!("The largest cahr in 'list_char' is {}", largest_char);
}
