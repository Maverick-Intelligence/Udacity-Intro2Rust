use validator_macro::validate;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    #[validate]
    fn validate(&self) {}
}

fn main() {
    let user1 = User {
        name: String::from(""),
        age: 25,
    };

    user1.validate();

    let user2 = User {
        name: String::from("Nino"),
        age: 120,
    };

    user2.validate();

    let user3 = User {
        name: String::from("Josephine"),
        age: 18,
    };

    user3.validate();

    let user4 = User {
        name: String::from("Valeriya"),
        age: 30,
    };

    user4.validate();
}
