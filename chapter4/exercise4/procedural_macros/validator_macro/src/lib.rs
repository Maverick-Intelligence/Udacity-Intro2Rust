use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn validate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let block = &input.block;

    let result = quote! {
        fn #func_name(&self) {
            println!("Executing user information validation . . .");

            if self.name.is_empty() {
                println!("User name cannot be empty! Please retry");
            } else {
                println!("User name {} is valid!", self.name);
            }

            let age_limit: String;

            match self.age {
                0..=18 => {
                    age_limit = String::from("too young");
                    println!("User age {} is not valid! User is {}", self.age, age_limit);
                },
                120..=130 => {
                    age_limit = String::from("too F old");
                    println!("User age {} is not valid! User is {}", self.age, age_limit);
                }
                19..=119 => println!("User age {} is valid!", self.age),
                _ => println!("User age {} might not be a number!", self.age),

            }
            #block
        }
    };
    TokenStream::from(result)
}
