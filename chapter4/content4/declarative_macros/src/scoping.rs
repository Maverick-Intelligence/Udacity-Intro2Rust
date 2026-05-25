macro_rules! some_macro {
    () => {
        println!("Hello, World! (called from a submodule)");
    };
}

mod inner {
    pub(super) fn test() {
        some_macro!();
    }
}

pub fn demo() {
    println!("=== 4.1.4 Scoping Quirks ===");

    inner::test();

    println!();
}
