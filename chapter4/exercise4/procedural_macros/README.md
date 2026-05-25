# Exercise — `#[validate]` Attribute Proc-macro

A two-crate workspace: a **proc-macro crate** (`validator_macro`) defines a `#[validate]` attribute that rewrites a method's body, and a **consumer crate** (`field_validator`) uses it on a `User` struct.

```
procedural_macros/
├── Cargo.toml                       ← workspace root
├── validator_macro/                 ← the macro crate ([lib] proc-macro = true)
│   ├── Cargo.toml
│   └── src/lib.rs                   ← #[validate] implementation
└── field_validator/                 ← the consumer
    ├── Cargo.toml
    └── src/main.rs                  ← User struct, #[validate] on User::validate
```

Run it:

```bash
cargo run -p field_validator
```

```
Executing user information validation . . .
User name cannot be empty! Please retry
User age 25 is valid!
Executing user information validation . . .
User name Nino is valid!
User age 120 is not valid! User is too F old
Executing user information validation . . .
User name Josephine is valid!
User age 18 is not valid! User is too young
Executing user information validation . . .
User name Valeriya is valid!
User age 30 is valid!
```

---

## 1. The workspace — [`Cargo.toml`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/procedural_macros/Cargo.toml)

```toml
[workspace]
resolver = "3"
members = ["validator_macro", "field_validator"]
```

| Piece | ELI5 |
|---|---|
| `[workspace]` | One `Cargo.lock`, one `target/`, one `cargo build` for many crates. |
| `resolver = "3"` | Use the **edition 2024** feature resolver. |
| `members` | The two crates Cargo manages together. |

Required here because proc-macro crates must live in their **own** crate (with `[lib] proc-macro = true`) — but you want it sitting right next to the binary that consumes it.

---

## 2. The macro crate — [`validator_macro/src/lib.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/procedural_macros/validator_macro/src/lib.rs)

### Cargo.toml flag that turns it into a proc-macro

```toml
[lib]
proc-macro = true

[dependencies]
quote = "1.0.45"
syn = { version = "2.0.117", features = ["full"] }
```

| Dep | What it does |
|---|---|
| `syn` (`full`) | Parses a `TokenStream` into a typed AST. `full` is required to handle items like `fn`/`struct`. |
| `quote` | Generates a `TokenStream` from Rust-shaped templates with `#variable` interpolation. |
| `proc-macro = true` | Tells `rustc` "this is a proc-macro crate — load it into the compiler at build time." |

### The macro

```rust
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
                0..=18    => { age_limit = String::from("too young");  /* … */ }
                120..=130 => { age_limit = String::from("too F old");  /* … */ }
                19..=119  => println!("User age {} is valid!", self.age),
                _         => println!("User age {} might not be a number!", self.age),
            }

            #block      // ← the body the user wrote, kept after the validation
        }
    };
    TokenStream::from(result)
}
```

| Piece | ELI5 |
|---|---|
| `#[proc_macro_attribute]` | "This `fn` is an **attribute** proc-macro — call it as `#[validate]` on top of an item." |
| `_attr` | The tokens **inside** the parens on the attribute (e.g. `#[validate(STRICT)]` → `STRICT`). We don't use any here. |
| `item` | The thing the attribute is sitting on — here, a method (`ItemFn`). |
| `parse_macro_input!(item as ItemFn)` | Parse the method's tokens into a typed AST node. |
| `input.sig.ident` | The method's name (`validate`). |
| `input.block` | The original body the user wrote (here: empty `{}`). |
| `quote! { fn #func_name(&self) { … #block } }` | **Replace** the original method with a new function of the same name, hard-coded validation logic, and the user's original body tacked on at the end. |
| `TokenStream::from(result)` | Hand the rewritten function back to the compiler. |

> ⚠️ This macro hard-codes references to `self.name` and `self.age` — so it **only works** on a method whose containing type has those exact fields. Not a generic validator; just a teaching exercise. A real-world macro would parse field names from the struct itself or take them in `_attr`.

---

## 3. The consumer — [`field_validator/src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/procedural_macros/field_validator/src/main.rs)

```rust
use validator_macro::validate;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    #[validate]
    fn validate(&self) {}     // ← the macro rewrites this whole body
}

fn main() {
    let user1 = User { name: String::from(""),        age: 25  };
    user1.validate();
    let user2 = User { name: String::from("Nino"),     age: 120 };
    user2.validate();
    let user3 = User { name: String::from("Josephine"), age: 18 };
    user3.validate();
    let user4 = User { name: String::from("Valeriya"), age: 30 };
    user4.validate();
}
```

| Piece | ELI5 |
|---|---|
| `use validator_macro::validate;` | Import the proc-macro by name — works better with rust-analyzer than path-style attributes. |
| `#[validate]` | The compiler hands the whole `fn validate(&self) {}` to the macro and uses whatever comes back instead. |
| `fn validate(&self) {}` | The **stub** the user writes. The body is empty because the macro injects everything. |

### Test cases — what each user covers

| User | name | age | Triggers branch | Why |
|---|---|---|---|---|
| `user1` | `""` | 25 | empty-name fail + valid age | Tests the empty-string guard. |
| `user2` | `"Nino"` | 120 | valid name + "too F old" | Tests the upper-bound match arm (`120..=130`). |
| `user3` | `"Josephine"` | 18 | valid name + "too young" | Tests the lower-bound match arm (`0..=18`). |
| `user4` | `"Valeriya"` | 30 | both valid | The happy path. |

---

## 4. What the compiler actually sees

After macro expansion, the `impl User { … }` block looks like this (paraphrased):

```rust
impl User {
    fn validate(&self) {
        println!("Executing user information validation . . .");

        if self.name.is_empty() {
            println!("User name cannot be empty! Please retry");
        } else {
            println!("User name {} is valid!", self.name);
        }

        match self.age {
            0..=18    => { /* "too young"  */ }
            120..=130 => { /* "too F old"  */ }
            19..=119  => { /* "valid!"     */ }
            _         => { /* "not a num"  */ }
        }
        {}   // ← original empty block from the user's stub
    }
}
```

There's **zero runtime cost** to the macro — it just shapes the source the compiler will see.

---

## 5. Cheat Sheet

| Concept | One-liner |
|---|---|
| Attribute proc-macro signature | `pub fn name(attr: TokenStream, item: TokenStream) -> TokenStream` |
| Mark it | `#[proc_macro_attribute]` |
| Crate type | `[lib] proc-macro = true` in `Cargo.toml` |
| Parse the input | `parse_macro_input!(item as ItemFn)` |
| Reuse the original body | Capture `&input.block`, splice with `#block` in `quote!` |
| Workspace setup | One `Cargo.toml` at the root listing each member |
| Import the macro | `use validator_macro::validate;` (better than `#[validator_macro::validate]`) |

| Command | Does |
|---|---|
| `cargo run -p field_validator` | Build the macro crate, then build + run the consumer |
| `cargo build -p validator_macro` | Build only the proc-macro crate (rebuilds the `.so` rust-analyzer loads) |
| `cargo expand -p field_validator` | See the expanded code (install: `cargo install cargo-expand`) |
