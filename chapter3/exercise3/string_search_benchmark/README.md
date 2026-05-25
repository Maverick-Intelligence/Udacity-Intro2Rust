# String Search Benchmark

A tiny **library** that finds a string (the "needle") inside another string (the "haystack") two ways — `contains` and `find` — plus a Criterion **benchmark** that races them head-to-head.

```
string_search_benchmark/
├── Cargo.toml
├── src/
│   └── lib.rs              ← library + unit tests
└── benches/
    └── benchmark.rs        ← Criterion benchmarks
```

---

## The Library — [`src/lib.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/exercise3/string_search_benchmark/src/lib.rs)

### A result enum for logging

```rust
#[derive(Debug)]
#[allow(dead_code)]
enum ResultType {
    Boolean(bool),
    OptUSize(Option<usize>),
}
```

| Word | ELI5 |
|---|---|
| `enum` | A type that can be **one of these shapes**. Either a `Boolean(bool)` or an `OptUSize(Option<usize>)`. |
| `#[derive(Debug)]` | Free auto-printer for `{:?}`. |
| `#[allow(dead_code)]` | Tells the compiler "the inner value is only used by `Debug` — don't warn me." Rust's dead-code lint **intentionally ignores** derived `Debug` impls, so you have to allow it explicitly. |

### The two search functions

```rust
pub fn search_contains(haystack: &str, needle: &str, is_logged: bool) -> bool {
    let is_found = haystack.contains(needle);
    if is_logged {
        print(haystack, needle, ResultType::Boolean(is_found));
    }
    is_found
}

pub fn search_find(haystack: &str, needle: &str, is_logged: bool) -> Option<usize> {
    let is_found = haystack.find(needle);
    if is_logged {
        print(haystack, needle, ResultType::OptUSize(is_found));
    }
    is_found
}
```

| Function | Wraps | Returns | Question it answers |
|---|---|---|---|
| `search_contains` | `str::contains(p)` | `bool` | "Is the needle **anywhere** in the haystack?" |
| `search_find` | `str::find(p)` | `Option<usize>` | "**Where** does the needle start?" — `Some(index)` or `None`. |

| Parameter | ELI5 |
|---|---|
| `haystack: &str` | Borrow the big string — don't take ownership. |
| `needle: &str` | Borrow the pattern. |
| `is_logged: bool` | Flag to skip the `println!`. Printing is **much** slower than searching, so during benchmarks we pass `false` to keep the measurement honest. |

Under the hood `str::contains(p)` is literally `str::find(p).is_some()` — the work is the same, only the return type differs.

### Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haystack_contain_needle() {
        let haystack = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let needle: &str = "PQR";
        assert!(search_contains(&haystack, needle, true));
    }

    #[test]
    fn test_needle_found_in_haystack() {
        let haystack = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let needle: &str = "PQR";
        assert_eq!(search_find(&haystack, needle, true), Some(15));
    }
}
```

| Thing | ELI5 |
|---|---|
| `#[cfg(test)] mod tests` | This module only compiles when running `cargo test`. |
| `use super::*;` | Pull every item from the parent module into scope. |
| `#[test]` | "This function is one test." |
| `assert!(x)` | Crashes the test if `x` is `false`. |
| `assert_eq!(a, b)` | Crashes if `a != b`, and prints both sides on failure. |

Run them with:

```bash
cargo test                       # all tests
cargo test -- --show-output      # also show `println!`s
```

---

## The Benchmark — [`benches/benchmark.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/exercise3/string_search_benchmark/benches/benchmark.rs)

### Build a deliberately hard input

```rust
fn produce_haystack_needle() -> (String, &'static str) {
    let lorem  = "Lorem ipsum dolor sit amet, …";
    let needle = "Lorem ipsum dolor sit amet, … magna aliquaXYZZY42";

    let haystack = format!("{}{}", lorem.repeat(1_000_000), needle);
    (haystack, needle)
}
```

| Piece | ELI5 |
|---|---|
| `-> (String, &'static str)` | Returns a tuple: an owned `String` (the haystack) and a borrowed `&str` (the needle). |
| `'static` | The needle is a string literal baked into the binary — it lives for the whole program. |
| `lorem.repeat(1_000_000)` | Stamp the Lorem block out one million times. The haystack is huge on purpose. |
| `format!("{}{}", …)` | Glue the repeated block and the needle together into one big `String`. |
| `(haystack, needle)` (no `;`) | Last expression of the function = return value. |

**Why this input is hard.** The needle starts with ~110 characters of standard Lorem Ipsum — the same prefix that appears in **every** repetition of the haystack. So the search algorithm partially matches at the start of every repetition, then has to discover that the *real* needle ends in `XYZZY42` while the haystack body ends in `". Ut enim…"`. The actual match is **only at the very end** of the haystack — worst case on purpose.

### One benchmark per function

```rust
fn benchmark_search_contains(c: &mut Criterion) {
    let (haystack, needle) = produce_haystack_needle();

    c.bench_function("search_contains", |b| {
        b.iter(|| {
            search_contains(
                std::hint::black_box(&haystack),
                std::hint::black_box(needle),
                std::hint::black_box(false),
            )
        })
    });
}
```

| Piece | ELI5 |
|---|---|
| `c: &mut Criterion` | Criterion is the stopwatch — borrow it mutably so we can register a bench on it. |
| `let (haystack, needle) = …` | **Destructuring**: pull both tuple fields into their own locals. |
| `c.bench_function("name", \|b\| …)` | Register one benchmark named `"name"`. |
| `b.iter(\|\| …)` | The closure runs many times; Criterion records mean + stddev. |
| `std::hint::black_box(x)` | "Pretend this value is unknown at compile time" — stops the optimizer from precomputing the answer and skipping the search. |
| `is_logged = false` | Skip the `println!` inside the library so we time the **search**, not the print. |

`benchmark_search_find` has the same shape and uses the same `produce_haystack_needle()` — apples-to-apples comparison.

### Wire it into a `main()`

```rust
criterion_group!(benches, benchmark_search_contains, benchmark_search_find);
criterion_main!(benches);
```

| Macro | ELI5 |
|---|---|
| `criterion_group!` | Bundle several `fn(&mut Criterion)`s into one group called `benches`. |
| `criterion_main!` | Generate the `fn main()`. Required because `Cargo.toml` says `harness = false`. |

### `Cargo.toml` wiring

```toml
[dependencies]
criterion = "0.8.2"

[[bench]]
name = "benchmark"     # matches the file name: benches/benchmark.rs
harness = false        # let Criterion provide main, not Cargo's default test harness
```

Run:

```bash
cargo bench
```

HTML report opens at `target/criterion/report/index.html`. Expect `search_contains` and `search_find` to land within a few percent of each other — `contains` is literally `find().is_some()` under the hood.

---

## Cheat Sheet

| Command | Does |
|---|---|
| `cargo build` | Compile the library |
| `cargo test` | Run unit tests |
| `cargo test -- --show-output` | …with `println!` shown |
| `cargo bench` | Run Criterion benchmarks |
