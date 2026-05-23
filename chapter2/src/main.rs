mod error_handling;
mod generics;
mod project_calculator;
mod traits;

fn main() {
    generics::execute_generics_example();
    traits::execute_trait_example();
    error_handling::execute_error_handling_example();
    println!("CHAPTER 2 PROJECT: CALCULATOR");
    project_calculator::calculator(21, 21, 84, 42, 21, 2, 84, 2, 42);
}
