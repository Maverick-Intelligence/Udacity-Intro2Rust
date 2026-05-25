use proc_macros_lib::{Hello, log_input, str_with_len};

#[derive(Hello)]
struct MyStruct;

#[log_input(INFO)]
fn add(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    let result = str_with_len!("Welcome Rustafarays! Hello, world of Rust!");
    println!("== 4.2.1. Procedural Function-like Macros ==");
    println!("Result: {:?}\n", result);

    println!("== 4.2.2. Procedural Attribute Macros ==");
    let result = add(21, 21);
    println!("Result: {}\n", result);

    println!("== 4.2.3. Procedural Derive  Macros ==");
    let instance = MyStruct;
    instance.hello();
}
