fn immutable_borrow_example() {
    println!("- 1.2.1 Borrow Checker: Immutable Borrow -");
    let some_string = String::from("Hello, World!");
    let some_other_string = &some_string;
    println!(
        "`&some_string` makes a read-only reference, so `some_string` = \"{}\" and `some_other_string` = \"{}\" stay valid.",
        some_string, some_other_string
    );
    println!("--------------------------------------------------\n\n")
}

fn try_immutable_during_mutable() -> Result<String, String> {
    Err(String::from(
        "error[E0502]: cannot borrow `some_string` as immutable because it is also borrowed as mutable",
    ))
}

fn borrow_rule_one_fail_example() {
    println!("- 1.2.2 Borrow Checker: Rule 1 (Immutable + Mutable) -");
    match try_immutable_during_mutable() {
        Ok(value) => println!("Read value back: {}", value),
        Err(err) => println!(
            "Reading a value while a mutable borrow is active does not compile: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn try_two_mutable_borrows() -> Result<String, String> {
    Err(String::from(
        "error[E0499]: cannot borrow `some_string` as mutable more than once at a time",
    ))
}

fn borrow_rule_two_fail_example() {
    println!("- 1.2.3 Borrow Checker: Rule 2 (Two Mutable) -");
    match try_two_mutable_borrows() {
        Ok(value) => println!("Read value back: {}", value),
        Err(err) => println!(
            "Taking two mutable borrows at once does not compile: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn try_assign_to_immutable() -> Result<String, String> {
    Err(String::from(
        "error[E0384]: cannot assign twice to immutable variable `some_string`",
    ))
}

fn immutable_variable_fail_example() {
    println!("- 1.2.4 Mutability: Immutable Variable -");
    match try_assign_to_immutable() {
        Ok(value) => println!("Reassigned successfully: {}", value),
        Err(err) => println!(
            "Reassigning a variable without `mut` does not compile: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn mutable_variable_example() {
    println!("- 1.2.5 Mutability: Mutable Variable -");
    let mut some_string = String::from("Hello, World!");
    let before = some_string.clone();
    some_string = String::from("Bye, World!");
    println!(
        "Adding `mut` lets us reassign `some_string` from \"{}\" to \"{}\".",
        before, some_string
    );
    println!("--------------------------------------------------\n\n")
}

fn mutable_borrow_example() {
    println!("- 1.2.6 Mutability: Mutable Borrow -");
    let mut some_string = String::from("Hello, World!");
    let some_other_string = &mut some_string;
    *some_other_string = String::from("Bye, World!");
    println!(
        "`&mut some_string` lets us modify the value through the reference, so `some_string` is now \"{}\".",
        some_string
    );
    println!("--------------------------------------------------\n\n")
}

fn scope_example() {
    println!("- 1.2.7 Scope: Non-Lexical Lifetimes -");
    let mut some_string = String::from("Hello, World!");
    {
        let some_other_string = &mut some_string;
        *some_other_string = String::from("Bye, World!");
    }
    let yet_another_string = &mut some_string;
    *yet_another_string = String::from("Hello again, World!");
    println!(
        "Each `&mut` ends at its last use, so a new `&mut` is allowed afterwards: `some_string` is now \"{}\".",
        some_string
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_borrowing_example() {
    println!("=== Chapter 1.2. Borrowing ===\n");
    immutable_borrow_example();
    borrow_rule_one_fail_example();
    borrow_rule_two_fail_example();
    immutable_variable_fail_example();
    mutable_variable_example();
    mutable_borrow_example();
    scope_example();
}
