pub(crate) trait Animal {
    fn says(&self) -> String;
}

pub struct Dog {
    pub says: String,
}

pub struct Cat {
    pub says: String,
}

impl Animal for Dog {
    fn says(&self) -> String {
        self.says.clone()
    }
}

impl Animal for Cat {
    fn says(&self) -> String {
        self.says.clone()
    }
}

pub(crate) trait GreetingDefault {
    fn speak(&self) -> String {
        "Hello everyone!".to_string()
    }
}

pub struct GreetingEnglish {
    #[allow(dead_code)]
    pub speak: String,
}

pub struct GreetingGerman {
    pub speak: String,
}

impl GreetingDefault for GreetingEnglish {}

impl GreetingDefault for GreetingGerman {
    fn speak(&self) -> String {
        self.speak.clone()
    }
}

pub fn execute_trait_example_animal() {
    println!("- 2.2.1 Trait Without Default (Animal) -");
    let dog = Dog {
        says: "Woof".to_string(),
    };
    let cat = Cat {
        says: "Meow".to_string(),
    };
    println!(
        "Each type provides its own `says()`: the dog says \"{}\" and the cat says \"{}\".",
        dog.says(),
        cat.says()
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_trait_example_greeting() {
    println!("- 2.2.2 Trait With Default (GreetingDefault) -");
    let english_greeting = GreetingEnglish {
        speak: "".to_string(),
    };
    let german_greeting = GreetingGerman {
        speak: "Moin!".to_string(),
    };
    println!(
        "English keeps the trait's default \"{}\" while German overrides it with \"{}\".",
        english_greeting.speak(),
        german_greeting.speak()
    );
    println!("--------------------------------------------------\n\n")
}

pub fn execute_trait_example() {
    println!("=== Chapter 2.2. Traits ===\n");
    execute_trait_example_animal();
    execute_trait_example_greeting();
}
