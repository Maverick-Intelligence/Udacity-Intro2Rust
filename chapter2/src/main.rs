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
    println!("Hello, world!");
}
