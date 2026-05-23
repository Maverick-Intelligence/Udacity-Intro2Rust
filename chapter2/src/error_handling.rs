pub fn execute_unwrap_example() {
    println!("Chapter 2.3.1. Error Handling - Unwrap");
    let some_option = Some(42);
    let value = some_option.unwrap();
    println!("The value is (unnwrapped): {}", value);
    println!("#####################################");
}

pub fn execute_expect_example() {
    println!("Chapter 2.3.2. Error Handling - Expect");
    let some_option = Some(42);
    let value = some_option.expect("Some option is None");
    println!("The value is (expected): {}", value);
    println!("#####################################");
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}

pub fn execute_recoverable_error_example() {
    println!("Chapter 2.3.3. Error Handling - Recoverable Error with Match");
    let result1 = divide(84, 2);
    match result1 {
        Ok(value) => println!("The value is (recoverable): {}", value),
        Err(error) => println!("Error: {}", error),
    }
    let result2 = divide(10, 0);

    match result2 {
        Ok(value) => println!("The value is (recoverable): {}", value),
        Err(error) => println!("Error example divide(10, 0): {}", error),
    }
    println!("#####################################");
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn execute_try_operator_example() {
    println!("Chapter 2.3.4. Error Handling - Try Operator (?)");
    let result = read_file("");
    println!("Result: {:?}", result);
    println!("#####################################");
}

pub fn execute_error_handling_example() {
    println!("Chapter 2.3. Error Handling");
    execute_unwrap_example();
    execute_expect_example();
    execute_recoverable_error_example();
    execute_try_operator_example();
}
