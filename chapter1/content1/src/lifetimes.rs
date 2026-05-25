fn try_dangling_reference() -> Result<i32, String> {
    Err(String::from(
        "error[E0597]: `x` does not live long enough — borrowed value does not live long enough",
    ))
}

fn dangling_reference_fail_example() {
    println!("- 1.3.1 Dangling Reference -");
    match try_dangling_reference() {
        Ok(value) => println!("Read value back: {}", value),
        Err(err) => println!(
            "A reference cannot outlive the value it points to: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn get_slice_of_string_until<'a>(some_string: &'a String, char_idx: usize) -> &'a str {
    &some_string[..char_idx]
}

fn lifetime_elision_example() {
    println!("- 1.3.2 Lifetime Elision -");
    let some_string = String::from("Hello, World!");
    let slice = get_slice_of_string_until(&some_string, 5);
    println!(
        "The compiler infers the output lifetime from the single input, so `slice` = \"{}\".",
        slice
    );
    println!("--------------------------------------------------\n\n")
}

fn try_longest_without_lifetime() -> Result<String, String> {
    Err(String::from(
        "error[E0106]: missing lifetime specifier — expected lifetime parameter",
    ))
}

fn missing_lifetime_fail_example() {
    println!("- 1.3.3 Missing Lifetime Annotation -");
    match try_longest_without_lifetime() {
        Ok(value) => println!("Returned: {}", value),
        Err(err) => println!(
            "With two input references, the compiler cannot pick an output lifetime: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn explicit_lifetime_example() {
    println!("- 1.3.4 Explicit Lifetime Annotation -");
    let str1 = String::from("hello");
    let str2 = String::from("world!");
    let result = longest(&str1, &str2);
    println!(
        "`<'a>` tells the compiler the output lives as long as the shorter input, so `longest` = \"{}\".",
        result
    );
    println!("--------------------------------------------------\n\n")
}

struct Game<'a> {
    level: &'a i32,
}

impl Game<'_> {
    fn return_level(&self) -> &i32 {
        self.level
    }
}

fn lifetime_in_struct_example() {
    println!("- 1.3.5 Lifetime in Struct -");
    let level = 1;
    let game = Game { level: &level };
    let returned = game.return_level();
    println!(
        "`Game<'a>` borrows `level` and the method returns it back; `returned` = {}.",
        returned
    );
    println!("--------------------------------------------------\n\n")
}

fn try_static_with_owned_string() -> Result<String, String> {
    Err(String::from(
        "error[E0597]: `str1` does not live long enough — borrowed value does not live long enough",
    ))
}

fn static_lifetime_fail_example() {
    println!("- 1.3.6 Static Lifetime: Owned String Fails -");
    match try_static_with_owned_string() {
        Ok(value) => println!("Returned: {}", value),
        Err(err) => println!(
            "Owned Strings are dropped at the end of their scope: {}",
            err
        ),
    }
    println!("--------------------------------------------------\n\n")
}

fn static_lifetime_example() {
    println!("- 1.3.7 Static Lifetime: String Literals -");
    let result;
    {
        let str1 = "123456789";
        let str2 = "123";
        result = longest(str1, str2);
    }
    println!(
        "String literals have the `'static` lifetime, so `result` = \"{}\" stays valid outside the inner scope.",
        result
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_lifetimes_example() {
    println!("=== Chapter 1.3. Lifetime ===\n");
    dangling_reference_fail_example();
    lifetime_elision_example();
    missing_lifetime_fail_example();
    explicit_lifetime_example();
    lifetime_in_struct_example();
    static_lifetime_fail_example();
    static_lifetime_example();
}
