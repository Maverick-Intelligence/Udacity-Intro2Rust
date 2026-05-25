# Chapter 1 — Ownership, Borrowing, and Lifetime

Rust's safety story, built up from three rules: every value has one owner, every reference is checked, and no reference ever outlives the thing it points to.

| What | Where |
|---|---|
| **Content** — concepts, ELI5 tables, runnable examples | [`content1/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/README.md) |
| **Exercise** — small project that puts the concepts into practice | [`exercise1/book_registry/`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/exercise1/book_registry) |

## What's inside the content

| Section | Topic |
|---|---|
| 1.1 | **Ownership** — memory management without GC; Move / Copy / Clone |
| 1.2 | **Borrowing** — `&` / `&mut`, the borrow checker's two rules, NLL |
| 1.3 | **Lifetime** — references, elision rules, `'static`, struct lifetimes |

Run every example with:

```bash
cd content1 && cargo run
```
