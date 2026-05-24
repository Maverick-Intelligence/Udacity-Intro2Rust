enum Operation {
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
    println!("CHAPTER 2 PROJECT: CALCULATOR");
    let addition = calculate(Operation::Add, add_a, add_b);
    let subtraction = calculate(Operation::Subtract, sub_a, sub_b);
    let multiplication = calculate(Operation::Multiply, mul_a, mul_b);
    let division_non_zero = calculate(Operation::Divide, div_nz_a, div_nz_b);

    const DIV_WZ_B: i32 = 0;
    let division_with_zero = calculate(Operation::Divide, div_wz_a, DIV_WZ_B);

    match addition {
        Ok(result) => println!("{} + {}: {}", add_a, add_b, result),
        Err(e) => println!("Error: {}", e),
    }

    match subtraction {
        Ok(result) => println!("{} - {}: {}", sub_a, sub_b, result),
        Err(e) => println!("Error: {}", e),
    }

    match multiplication {
        Ok(result) => println!("{} * {}: {}", mul_a, mul_b, result),
        Err(e) => println!("Error: {}", e),
    }

    match division_non_zero {
        Ok(result) => println!("{} / {}: {}", div_nz_a, div_nz_b, result),
        Err(e) => println!("Error: {}", e),
    }

    match division_with_zero {
        Ok(result) => println!("{} / {}: {}", div_wz_a, DIV_WZ_B, result),
        Err(e) => println!("Error: {}", e),
    }
}
