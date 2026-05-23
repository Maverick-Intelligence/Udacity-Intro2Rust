# Chapter 2 — Generics in Rust

## The Story of "The One Rule That Works For Everything"

Imagine you have a toy that picks the biggest number from a list. You'd build it once for numbers, right? But then you realize — you also want to pick the biggest letter, or the biggest decimal! Do you build three separate toys? That sounds silly.

**Generics in Rust let you build one toy that works for many types.** This chapter shows exactly that idea, step by step.

---

## Your Code, Explained Simply

### Part 1: The "Before Generics" Version — Copying Rules

At first, you wrote **two separate functions** that do the exact same thing, just for different types:

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for i in 0..list.len() {
        if list[i] > largest {
            largest = list[i];
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for i in 0..list.len() {
        if list[i] > largest {
            largest = list[i];
        }
    }
    largest
}
```

**What's happening?** Both functions are identical in logic. They both:

1. Start with the **first item** as the current "biggest."
2. Look at every item in the list.
3. If the current item is **bigger** than the biggest so far, replace it.
4. Return the biggest item at the end.

**The problem?** The only difference is the type (`i32` vs `char`). You're writing the **same rule twice**. Imagine coloring two different toys the same color — why not just paint one toy, and say "this color works for all toys"?

---

### Part 2: The "After Generics" Version — One Rule to Rule Them All

Then you wrote **one function** that works for **any type** you give it:

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in 0..list.len() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }
    largest
}
```

**What changed?** Let me break it down like a secret code:

| Code | What It Means |
|---|---|
| `fn largest<T: ...` | "This function works for **any type** I'll call `T`." |
| `: std::cmp::PartialOrd` | "But `T` must be something I can **compare with `>`** (like numbers or letters)." |
| `list: &[T]` | "You give me a list of `T`s. I don't care what `T` is." |
| `-> &T` | "I'll give you back a reference to the biggest `T` I found." |

**The magic word is `<T>`** — it's a **placeholder**, like an empty box waiting for you to fill it with a type.

---

## How Generics Work (The "Box" Analogy)

Think of `<T>` as an **empty box** labeled "Type":

```
Before Generics:              After Generics:
┌──────────────┐             ┌───────────────────┐
│ largest_i32  │             │   largest<T>      │
│              │             │ ┌───────────────┐  │
│ Needs i32    │             │ │   ???         │  │
│              │             │ │   (your type) │  │
└──────────────┘             │ └───────────────┘  │
                             └───────────────────┘
                             You fill in the box!
```

When you call `largest(&list_i32)`, Rust **fills the box with `i32`**:

```
fn largest<i32>(list: &[i32]) -> &i32
```

When you call `largest(&list_char)`, Rust **fills the box with `char`**:

```
fn largest<char>(list: &[char]) -> &char
```

When you call `largest(&list_f32)`, Rust **fills the box with `f32`**:

```
fn largest<f32>(list: &[f32]) -> &f32
```

**One function. Three uses. Zero extra code.**

---

## The "You Can Only Compare Things" Rule

Not everything can be compared! Try comparing a `String` with `>` and the compiler will stop you. That's why this line matters:

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T])
```

**`PartialOrd`** is Rust's way of saying: **"T must be a type that knows how to do `>` comparisons."**

It's like a dance party where only people who know how to dance (`>`) can come in. Numbers? They know how to dance. Letters? They know how to dance. A `Book` struct without a dance rule? **Bouncer says no.**

---

## How Your Code Runs, Step by Step

### Step 1: You have three different lists

```rust
let list_i32  = [1234, 2345, 3456, 4567, 5678];
let list_char = ['a', 'b', 'c', 'd', 'e'];
let list_f32  = [0.12345, 1.23456, 2.34567, 3.45678, 4.56789];
```

Three lists. Three types. That's the **problem** generics solve.

### Step 2: You call `largest` on each list

```rust
let largest_temp_i32  = largest(&list_i32);   // T = i32
let largest_temp_char = largest(&list_char);   // T = char
let largest_temp_f32  = largest(&list_f32);    // T = f32
```

**One call. Three types.** Rust automatically fills in the `T` box for you.

### Step 3: You see the results

```rust
println!("The largest i32 in 'list_temp_i32' is {}", largest_temp_i32);   // 5678
println!("The largest char in 'list_temp_char' is {}", largest_temp_char); // e
println!("The largest f32 in 'list_temp_f32' is {}", largest_temp_f32);    // 4.56789
```

---

## Why This Is a Big Deal

| Before Generics | After Generics |
|---|---|
| Write `largest_i32` | Write `largest<T>` once |
| Write `largest_char` | One call works for `char` |
| Write `largest_f32` | One call works for `f32` |
| **57 lines of repeated code** | **A few lines of reusable code** |
| Fix bugs in 3 places | Fix bugs in 1 place |
| Remember 3 names | Remember 1 name |

**Generics = Write once, use everywhere.** It's Rust's version of "build one, sell a million."

---

## Another Way to Write the Same Thing (Bounds on the `impl`)

There's another style Rust programmers use. Instead of putting `T: PartialOrd` on the function, you put it on the type:

```rust
fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
```

It does the **exact same thing**. Some people prefer this style when the function gets long — it keeps the top of the function clean. Think of it like a restaurant menu:

```
fn largest<T>(list: &[T]) -> &T   ← the menu title (clean!)
where
    T: PartialOrd                 ← the ingredients list (at the bottom)
```

---

## Summary: The Big Picture

```
Problem: "I need to find the biggest thing, but for many types!"

Bad solution: Write 100 separate functions (one per type). 🤯
Good solution: Write 1 function with a type placeholder `T`. ✨
