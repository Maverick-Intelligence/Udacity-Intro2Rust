macro_rules! one {
    () => {
        1
    };
}

macro_rules! is_type {
    ($t:ty) => {
        // Does nothing, but checks the type fragment.
    };
}

macro_rules! my_vec {
    ($($x:expr),*) => {{
        let mut tmp = Vec::new();
        $( tmp.push($x); )*
        tmp
    }};
}

pub fn demo() {
    println!("=== 4.1.1 Basics ===");

    let two = one!() + one!();
    println!("one!() + one!() = {}", two);

    is_type!(i32);
    is_type!(Vec<String>);
    println!("is_type!(i32) and is_type!(Vec<String>) compiled as no-ops");

    let v = my_vec![1, 2, 3];
    println!("my_vec![1, 2, 3] = {:?}", v);

    println!();
}
