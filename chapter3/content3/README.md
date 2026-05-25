# Chapter 3 — Modules, Tooling, and Workflow

A walk-through of how Rust organises code into **modules**, plus the everyday tools (`cargo fmt`, `clippy`, `anyhow`, tests, `cargo doc`, Criterion) that go around them. The running example is a tiny restaurant where the dining area takes an order and the kitchen prepares the dish.

```
content3/
├── Cargo.toml
└── src/
    └── main.rs        ← restaurant example, all in one file
```

---

## 1. What is a module?

A **module** is a **room** in your code's house. Things inside the room are **private** by default — outsiders can't see them unless you tag them `pub`.

[`src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/content3/src/main.rs)

```rust
mod kitchen_area {
    pub(super) fn make_dish(dish: String) {
        println!("Preparing {}...", dish);
    }
}

mod dining_area {
    pub(super) fn order_apple() {
        println!("Ordering apple.");
        super::kitchen_area::make_dish("apple".to_string());
    }
}

fn main() {
    dining_area::order_apple();
}
```

| Word | ELI5 |
|---|---|
| `mod kitchen_area { … }` | "Make a room called `kitchen_area` right here." |
| `pub(super) fn make_dish` | "The room above me (the parent module) can see this — nobody else." |
| `super::kitchen_area::make_dish` | "From inside `dining_area`, walk up one level (`super`) then into the kitchen and call `make_dish`." |
| `dining_area::order_apple()` from `main` | `main` is the parent of both rooms, so it can poke into either. |

### Run it

```bash
cargo run
```

```
Ordering apple.
Preparing apple...
```

---

## 2. Modules in **one** file (inline)

What [`src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/content3/src/main.rs) does today: both rooms live in curly braces inside `main.rs`.

```rust
mod kitchen_area { /* ... */ }
mod dining_area  { /* ... */ }

fn main() { /* ... */ }
```

Good when modules are **small**. When they grow, split them out.

---

## 3. Modules in **multiple** files

The compiler treats each `.rs` file as the body of a module with the same name. Three equivalent layouts for the same restaurant:

### 3.a Flat — one file per module (modern Rust 2018+)

```
src/
├── main.rs
├── kitchen_area.rs
└── dining_area.rs
```

`main.rs`:

```rust
mod kitchen_area;     // body lives in kitchen_area.rs
mod dining_area;      // body lives in dining_area.rs

fn main() { dining_area::order_apple(); }
```

`kitchen_area.rs`:

```rust
pub(super) fn make_dish(dish: String) { println!("Preparing {}...", dish); }
```

`dining_area.rs`:

```rust
pub(super) fn order_apple() {
    println!("Ordering apple.");
    super::kitchen_area::make_dish("apple".to_string());
}
```

### 3.b Folder with `mod.rs` (older style, still valid)

```
src/
├── main.rs
└── restaurant/
    ├── mod.rs
    ├── kitchen_area.rs
    └── dining_area.rs
```

`restaurant/mod.rs` declares the sub-modules: `pub mod kitchen_area; pub mod dining_area;`.

### 3.c Folder with a sibling `restaurant.rs` (also modern)

```
src/
├── main.rs
├── restaurant.rs              ← acts as restaurant/mod.rs
└── restaurant/
    ├── kitchen_area.rs
    └── dining_area.rs
```

| Layout | When to use |
|---|---|
| **Inline** (`mod foo { … }`) | Module is tiny / experimental. |
| **Flat file** (`foo.rs`) | Module has nothing nested inside it. |
| **Folder** (`foo/mod.rs` or `foo.rs` + `foo/`) | Module itself contains sub-modules. |

---

## 4. Visibility — who can see what?

| Keyword | ELI5 |
|---|---|
| (no keyword) | **Private**. Only this module can see it. |
| `pub` | Everyone, even outside the crate. |
| `pub(crate)` | Only code inside this crate (this library/binary). |
| `pub(super)` | Only the **parent** module. Used in the example. |
| `pub(in path)` | Only the module at `path`. Rare. |

## 5. Paths — `self`, `super`, `crate`

| Prefix | Means |
|---|---|
| `self::` | "This module." |
| `super::` | "One module up." (Used in the example to reach `kitchen_area` from inside `dining_area`.) |
| `crate::` | "The root of this crate." Always absolute. |
| `use foo::bar;` | Import `bar` so you can just write `bar` instead of `foo::bar`. |

---

## 6. `cargo fmt` — auto-formatter (rustfmt)

Reformats every `.rs` file to the official Rust style. No arguments, no debate.

```bash
cargo fmt              # rewrite files
cargo fmt -- --check   # CI-friendly: exit non-zero if anything would change
```

Optional config file `rustfmt.toml` at the project root if you want to tweak (e.g. `max_width = 100`).

---

## 7. `cargo clippy` — linter

Catches likely bugs and "you could write this nicer" issues. Built on top of the compiler.

```bash
cargo clippy                            # warnings
cargo clippy -- -D warnings             # promote warnings to errors (great for CI)
cargo clippy --fix                      # auto-apply safe suggestions
```

| Example lint | What it catches |
|---|---|
| `needless_return` | `return x;` at end of function. |
| `unwrap_used` | `.unwrap()` on a `Result`/`Option`. |
| `clone_on_copy` | `.clone()` on a `Copy` type like `i32`. |

---

## 8. `anyhow` — easy error handling

Add to `Cargo.toml`:

```toml
[dependencies]
anyhow = "1"
```

Then any error type can be `?`-propagated through one return type:

```rust
use anyhow::{Context, Result};

fn read_dish_list(path: &str) -> Result<String> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("could not read {}", path))?;   // adds a hint
    Ok(contents)
}
```

| Thing | ELI5 |
|---|---|
| `anyhow::Result<T>` | Shortcut for `Result<T, anyhow::Error>` — accepts any error type via `?`. |
| `.with_context(\|\| "...")` | Attaches a human note that travels with the error up the call stack. |
| Use it for | Apps and binaries. |
| **Don't** use it for | Libraries where callers want typed errors — use `thiserror` instead. |

---

## 9. Testing

Three places tests can live:

| Where | What it is | Sees private items? |
|---|---|---|
| `#[cfg(test)] mod tests { … }` inside a `.rs` | **Unit test** | Yes |
| `tests/foo.rs` | **Integration test** (separate binary) | No — only `pub` |
| `/// # Examples …` in a `///` doc comment | **Doc test** (Section 10) | No |

Boilerplate:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_runs() {
        // assertions
    }
}
```

```bash
cargo test                       # everything
cargo test --doc                 # only doctests
cargo test -- --show-output      # show println!s
```

---

## 10. Documentation — `cargo doc`

Triple-slash comments (`///`) become HTML pages on `cargo doc`. Code blocks inside doc comments are **compiled and executed** as doctests.

```rust
/// Prepares a dish in the kitchen.
///
/// # Examples
///
/// ```
/// kitchen_area::make_dish("apple".to_string());
/// ```
pub(super) fn make_dish(dish: String) { /* ... */ }
```

| Tag | Meaning |
|---|---|
| `///` | Doc comment on the item below. |
| `//!` | Doc comment on the **enclosing** module/crate (put at the top of the file). |
| `# Examples` | Convention — H1 section name `rustdoc` recognises. |

```bash
cargo doc --open       # build HTML and open in browser
cargo test --doc       # run only the code blocks in docs
```

---

## 11. Benchmarking — Criterion

Cargo's built-in `#[bench]` is unstable; **everyone uses Criterion** instead.

`Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.8"

[[bench]]
name = "my_bench"
harness = false        # let Criterion provide main
```

`benches/my_bench.rs`:

```rust
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_order(c: &mut Criterion) {
    c.bench_function("order_apple", |b| b.iter(|| black_box(/* call */)));
}

criterion_group!(benches, bench_order);
criterion_main!(benches);
```

| Thing | ELI5 |
|---|---|
| `b.iter(\|\| …)` | The closure runs many times; Criterion records mean + stddev. |
| `black_box(x)` | Stops the optimizer from precomputing the answer. |
| `harness = false` | Required because Criterion supplies its own `main`. |

```bash
cargo bench
# HTML report → target/criterion/report/index.html
```

A worked two-bench example is in [`chapter3/exercise3/string_search_benchmark`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/exercise3/string_search_benchmark/benches/benchmark.rs).

---

## Cheat Sheet

| Command | Does |
|---|---|
| `cargo run` | Build and run the binary |
| `cargo fmt` | Auto-format every `.rs` file |
| `cargo fmt -- --check` | CI: fail if files aren't formatted |
| `cargo clippy` | Lint (suggest improvements) |
| `cargo clippy --fix` | Apply safe lint suggestions |
| `cargo test` | Run unit + integration + doctests |
| `cargo test --doc` | Doctests only |
| `cargo doc --open` | Build HTML docs and open them |
| `cargo bench` | Run Criterion benchmarks |
