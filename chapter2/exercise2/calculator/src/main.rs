mod calculator;

fn main() {
    println!("=== Chapter 2 Exercise: Calculator ===\n");

    let add_a = 21.0;
    let add_b = 21.0;
    let sub_a = 63;
    let sub_b = 21;
    let mul_a = 0.314;
    let mul_b = 256.235;
    let div_nz_a = 0.314;
    let div_nz_b = 0.0001;
    let div_wz_a = 42;

    let addition = calculator::calculate(calculator::Operation::Add, add_a, add_b);
    let subtraction = calculator::calculate(calculator::Operation::Subtract, sub_a, sub_b);
    let multiplication = calculator::calculate(calculator::Operation::Multiply, mul_a, mul_b);
    let division_non_zero =
        calculator::calculate(calculator::Operation::Divide, div_nz_a, div_nz_b);

    const DIV_WZ_B: i32 = 0;
    let division_with_zero =
        calculator::calculate(calculator::Operation::Divide, div_wz_a, DIV_WZ_B);

    println!("- 1. Addition -");
    match addition {
        Ok(result) => println!(
            "Generic `calculate(Add, {}, {})` returned Ok({}).",
            add_a, add_b, result
        ),
        Err(e) => println!(
            "Generic `calculate(Add, {}, {})` returned Err({}).",
            add_a, add_b, e
        ),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 2. Subtraction -");
    match subtraction {
        Ok(result) => println!(
            "Generic `calculate(Subtract, {}, {})` returned Ok({}).",
            sub_a, sub_b, result
        ),
        Err(e) => println!(
            "Generic `calculate(Subtract, {}, {})` returned Err({}).",
            sub_a, sub_b, e
        ),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 3. Multiplication -");
    match multiplication {
        Ok(result) => println!(
            "Generic `calculate(Multiply, {}, {})` returned Ok({}).",
            mul_a, mul_b, result
        ),
        Err(e) => println!(
            "Generic `calculate(Multiply, {}, {})` returned Err({}).",
            mul_a, mul_b, e
        ),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 4. Division (Non-Zero) -");
    match division_non_zero {
        Ok(result) => println!(
            "Generic `calculate(Divide, {}, {})` returned Ok({}).",
            div_nz_a, div_nz_b, result
        ),
        Err(e) => println!(
            "Generic `calculate(Divide, {}, {})` returned Err({}).",
            div_nz_a, div_nz_b, e
        ),
    }
    println!("--------------------------------------------------\n\n");

    println!("- 5. Division (Zero) -");
    match division_with_zero {
        Ok(result) => println!(
            "Generic `calculate(Divide, {}, {})` returned Ok({}).",
            div_wz_a, DIV_WZ_B, result
        ),
        Err(e) => println!(
            "Generic `calculate(Divide, {}, {})` returned Err({}); the zero divisor is rejected by our guard.",
            div_wz_a, DIV_WZ_B, e
        ),
    }
    println!("--------------------------------------------------\n\n")
}
