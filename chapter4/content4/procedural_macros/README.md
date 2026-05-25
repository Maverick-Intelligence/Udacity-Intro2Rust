# Function-like Procedural Macros

A **procedural macro** is just an exported Rust **function** that takes a `TokenStream` in and gives a `TokenStream` back — no `macro_rules!` pattern matching, just code that builds code. They're more flexible than declarative macros and can do anything a regular Rust function can (parse, generate, transform).

```
procedural_macros/
├── Cargo.toml             ← workspace root
├── proc_macros_lib/       ← the macro crate (proc-macro = true)
│   ├── Cargo.toml
│   └── src/lib.rs
└── proc_macros_bin/       ← the consumer
    ├── Cargo.toml
    └── src/main.rs
```

Run the example:

```bash
cargo run -p proc_macros_bin
```

```
(42, "Welcome Rustafarays! Hello, world of Rust!")
```

---

## 1. Function-like vs Attribute proc-macros

You usually meet proc-macros as **attributes** like `#[derive(Debug)]` or `#[test]`. **Function-like** proc-macros are different — they're invoked like declarative macros (`my_macro!(...)`), just much more powerful.

| Macro flavour | How you call it | Defined with |
|---|---|---|
| Declarative | `name!(args)` | `macro_rules!` |
| **Function-like proc-macro** | `name!(args)` | `#[proc_macro] pub fn name(input: TokenStream) -> TokenStream` |
| Derive proc-macro | `#[derive(Trait)]` | `#[proc_macro_derive(Trait)]` |
| Attribute proc-macro | `#[my_attr] fn …` | `#[proc_macro_attribute]` |

This lesson covers the **function-like** kind.

---

## 2. Proc-macros must live in their own crate

The `rustc` toolchain compiles proc-macro crates differently from regular ones — they are loaded **into the compiler** at build time to expand into your code. To opt in:

```toml
# proc_macros_lib/Cargo.toml
[lib]
proc-macro = true
```

A proc-macro crate can **only** export proc-macro items (functions tagged with one of the proc-macro attributes). It can't expose normal items to its consumers. That's why we have **two** crates here and need…

---

## 3. Cargo Workspaces

A **workspace** is a single `Cargo.lock`, a single `target/` directory, and one `cargo build` command that builds **multiple member crates** together.

```toml
# procedural_macros/Cargo.toml  (workspace root)
[workspace]
resolver = "2"
members = ["proc_macros_lib", "proc_macros_bin"]
```

| Benefit | ELI5 |
|---|---|
| Shared dependencies | Crates resolve once; no duplicate compilations. |
| Coordinated builds & tests | `cargo build` / `cargo test` at the root touches every member. |
| Required for proc-macros | The macro crate must be a separate crate, but you want it to live next to its consumer. |

Run a specific member:

```bash
cargo run -p proc_macros_bin      # run the consumer
cargo build -p proc_macros_lib    # build only the macro crate
```

---

## 4. The proc-macro crate — [`proc_macros_lib/src/lib.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/procedural_macros/proc_macros_lib/src/lib.rs)

### Cargo.toml

```toml
[package]
name = "proc_macros_lib"
version = "0.1.0"
edition = "2024"

[dependencies]
quote = "1.0.45"
syn = { version = "2.0.117", features = ["full"] }

[lib]
proc-macro = true
```

| Crate | What it does |
|---|---|
| `syn` | **Parses** a `TokenStream` into a typed Rust syntax tree (literals, expressions, items, etc.). |
| `quote` | **Generates** a `TokenStream` from Rust-shaped templates with `#variable` interpolation. |
| `proc-macro = true` | The flag that tells `rustc` "this crate is a proc-macro crate." |

The `["full"]` feature on `syn` unlocks parsing whole items like `fn`, `struct`, etc. — needed by the attribute macro further below.

### The macro

```rust
use proc_macro;
use quote;
use syn;

#[proc_macro]
pub fn str_with_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);

    let input_str = input.value();
    let len = input_str.len();

    let output = quote::quote! {
        (#len, #input_str)
    };

    output.into()
}
```

Step-by-step:

| Step | ELI5 |
|---|---|
| `#[proc_macro]` | Marks this `fn` as a function-like proc-macro. |
| `input: proc_macro::TokenStream` | The raw tokens between `(…)` at the call site. |
| `parse_macro_input!(input as syn::LitStr)` | "Treat the input as a string literal." On failure it emits a proper compile error pointing at the call site. |
| `let len = input_str.len();` | Compute the length **at compile time**. |
| `quote::quote! { (#len, #input_str) }` | Build a `TokenStream2` shaped like `(42, "Welcome…")`. The `#name` syntax interpolates Rust values into the generated tokens. |
| `output.into()` | Convert `quote`'s `TokenStream2` into the `proc_macro::TokenStream` the compiler expects. |

So `str_with_len!("Welcome…")` compiles to **the tuple literal** `(42, "Welcome…")` — no runtime work at all.

---

## 5. The consumer crate — [`proc_macros_bin/src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/procedural_macros/proc_macros_bin/src/main.rs)

```toml
# proc_macros_bin/Cargo.toml
[dependencies]
proc_macros_lib = { path = "../proc_macros_lib" }
```

```rust
use proc_macros_lib::str_with_len;

fn main() {
    let result = str_with_len!("Welcome Rustafarays! Hello, world of Rust!");
    println!("{:?}", result);
}
```

```bash
$ cargo run -p proc_macros_bin
(42, "Welcome Rustafarays! Hello, world of Rust!")
```

| Notice | ELI5 |
|---|---|
| `use proc_macros_lib::str_with_len;` | Import the proc-macro by name. |
| Path-style dep | Sibling crate via `path = "../proc_macros_lib"` — handy inside a workspace. |
| No runtime cost | The macro ran during `cargo build`; the binary just prints the precomputed tuple. |

---

## 6. Attribute Proc-macros — `log_input`

We've seen a **function-like** proc-macro (`#[proc_macro]`). An **attribute** proc-macro is the same idea but worn as a tag on top of an item: a function, struct, impl block, etc. The macro receives the item, may read or rewrite it, and emits whatever replaces it.

```rust
/// Prints:
///   <LEVEL>: Calling function '<FUNCTION_NAME>' with args: (<ARG1>, <ARG2>, …)
///
/// Example usage:
///   #[log_input(<LEVEL>)]
#[proc_macro_attribute]
pub fn log_input(ann: TokenStream, item: TokenStream) -> TokenStream {
    // …
}
```

### Two differences vs the function-like macro

| Function-like | Attribute |
|---|---|
| `#[proc_macro]` | **`#[proc_macro_attribute]`** |
| `fn name(input: TokenStream) -> TokenStream` | `fn name(ann: TokenStream, item: TokenStream) -> TokenStream` — **two** inputs |
| Called as `name!(args)` | Sits on top of an item: `#[name(ann)] fn …` |

| Param | What it carries |
|---|---|
| `ann` | The tokens **inside the parens** on the attribute — e.g. `INFO` from `#[log_input(INFO)]`. **Optional** — an attribute macro can take no annotation at all (just like `#[proc_macro_attribute]` itself doesn't have one). |
| `item` | The whole item the attribute is sitting on — usually a function, struct, or impl block. |

### Walking through the body

```rust
let mut input = parse_macro_input!(item as ItemFn);   // ① parse the function
let ann_str = ann.to_string();                         // ② the level, as a raw string

let fn_name = &input.sig.ident;                        // ③ grab the function's name

let args: Vec<_> = input.sig.inputs.iter()             // ④ collect each arg's name
    .map(|arg| match arg {
        FnArg::Typed(pat_type) => {
            if let Pat::Ident(ident) = &*pat_type.pat {
                let arg_value = &ident.ident;
                quote! { #arg_value }
            } else {
                quote! {}
            }
        }
        _ => quote! {},
    })
    .collect();

let log_stmt = quote! {                                // ⑤ build the println!
    println!(
        "{}: Calling function '{}' with args: {:?}",
        #ann_str, stringify!(#fn_name), (#(#args),*)
    );
};

let original_body = input.block;                       // ⑥ remember the old body
input.block = Box::new(parse_quote!({                  // ⑦ wrap: log, then original
    #log_stmt
    #original_body
}));

quote! { #input }.into()                               // ⑧ emit the rewritten fn
```

| Step | ELI5 |
|---|---|
| ① `parse_macro_input!(item as ItemFn)` | Parse the tokens of the function the attribute sits on into a typed AST node. Requires `syn`'s `full` feature. |
| ② `ann.to_string()` | Turn the annotation (e.g. `INFO`) into a raw `String`. Because we just stringify it, the annotation can be virtually anything. |
| ③ `input.sig.ident` | The function's name (e.g. `add`). |
| ④ `input.sig.inputs.iter()` | Each parameter. For typed params (`a: u8`), peel out the binding name; ignore receivers like `self`. Path: `FnArg::Typed` → `Pat::Ident` → the actual identifier. |
| ⑤ `quote! { println!(…) }` | Build the log line, interpolating level + function name + arg names. The `(#(#args),*)` repetition becomes a **tuple of arg values** at runtime. |
| ⑥–⑦ `input.block = …` | Rewrite the function's body: log line first, then the original via `parse_quote!`. |
| ⑧ `quote! { #input }` | Hand back the **modified** function as the macro's output — it replaces the original at the call site. |

### Using it — [`proc_macros_bin/src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/procedural_macros/proc_macros_bin/src/main.rs)

```rust
#[proc_macros_lib::log_input(INFO)]
fn add(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    let result = add(21, 21);
    println!("Result: {}", result);
}
```

Output:

```
INFO: Calling function 'add' with args: (21, 21)
Result: 42
```

So at compile time, `add` got rewritten to:

```rust
fn add(a: u8, b: u8) -> u8 {
    println!("INFO: Calling function 'add' with args: {:?}", (a, b));
    a + b
}
```

| Win | ELI5 |
|---|---|
| Zero call-site noise | Stick `#[log_input(INFO)]` on the function once; every call gets logged. |
| No runtime reflection | The arg names and function name are baked in at compile time. |
| Still type-checked | Because the macro emits real Rust, the compiler catches type errors normally. |

---

## 7. Recap

| Thing | One-liner |
|---|---|
| Definition | A proc-macro is just `fn(TokenStream) -> TokenStream`. |
| Crate type | Lives in its own crate with `[lib] proc-macro = true`. |
| Helpers | `syn` for parsing input; `quote` for generating output. |
| Multi-crate setup | Use a Cargo **workspace** to host the macro + consumer side by side. |
| Function-like | `#[proc_macro]` + `name!(args)` call style. |
| Attribute | `#[proc_macro_attribute]` + `#[name(ann)] item` — takes two `TokenStream`s. |
| Compile-time work | Runs at `cargo build` time, not at runtime. |

| Command | Does |
|---|---|
| `cargo build` | Build every workspace member |
| `cargo run -p proc_macros_bin` | Run the consumer binary |
| `cargo build -p proc_macros_lib` | Build just the proc-macro crate |
| `cargo expand -p proc_macros_bin` | See what the macro expanded into (install: `cargo install cargo-expand`) |

> Next lesson: **derive proc-macros** (`#[proc_macro_derive]`). A sample implementation (`#[derive(Hello)]`) already lives in [`proc_macros_lib/src/lib.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/procedural_macros/proc_macros_lib/src/lib.rs) — it generates a `Hello` trait impl on the annotated type.
