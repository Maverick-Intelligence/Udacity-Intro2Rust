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
    let dog = Dog {
        says: "Woof".to_string(),
    };
    let cat = Cat {
        says: "Meow".to_string(),
    };

    println!("The dog says {}", dog.says());
    println!("The cat says {}", cat.says());
    println!("#####################################");
}

pub fn execute_trait_example_greeting() {
    let english_greeting = GreetingEnglish {
        speak: "".to_string(),
    };
    let german_greeting = GreetingGerman {
        speak: "Moin!".to_string(),
    };

    println!("The greeting in English is {}", english_greeting.speak());
    println!("The greeting in German is {}", german_greeting.speak());
    println!("#####################################");
}

pub fn execute_trait_example() {
    println!("Chapter 2.2. Traits");
    execute_trait_example_animal();
    execute_trait_example_greeting();
}
