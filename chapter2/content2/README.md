# Chapter 2 — Generics, Traits, Error Handling, and a Calculator Project

Rust gives you three powerful tools for writing safe, reusable code:

| Concept | One-line meaning |
|---|---|
| **Generics** | One function/struct that works for many types. |
| **Traits** | A shared contract that multiple types can agree to follow. |
| **Error Handling** | Errors are values (`Result<T, E>`), not crashes. |

The code lives in four modules, run them all with `cargo run`:

| Chapter | Module | Topic |
|---|---|---|
| 2.1 | [`src/generics.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/src/generics.rs) | Generics |
| 2.2 | [`src/traits.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/src/traits.rs) | Traits |
| 2.3 | [`src/error_handling.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/src/error_handling.rs) | Error Handling |
| 2.4 | [`src/project.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/exercise2/calculator/src/calculator.rs) | Calculator Project (all three combined) |

---

## 2.1 Generics

**Generics** let you write **one function that works for many types** instead of duplicating the same logic for each type.

### 2.1.1 Type-Specific (the problem)

Without generics you write the same algorithm twice — once per type:

```rust
pub fn largest_i32(list: &[i32]) -> i32 { /* find max */ }
pub fn largest_char(list: &[char]) -> char { /* find max */ }
```

Both functions are identical except for the type. Duplication scales badly.

### 2.1.2 Generic (the fix)

`<T>` says *"any type"*; the trait bound `: PartialOrd` says *"…that supports `>`"*:

```rust
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in 0..list.len() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }
    largest
}
```

| Part | Meaning |
|---|---|
| `<T>` | "Any type — I don't care what it is." |
| `: PartialOrd` | "But it must support `>` comparison." |
| `list: &[T]` | "A slice of that type." |
| `-> &T` | "I return a reference to the biggest one." |

| Type | Implements `PartialOrd`? | Works with `largest`? |
|---|---|---|
| `i32` | Yes | Yes |
| `char` | Yes | Yes |
| `f32` | Yes | Yes |
| `String` | No | Compile-time error |

[`execute_generics_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/src/generics.rs)

---

## 2.2 Traits

**Traits** define a **shared contract** — a set of methods that multiple types can agree to implement. Think of it as a recipe card on the wall: every type reads it and decides how to follow it.

### 2.2.1 Trait Without Default (`Animal`)

The trait declares the method signature only; every type **must** provide its own body.

```rust
pub(crate) trait Animal {
    fn says(&self) -> String;   // no body
}

impl Animal for Dog { fn says(&self) -> String { self.says.clone() } }
impl Animal for Cat { fn says(&self) -> String { self.says.clone() } }
```

| Type | Writes its own `says()`? | Output |
|---|---|---|
| `Dog` | Yes | `"Woof"` |
| `Cat` | Yes | `"Meow"` |

Same interface, different behaviour.

### 2.2.2 Trait With Default (`GreetingDefault`)

The trait provides a default body; types may **use** it or **override** it.

```rust
pub(crate) trait GreetingDefault {
    fn speak(&self) -> String {
        "Hello everyone!".to_string()      // default
    }
}

impl GreetingDefault for GreetingEnglish {}              // uses default

impl GreetingDefault for GreetingGerman {
    fn speak(&self) -> String { self.speak.clone() }     // overrides
}
```

| Type | Uses default? | `speak()` returns |
|---|---|---|
| `GreetingEnglish` | Yes | `"Hello everyone!"` (from the trait) |
| `GreetingGerman` | No (overrides) | `"Moin!"` (its own value) |

[`execute_trait_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/src/traits.rs)

---

## 2.3 Error Handling

In Rust, errors are **values** carried in `Result<T, E>` or `Option<T>` — not exceptions, not crashes. You decide how to handle them.

| Tool | What it does | When to use |
|---|---|---|
| `unwrap()` | Returns the inner value or **panics** | Only when failure is impossible |
| `expect("msg")` | `unwrap()` plus a custom panic message | Same as `unwrap` but with context |
| `Result<T, E>` + `match` | Force the caller to handle both outcomes | Most cases |
| `?` operator | Returns the `Err` early, otherwise unwraps the `Ok` | Inside `Result`-returning functions |

### 2.3.1 `unwrap()`

```rust
let some_option = Some(42);
let value = some_option.unwrap();   // 42
// If it were None, the program would PANIC.
```

### 2.3.2 `expect(msg)`

```rust
let value = some_option.expect("Some option is None");
```

Same behaviour as `unwrap`, but the panic message helps you locate the bug.

### 2.3.3 Recoverable Error with `match`

```rust
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err(String::from("Division by zero is not allowed")) } else { Ok(a / b) }
}

match divide(10, 0) {
    Ok(value)  => println!("Result: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

| Return | Inside the box | Meaning |
|---|---|---|
| `Ok(42)` | The answer | Success |
| `Err("...")` | The error message | Failure |

### 2.3.4 Try Operator (`?`)

`?` is the lazy shortcut inside functions that already return `Result`:

```rust
pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;   // early-return on Err
    Ok(content)
}
```

| Outcome | What `?` does |
|---|---|
| `Ok(value)` | Unwraps the value, keeps going |
| `Err(e)` | Returns the error immediately from the current function |

Without `?` (5 lines):

```rust
let content = match std::fs::read_to_string(file_path) {
    Ok(v)  => v,
    Err(e) => return Err(e),
};
```

With `?` (1 line):

```rust
let content = std::fs::read_to_string(file_path)?;
```

[`execute_error_handling_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/src/error_handling.rs)

---

## 2.4 Calculator Project

The [`project.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/exercise2/calculator/src/calculator.rs) module ties Generics + Traits + Error Handling together in one small program.

### The `Operation` enum

```rust
enum Operation { Add, Subtract, Multiply, Divide }
```

### The generic `calculate` function

```rust
pub fn calculate<T>(op: Operation, a: T, b: T) -> Result<T, &'static str>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + PartialEq
        + From<u8>,
{
    match op {
        Operation::Add      => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide   => {
            if b == T::from(0) { Err("Zero division is illegal!") } else { Ok(a / b) }
        }
    }
}
```

| Where each concept shows up | How |
|---|---|
| **Generics** | `<T>` plus the `where` clause make `calculate` work for any numeric type that supports `+ - * /`, `==`, and conversion from `0`. |
| **Traits** | The bounds (`Add`, `Sub`, `Mul`, `Div`, `PartialEq`, `From<u8>`) are all traits the type must implement. |
| **Error Handling** | The function returns `Result<T, &'static str>`; the divide-by-zero guard returns `Err(...)`. |

### What the caller does

```rust
project::calculator(21, 21, 84, 42, 21, 2, 84, 2, 42);
```

Runs five operations through `calculate` and `match`es each `Result`:

| # | Operation | Result |
|---|---|---|
| 2.4.1 | `21 + 21` | `Ok(42)` |
| 2.4.2 | `84 - 42` | `Ok(42)` |
| 2.4.3 | `21 * 2` | `Ok(42)` |
| 2.4.4 | `84 / 2` | `Ok(42)` |
| 2.4.5 | `42 / 0` | `Err("Zero division is illegal!")` |

[`calculator`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/exercise2/calculator/src/calculator.rs)

---

## Final Summary

| Concept | What It Solves | In One Line |
|---|---|---|
| **Generics** | Same logic for many types | Write once, use everywhere |
| **Traits** | Shared interface across types | Same rules, different flavours |
| **Error Handling** | Survive when things go wrong | Errors are values, not crashes |
| **Project** | All three working together | A tiny generic, trait-bounded, error-aware calculator |
