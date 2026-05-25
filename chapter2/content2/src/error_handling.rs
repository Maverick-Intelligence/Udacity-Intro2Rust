pub fn execute_unwrap_example() {
    println!("- 2.3.1 Unwrap -");
    let some_option = Some(42);
    let value = some_option.unwrap();
    println!(
        "`unwrap()` extracts the value when we are sure the `Option` is `Some`: value = {}.",
        value
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_expect_example() {
    println!("- 2.3.2 Expect -");
    let some_option = Some(42);
    let value = some_option.expect("Some option is None");
    println!(
        "`expect(msg)` is `unwrap()` plus a custom panic message: value = {}.",
        value
    );
    println!("--------------------------------------------------\n\n")
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}

pub fn execute_recoverable_error_example() {
    println!("- 2.3.3 Recoverable Error with Match -");
    let result1 = divide(84, 2);
    match result1 {
        Ok(value) => println!("divide(84, 2) succeeded with Ok({})", value),
        Err(error) => println!("divide(84, 2) failed with Err({})", error),
    }
    let result2 = divide(10, 0);
    match result2 {
        Ok(value) => println!("divide(10, 0) succeeded with Ok({})", value),
        Err(error) => println!("divide(10, 0) failed with Err({})", error),
    }
    println!("`Result` + `match` forces us to handle both the success and failure paths.");
    println!("--------------------------------------------------\n\n")
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn execute_try_operator_example() {
    println!("- 2.3.4 Try Operator (?) -");
    let result = read_file("");
    println!(
        "The `?` operator propagates the inner `Err` from `read_to_string` for an empty path: {:?}",
        result
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_error_handling_example() {
    println!("=== Chapter 2.3. Error Handling ===\n");
    execute_unwrap_example();
    execute_expect_example();
    execute_recoverable_error_example();
    execute_try_operator_example();
}
