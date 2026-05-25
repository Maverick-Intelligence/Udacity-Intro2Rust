# Chapter 3 — Modules, Tooling, and Workflow

How Rust organises code (modules in one file vs many files) and the everyday tools that ship with it: `cargo fmt`, `clippy`, `anyhow`, tests, `cargo doc`, and Criterion benchmarks.

| What | Where |
|---|---|
| **Content** — modules, visibility, paths + all the workflow tools | [`content3/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/content3/README.md) |
| **Exercise — Shape Calculator** — library + examples + tests + benches | [`exercise3/shape_calculator/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/exercise3/shape_calculator/README.md) |
| **Exercise — String Search Benchmark** — head-to-head `contains` vs `find` with Criterion | [`exercise3/string_search_benchmark/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter3/exercise3/string_search_benchmark/README.md) |

## What's inside the content

| Section | Topic |
|---|---|
| 3.1 | **Modules** — inline, flat file, folder with `mod.rs` |
| 3.2 | **Visibility & Paths** — `pub`, `pub(crate)`, `pub(super)`, `self::`, `super::`, `crate::` |
| 3.3 | **`cargo fmt`** — auto-formatter |
| 3.4 | **`cargo clippy`** — linter |
| 3.5 | **`anyhow`** — easy error handling for apps |
| 3.6 | **Testing** — unit vs integration vs doctests |
| 3.7 | **`cargo doc`** — HTML docs from `///` comments |
| 3.8 | **Criterion** — benchmarking |

Run the example:

```bash
cd content3 && cargo run
```

Run an exercise:

```bash
cd exercise3/shape_calculator && cargo test
cd exercise3/string_search_benchmark && cargo bench
```
