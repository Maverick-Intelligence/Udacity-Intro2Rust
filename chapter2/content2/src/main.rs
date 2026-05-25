mod error_handling;
mod generics;
mod traits;

fn main() {
    println!("-------------------------");
    println!("Intro to Rust | Chapter 2");
    println!("-------------------------\n");
    generics::execute_generics_example();
    traits::execute_trait_example();
    error_handling::execute_error_handling_example();
}
