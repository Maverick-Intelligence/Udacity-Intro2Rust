fn memory_management_example() {
    println!("- 1.1.1 Memory Management -");
    let array = vec![0; 10];
    println!(
        "Rust allocated a Vec of length {} on the heap and will drop it automatically when this function returns.",
        array.len()
    );
    println!("--------------------------------------------------\n\n")
}

fn move_example() {
    println!("- 1.1.2 Variable Interaction: Move -");
    let some_string = String::from("Hello, World!");
    let some_other_string = some_string;
    println!(
        "`some_string` was moved into `some_other_string` = \"{}\"; the original binding is no longer usable.",
        some_other_string
    );
    println!("--------------------------------------------------\n\n")
}

fn try_use_after_move() -> Result<String, String> {
    let mut some_string = Some(String::from("Hello, World!"));
    let _moved = some_string.take();
    match some_string {
        Some(s) => Ok(s),
        None => Err(String::from(
            "error[E0382]: borrow of moved value: `some_string`",
        )),
    }
}

fn move_fail_example() {
    println!("- 1.1.3 Variable Interaction: Use After Move -");
    match try_use_after_move() {
        Ok(value) => println!("Read value back: {}", value),
        Err(err) => println!(
            "Reading a String after moving it does not compile: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn copy_example() {
    println!("- 1.1.4 Variable Interaction: Copy -");
    let some_int = 42;
    let some_other_int = some_int;
    println!(
        "`i32` implements Copy, so assignment duplicates the value: `some_int` = {} and `some_other_int` = {} are both still valid.",
        some_int, some_other_int
    );
    println!("--------------------------------------------------\n\n")
}

fn clone_example() {
    println!("- 1.1.5 Variable Interaction: Clone -");
    let some_string = String::from("Hello, World!");
    let some_other_string = some_string.clone();
    println!(
        "Calling `.clone()` makes a deep copy on the heap: `some_string` = \"{}\" and `some_other_string` = \"{}\" are both still valid.",
        some_string, some_other_string
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_memory_management_example() {
    println!("=== Chapter 1.1. Ownership ===\n");
    memory_management_example();
    move_example();
    move_fail_example();
    copy_example();
    clone_example();
}
