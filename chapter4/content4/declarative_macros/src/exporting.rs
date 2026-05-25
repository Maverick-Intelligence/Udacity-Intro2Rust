#[macro_export]
macro_rules! my_macro {
    () => {
        $crate::exporting::some_function();
    };
}

pub fn some_function() {
    println!("Hello from `some_function` inside the current crate!");
}

pub fn demo() {
    println!("=== 3. Exporting Macros ===");

    crate::my_macro!();

    println!();
}
