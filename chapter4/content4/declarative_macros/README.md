# Declarative Macros

A **declarative macro** is code that writes code. You give it a pattern + a template; Rust pastes the template in wherever you call it. They're invoked with a trailing `!` — like `println!`, `vec!`, `panic!`.

```
declarative_macros/
├── Cargo.toml
└── src/
    ├── main.rs              ← runs every demo
    ├── basics.rs            ← § 1
    ├── metavariables.rs     ← § 2
    ├── exporting.rs         ← § 3
    ├── scoping.rs           ← § 4
    └── patterns.rs          ← § 5
```

Run everything:

```bash
cargo run
```

---

## 1. Basics — [`src/basics.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/declarative_macros/src/basics.rs)

### Anatomy

Every declarative macro has this shape:

```rust
macro_rules! $name {
    ($matcher0) => { $expansion0 };
    ($matcher1) => { $expansion1 };
    // …
}
```

| Piece | ELI5 |
|---|---|
| `macro_rules!` | The keyword that defines a declarative macro. |
| `$name` | Any identifier — what you'll call it (with `!`). |
| `$matcher` | Pattern the call site must look like (similar to a `match` arm). |
| `$expansion` | The Rust code that replaces the call site. |
| Multiple rules | Like a `match`: the first matcher that fits wins. |

### `one!` — the simplest macro

```rust
macro_rules! one {
    () => { 1 };
}

let two = one!() + one!();   // expands to `1 + 1` → 2
```

`one!(1)` does **not** compile because no rule expects a token inside the parens:

```text
error: no rules expected the token `1`
```

Arity and shape are checked **at parse time**, not against types — that's the difference between a macro and a function.

### Fragment specifiers — `is_type!`

A fragment specifier tells the matcher "expect a piece of Rust syntax of **this kind**." Written `$name:kind`.

```rust
macro_rules! is_type { ($t:ty) => { /* no-op */ }; }

is_type!(i32);
is_type!(Vec<String>);
```

| Specifier | Matches |
|---|---|
| `expr` | An expression: `1 + 1`, `do_thing()`, `x.field`, … |
| `ident` | An identifier: `some_var`, `MyType`, … |
| `literal` | A literal: `42`, `"hi"`, `true`, … |
| `ty` | A type: `i32`, `Vec<String>`, `&'a str`, … |
| `lifetime` | A lifetime: `'static`, `'a`, … |
| `pat` | A pattern (the LHS of `let`). |
| `block` | A `{ … }` block. |
| `tt` | A single **token tree** (most permissive). |

Full list lives in the [Rust reference](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables).

### Repetition markers — `my_vec!`

Functions have a fixed arity; macros don't.

```rust
macro_rules! my_vec {
    ($($x:expr),*) => {{
        let mut tmp = Vec::new();
        $( tmp.push($x); )*
        tmp
    }};
}

let v = my_vec![1, 2, 3];   // [1, 2, 3]
```

| Piece | ELI5 |
|---|---|
| `$( … )` | Capture group — what's inside repeats. |
| `,` before the marker | Separator between repetitions. |
| `*` | Repeat **0 or more** times. |
| `+` | Repeat **1 or more** times. |
| `?` | Repeat **0 or 1** time. |

So `my_vec![1, 2, 3]` expands to a block that creates a `Vec` and pushes 1, 2, 3 into it — same shape as `std::vec!`.

---

## 2. Metavariable Expressions — [`src/metavariables.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/declarative_macros/src/metavariables.rs)

Special expressions that either **transform** macro input or **obtain information** about it. The most common one is `stringify!`.

```rust
macro_rules! example {
    ($x:ident) => {
        println!("{}", stringify!($x));
    };
}

example!(my_var);   // prints: my_var
```

| Piece | ELI5 |
|---|---|
| `stringify!($x)` | Take whatever tokens `$x` matched and turn them into a `&'static str`. |
| Why it matters | Without it, the expansion would be `println!("{}", my_var)` — and `my_var` isn't defined anywhere, so it wouldn't compile. |

> **Homework**: [The Little Book of Rust Macros](https://veykril.github.io/tlborm/) is the canonical deep dive — it covers every metavariable expression in detail.

---

## 3. Exporting Macros — [`src/exporting.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/declarative_macros/src/exporting.rs)

To let other crates use your macro, mark it with `#[macro_export]`:

```rust
#[macro_export]
macro_rules! my_macro {
    () => {
        $crate::exporting::some_function();
    };
}

pub fn some_function() {
    println!("Hello from `some_function` inside the current crate!");
}
```

| Piece | ELI5 |
|---|---|
| `#[macro_export]` | Make the macro callable from **outside** the crate (as `crate_name::my_macro!`). It also gets placed at this crate's root. |
| `$crate` | A magic metavariable that resolves to the crate **where the macro was defined**. |

### Why `$crate` matters

If the expansion just said `some_function()`, then when a **different crate** invoked `my_macro!()`, Rust would look up `some_function` in *that* crate's scope and fail with:

```text
error[E0425]: cannot find function `some_function` in this scope
```

`$crate::exporting::some_function` always points back to **your** crate, no matter where the call site lives.

| In your crate, this call works… | …because `$crate` becomes |
|---|---|
| `my_macro!()` from `main.rs` | the current crate root |
| `my_macro!()` from a downstream user | the path of *your* crate as seen by them |

---

## 4. Scoping Quirks — [`src/scoping.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/declarative_macros/src/scoping.rs)

Unlike everything else in Rust, declarative macros **remain visible in submodules** defined after the macro. They are only accessible *after* their definition point in source order.

### With functions — does **not** compile

```rust
pub fn some_function() { println!("Hello, World!"); }

mod a {
    pub(crate) fn test() {
        some_function();    // error[E0425]: cannot find function `some_function`
    }
}
```

A submodule does not inherit the parent's items — you'd need `use crate::some_function;`.

### With macros — **works**

```rust
macro_rules! some_macro { () => { println!("Hello, World!"); }; }

mod a {
    pub(crate) fn test() {
        some_macro!();     // ✅ visible without any `use`
    }
}
```

| Rule | ELI5 |
|---|---|
| Macros are textually scoped | They become visible at the point they're defined and stay visible in everything that follows in the same file — including submodules. |
| But not retroactively | If `mod a` is defined **before** `macro_rules! some_macro`, the macro is not visible inside `a`. |

---

## 5. Macro Patterns — [`src/patterns.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/declarative_macros/src/patterns.rs)

Three idioms you'll see again and again.

### 5.a Match All

Catches any tokens you didn't enumerate.

```rust
macro_rules! match_all {
    ($($other:tt)*) => {
        println!("Matched: {:?}", stringify!($($other)*));
    };
}

match_all!(if let Some(x) = Some(5) {});
// Matched: "if let Some(x) = Some(5) {}"
```

`$($other:tt)*` says "any number of token trees, captured under the name `$other`." Combined with `stringify!`, it's a great debugging trick.

### 5.b Callbacks

You can pass a **macro** to another macro as a `:ident`.

```rust
macro_rules! recognize_tree {
    (larch) => { println!("Recognized the Larch tree!"); };
    (oak)   => { println!("Recognized the Oak tree!"); };
    ($($other:tt)*) => { println!("Unknown tree!") };
}

macro_rules! call_with_larch {
    ($callback:ident) => { $callback!(larch) };
}

call_with_larch!(recognize_tree);   // calls recognize_tree!(larch)
```

| Piece | ELI5 |
|---|---|
| `$callback:ident` | Capture the **name** of another macro. |
| `$callback!(larch)` | Re-emit it as a macro invocation. |

### 5.c TT Munchers

A recursive pattern that consumes the input **one token tree at a time** until empty. Perfect for counting, accumulating, or walking arbitrary input.

```rust
macro_rules! count_tts {
    () => { 0 };
    ($_tt:tt $($rest:tt)*) => { 1 + count_tts!($($rest)*) };
}

count_tts!(a b c d e)   // → 5
```

| Rule | When it matches |
|---|---|
| `() => { 0 }` | Base case — nothing left. |
| `($_tt:tt $($rest:tt)*) => { … }` | Recursive case — peel off one token, recurse on the rest. |

Expansion of `count_tts!(a b c d e)`:

```
1 + count_tts!(b c d e)
1 + 1 + count_tts!(c d e)
1 + 1 + 1 + count_tts!(d e)
1 + 1 + 1 + 1 + count_tts!(e)
1 + 1 + 1 + 1 + 1 + count_tts!()
1 + 1 + 1 + 1 + 1 + 0
```

---

## Cheat Sheet

| Thing | One-liner |
|---|---|
| Define a macro | `macro_rules! name { ($matcher) => { $expansion }; }` |
| Call a macro | `name!(args)` — the `!` is mandatory |
| Multiple rules | Separate with `;`, first match wins |
| Type-check input | Fragment specifier (`$x:expr`, `$t:ty`, …) |
| Variable arity | Repetition: `$( … ),*` (`*` / `+` / `?`) |
| Token-to-string | `stringify!(expr)` → `&'static str` |
| Export to other crates | `#[macro_export]` + use `$crate::…` for any paths inside |
| Visible in submodules | Yes, after the definition |
| Catch-all | `($($other:tt)*)` |
| Recursion | Have the expansion invoke the macro again (TT muncher) |

| Command | Does |
|---|---|
| `cargo run` | Build + run every demo |
| `cargo expand` | See expanded code (install: `cargo install cargo-expand`) |
| `cargo check` | Parse + type-check, faster than `run` |
