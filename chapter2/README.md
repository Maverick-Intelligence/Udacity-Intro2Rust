# Chapter 2 — Generics, Traits, and Error Handling

The three tools you need to write reusable, type-safe Rust: one function for many types, shared behaviour via contracts, and errors as values you can't silently ignore.

| What | Where |
|---|---|
| **Content** — concepts, ELI5 tables, runnable examples | [`content2/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/content2/README.md) |
| **Exercise** — generic calculator that combines all three concepts | [`exercise2/calculator/`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter2/exercise2/calculator) |

## What's inside the content

| Section | Topic |
|---|---|
| 2.1 | **Generics** — `<T: PartialOrd>` and friends; one function for `i32`, `char`, `f32`, … |
| 2.2 | **Traits** — shared interfaces, default methods, override-by-impl |
| 2.3 | **Error Handling** — `unwrap` / `expect` / `Result` / `match` / `?` |

Run every example with:

```bash
cd content2 && cargo run
```

Run the exercise project:

```bash
cd exercise2/calculator && cargo run
```
