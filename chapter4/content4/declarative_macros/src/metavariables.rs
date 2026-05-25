macro_rules! example {
    ($x:ident) => {
        println!("stringify!({}) = \"{}\"", stringify!($x), stringify!($x));
    };
}

pub fn demo() {
    println!("=== 4.1.2 Metavariable Expressions ===");

    example!(my_var);
    example!(SomeType);

    println!();
}
