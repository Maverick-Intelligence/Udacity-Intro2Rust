pub(crate) enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate<T>(op: Operation, a: T, b: T) -> Result<T, &'static str>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + PartialEq
        + From<u8>,
{
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == T::from(0) {
                Err("Zero division is illegal!")
            } else {
                Ok(a / b)
            }
        }
    }
}

pub fn calculator(
    add_a: i32,
    add_b: i32,
    sub_a: i32,
    sub_b: i32,
    mul_a: i32,
    mul_b: i32,
    div_nz_a: i32,
    div_nz_b: i32,
    div_wz_a: i32,
) {
    println!("=== Chapter 2.4. Calculator Project ===\n");

    let addition = calculate(Operation::Add, add_a, add_b);
    let subtraction = calculate(Operation::Subtract, sub_a, sub_b);
    let multiplication = calculate(Operation::Multiply, mul_a, mul_b);
    let division_non_zero = calculate(Operation::Divide, div_nz_a, div_nz_b);

    const DIV_WZ_B: i32 = 0;
    let division_with_zero = calculate(Operation::Divide, div_wz_a, DIV_WZ_B);

    println!("- 2.4.1 Addition -");
    match addition {
        Ok(result) => println!("Generic `calculate(Add, {}, {})` returned Ok({}).", add_a, add_b, result),
        Err(e) => println!("Generic `calculate(Add, {}, {})` returned Err({}).", add_a, add_b, e),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 2.4.2 Subtraction -");
    match subtraction {
        Ok(result) => println!("Generic `calculate(Subtract, {}, {})` returned Ok({}).", sub_a, sub_b, result),
        Err(e) => println!("Generic `calculate(Subtract, {}, {})` returned Err({}).", sub_a, sub_b, e),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 2.4.3 Multiplication -");
    match multiplication {
        Ok(result) => println!("Generic `calculate(Multiply, {}, {})` returned Ok({}).", mul_a, mul_b, result),
        Err(e) => println!("Generic `calculate(Multiply, {}, {})` returned Err({}).", mul_a, mul_b, e),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 2.4.4 Division (Non-Zero) -");
    match division_non_zero {
        Ok(result) => println!("Generic `calculate(Divide, {}, {})` returned Ok({}).", div_nz_a, div_nz_b, result),
        Err(e) => println!("Generic `calculate(Divide, {}, {})` returned Err({}).", div_nz_a, div_nz_b, e),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 2.4.5 Division (Zero) -");
    match division_with_zero {
        Ok(result) => println!("Generic `calculate(Divide, {}, {})` returned Ok({}).", div_wz_a, DIV_WZ_B, result),
        Err(e) => println!("Generic `calculate(Divide, {}, {})` returned Err({}); the zero divisor is rejected by our guard.", div_wz_a, DIV_WZ_B, e),
    }
    println!("--------------------------------------------------\n\n")
}
