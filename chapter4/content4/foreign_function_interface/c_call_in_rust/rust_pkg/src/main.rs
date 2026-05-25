fn unsafe_rust_example() {
    println!("== Chapter 4.3.1. Unsafe in Rust ==");
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is: {:#?}", r1);
    println!("r2 is: {:#?}\n", r2);

    let deref_r1 = unsafe { *r1 };
    let deref_r2 = unsafe { *r2 };

    println!("Unsafed dereferred r1 is: {:#?}", deref_r1);
    println!("Unsafed dereferred r2 is: {:#?}\n", deref_r2);
}

unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    println!("== Chapter 4.3. Foreign Function Interface ==\n");
    unsafe_rust_example();

    println!("== Chapter 4.3.2. Calling C Function ==");
    let result = unsafe { add(21, 21) };
    println!(
        "Calling C function add(a: i32, b: i32) with argument (21, 21) and the result is {}\n",
        result
    );
}
