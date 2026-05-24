# Chapter 3 — Shape Calculator

A tiny **library** that makes three shapes and computes each one's area: `Circle`, `Rectangle`, `Square`.

```
shape_calculator/
├── Cargo.toml
├── src/
│   ├── lib.rs              ← library front door
│   └── shapes/
│       ├── mod.rs          ← folder's table of contents
│       ├── shape.rs        ← trait + generic area function
│       ├── circle.rs
│       ├── rectangle.rs
│       └── square.rs
├── examples/shape_calculator.rs   ← runnable demo
├── tests/test.rs                  ← integration tests
└── benches/my_bench.rs            ← Criterion benchmarks
```

---

## 3.1 The Library

### Library crate — [`lib.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/src/lib.rs)

```rust
pub mod shapes;
pub fn get_shapes() { ... }
```

A **library** has no `main`. Other code uses it by name. `pub` = "outside world can see this."

### Module system — [`shapes/mod.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/src/shapes/mod.rs)

```rust
pub mod circle;
pub mod rectangle;
mod shape;          // private to this folder
pub mod square;
```

A folder is a module when it has a `mod.rs`. Each `.rs` inside is a sub-module.

| Keyword | ELI5 |
|---|---|
| `mod x;` | "Use the file `x.rs`." |
| `pub mod x;` | "…and let outsiders see it." |
| `use super::shape::area;` | "Go up one folder and grab `area` from `shape`." |

### Traits + Generics — [`shape.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/src/shapes/shape.rs)

```rust
pub trait AreaFor {
    fn area_for(&self, t: u8) -> f32;
}

impl AreaFor for f32          { /* circle / square math */ }
impl AreaFor for (f32, f32)   { /* rectangle math */ }

pub fn area<T: AreaFor>(t: u8, num: T) -> f32 { num.area_for(t) }
```

| Word | ELI5 |
|---|---|
| `trait` | A **promise**: "any type that implements me has these methods." |
| `impl Trait for Type` | "Here's how `Type` keeps that promise." |
| `<T: AreaFor>` | "Any type **T**, as long as it implements `AreaFor`." |
| `match t { 1 => … }` | A switch that picks one branch based on a value. `_` means "anything else." |
| `*self` | Look at what the reference points to (read the value, not the address). |

So `area(t, num)` is **one function** that works for two completely different `num` types — Rust picks the matching `impl` at compile time.

### Shape structs — [`circle.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/src/shapes/circle.rs) · [`rectangle.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/src/shapes/rectangle.rs) · [`square.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/src/shapes/square.rs)

All three follow the same pattern:

```rust
#[derive(Debug)]
pub struct Circle {
    t: u8,             // private
    pub rad: f32,      // readable from outside
    pub area: f32,
}

impl Circle {
    pub fn new(rad: f32) -> Self {        // constructor
        let t = 1;
        Self { t, rad, area: area(t, rad) }
    }

    pub fn print(&self) {                 // method on an instance
        println!("…");
    }
}
```

| Concept | ELI5 |
|---|---|
| `struct` | A bundle of named fields. |
| `#[derive(Debug)]` | Free auto-printer for `{:?}`. |
| `impl Type { … }` | Where methods live. |
| `Self` | Shortcut for "this struct's type." |
| `pub fn new(...) -> Self` | Constructor. Called like `Circle::new(5.0)`. |
| `&self` | "Borrow me — don't take ownership." |
| `Self { t, rad, … }` | Field shorthand — `t` instead of `t: t`. |
| `pub` on a field | Field is readable/writable from outside. No `pub` = private. |

The shape's `t` (1, 2, 3) is just a private tag so `area(t, num)` picks the right math.

| Shape | `t` | Input | Area math |
|---|---|---|---|
| Circle | 1 | `rad: f32` | `3.14 * rad * rad` |
| Rectangle | 2 | `(w, l): (f32, f32)` | `w * l` |
| Square | 3 | `dim: f32` | `dim * dim` |

---

## 3.2 Examples — [`examples/shape_calculator.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/examples/shape_calculator.rs)

Anything in `examples/` is a runnable demo that uses the library from the outside, like real users would.

```bash
cargo run --example shape_calculator
```

---

## 3.3 Tests — [`tests/test.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/tests/test.rs)

Anything in `tests/` is an **integration test** — a separate binary that imports the library as `shape_calculator::…`. It can only see `pub` things.

```rust
#[test]
fn test_create_circle() {
    let c = Circle::new(5.0);
    assert!((c.area - 78.5).abs() < 1e-4);
}
```

| | Unit tests (inside `src/`) | Integration tests (`tests/`) |
|---|---|---|
| Import path | `crate::...` | `shape_calculator::...` |
| Sees private items? | Yes | No |

```bash
cargo test                       # run all tests
cargo test -- --show-output      # also show println! output
```

> `assert!((a - b).abs() < 1e-4)` is the safe way to compare two `f32`s — they're imprecise, so checking "close enough" beats `==`.

---

## 3.4 Benchmarks — [`benches/my_bench.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/shape_calculator/benches/my_bench.rs)

`benches/` measures how fast code runs, using the **Criterion** crate.

```rust
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn benchmark_circle_area(c: &mut Criterion) {
    c.bench_function("circle_area", |b| b.iter(|| Circle::new(black_box(5.0))));
}

criterion_group!(benches, benchmark_circle_area, /* … */);
criterion_main!(benches);
```

| Thing | ELI5 |
|---|---|
| `Criterion` | Stopwatch that runs your code lots of times, then reports mean + stddev. |
| `b.iter(\|\| …)` | The thing being timed. |
| `black_box(…)` | Stops the compiler from cheating by precomputing the answer. |
| `criterion_group!` + `criterion_main!` | The two macros that build the bench binary's `main`. |

`Cargo.toml` registers the bench:

```toml
[dependencies]
criterion = "0.8.2"

[[bench]]
name = "my_bench"
harness = false        # let Criterion provide main, not the default test harness
```

```bash
cargo bench
```

HTML report opens at `target/criterion/report/index.html`.

---

## Cheat Sheet

| Command | Does |
|---|---|
| `cargo build` | Compile the library |
| `cargo run --example shape_calculator` | Run the demo |
| `cargo test` | Run unit + integration tests |
| `cargo test -- --show-output` | …with `println!` shown |
| `cargo bench` | Run Criterion benchmarks |
