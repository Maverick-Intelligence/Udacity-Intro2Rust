mod borrowing;
mod lifetimes;
mod memory_management;

fn main() {
    println!("-------------------------");
    println!("Intro to Rust | Chapter 1");
    println!("-------------------------\n");
    memory_management::execute_memory_management_example();
    borrowing::execute_borrowing_example();
    lifetimes::execute_lifetimes_example();
}
