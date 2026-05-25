# Chapter 1 — Ownership, Borrowing, and Lifetime

Rust guarantees memory safety **at compile time** without a garbage collector and without manual `free`. It does this with three ideas:

| Concept | One-line meaning |
|---|---|
| **Ownership** | Every value has one owner; freed when the owner goes out of scope. |
| **Borrowing** | Other code can read or mutate a value through references (`&` / `&mut`). |
| **Lifetime** | The compiler proves a reference never outlives the value it points to. |

The code lives in three modules, run them all with `cargo run`:

| Chapter | Module | Topic |
|---|---|---|
| 1.1 | [`src/memory_management.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/memory_management.rs) | Ownership |
| 1.2 | [`src/borrowing.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs) | Borrowing |
| 1.3 | [`src/lifetimes.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs) | Lifetime |

> **Failing examples**: compile-time errors can't be observed at runtime, so each `_fail` example **simulates** the bad state and returns a `Result` handled with `match`. In real Rust the `Err` branch is unreachable — the compiler rejects the code first.

---

## 1.1 Ownership

### 1.1.1 Memory Management

Most languages handle memory in one of two ways. Rust adds a third.

| Style | Example | Who frees memory? |
|---|---|---|
| Manual | C | You write `free()` |
| Garbage Collected | Java, Go | Runtime cleans up later |
| Ownership Model | Rust | Compiler inserts cleanup at compile time |

So in Rust you write this and **never** call `free`:

```rust
let array = vec![0; 10]; // dropped automatically at end of scope
```

[`memory_management_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/memory_management.rs)

### 1.1.2 Variable Interaction (Move, Copy, Clone)

When you write `let b = a;`, Rust does one of three things based on the type of `a`:

| Way | When it happens | What happens to `a`? | Cost | Example types |
|---|---|---|---|---|
| **Move** | Type owns heap data (no `Copy`) | Invalid — can't use it | Cheap (pointer transfer) | `String`, `Vec<T>` |
| **Copy** | Type is small & fixed-size (`Copy`) | Still valid | Cheap (bit duplication) | `i32`, `bool`, `char` |
| **Clone** | You call `.clone()` | Still valid | Expensive (deep copy) | Anything with `Clone` |

#### Move

```rust
let some_string = String::from("Hello, World!");
let some_other_string = some_string;
// println!("{}", some_string);   // error[E0382]: borrow of moved value
```

`String` owns heap data; moving transfers ownership. Reading the original would risk a double-free.

#### Copy

```rust
let some_int = 42;
let some_other_int = some_int;
println!("{}", some_int);   // 42 — i32 is Copy
```

#### Clone

```rust
let some_string = String::from("Hello, World!");
let some_other_string = some_string.clone();
println!("{}", some_string);   // works — independent deep copy
```

[`move_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/memory_management.rs), [`copy_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/memory_management.rs), [`clone_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/memory_management.rs)

---

## 1.2 Borrowing

### 1.2.1 Borrow Checker

A borrow (`&x`) lets you reference a value without owning it. The borrow checker enforces two rules:

| Rule | Plain-English | Error |
|---|---|---|
| **#1** | While a `&mut` is active, you can't take `&`. | `E0502` |
| **#2** | Only one `&mut` at a time. | `E0499` |

```rust
let some_string = String::from("Hello, World!");
let some_other_string = &some_string;       // read-only reference
println!("{}", some_string);                  // original still valid
```

[`immutable_borrow_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs), [`borrow_rule_one_fail_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs), [`borrow_rule_two_fail_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs)

### 1.2.2 Mutability

Variables are **immutable by default**. Opt in with `mut`.

| You wrote | Reassign? | `&mut`? |
|---|---|---|
| `let x = ...;` | No | No |
| `let mut x = ...;` | Yes | Yes |

```rust
let some_string = String::from("Hello, World!");
// some_string = String::from("Bye, World!");    // error[E0384]

let mut some_string = String::from("Hello, World!");
some_string = String::from("Bye, World!");        // works with `mut`
```

To **modify through a reference**, both must be mut/mutable, and you reach through the reference with `*`:

```rust
let mut some_string = String::from("Hello, World!");
let some_other_string = &mut some_string;
*some_other_string = String::from("Bye, World!");
```

[`immutable_variable_fail_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs), [`mutable_variable_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs), [`mutable_borrow_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs)

### 1.2.3 Scope

A borrow stays "active" until its **last use**, not until the closing `}` — this is called Non-Lexical Lifetimes (NLL). A new borrow can start as soon as the previous one is no longer used.

```rust
let mut s = String::from("Hello");
{
    let r1 = &mut s;
    *r1 = String::from("Bye");
}                       // r1 done
let r2 = &mut s;        // ok: previous borrow has ended
```

[`scope_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/borrowing.rs)

---

## 1.3 Lifetime

A **lifetime** is the answer to: *"how long is this reference valid?"* Rust ensures references never outlive what they point to.

### Dangling reference

```rust
let r;
{
    let x = 5;
    r = &x;
}                        // x dropped here
println!("{}", r);       // error[E0597]: `x` does not live long enough
```

### Lifetime elision

The compiler auto-fills lifetimes when the answer is unambiguous:

| Rule | What the compiler does |
|---|---|
| **#1** | Each input reference gets its own lifetime parameter. |
| **#2** | If there's one input lifetime, the output gets the same one. |
| **#3** | If one of the inputs is `&self`, the output gets `&self`'s lifetime. |

Works without annotation (Rule #2):

```rust
fn get_slice_of_string_until(some_string: &String, char_idx: usize) -> &str {
    &some_string[..char_idx]
}
```

### When elision is not enough

Two input references and one output — the compiler can't pick:

```rust
fn longest(x: &str, y: &str) -> &str { ... }   // error[E0106]: missing lifetime specifier
```

Fix with an explicit `'a`:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Lifetimes in structs

If a struct holds a reference, the struct needs a lifetime parameter:

```rust
struct Game<'a> {
    level: &'a i32,
}
```

By **Rule #3**, methods on `Game` returning a reference inherit `&self`'s lifetime — no manual annotation needed.

### `'static`

`'static` means "lives for the whole program." Every string literal (`"hello"`) has type `&'static str`.

| You write | Type | Lifetime | Lives where |
|---|---|---|---|
| `"hello"` | `&'static str` | `'static` | Program binary, forever |
| `"hello".to_string()` | `String` | Scope of the binding | Heap, dropped at end of scope |

So `longest` works across scopes with **literals** but fails with **owned `String`s** (which drop at the end of the inner block):

```rust
let str1 = "123456789";          // &'static str
let str2 = "123";                // &'static str
res = longest(str1, str2);       // ok — 'static
```

[`dangling_reference_fail_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs), [`lifetime_elision_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs), [`missing_lifetime_fail_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs), [`explicit_lifetime_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs), [`lifetime_in_struct_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs), [`static_lifetime_fail_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs), [`static_lifetime_example`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter1/content1/src/lifetimes.rs)

---

## Error-Code Cheat Sheet

| Code | Plain-English meaning | Bug it prevents |
|---|---|---|
| `E0382` | "You moved it; you can't read it." | Use-after-move / double-free |
| `E0384` | "You can't reassign — variable isn't `mut`." | Accidental mutation |
| `E0499` | "You can't have two `&mut` at once." | Data races |
| `E0502` | "You can't use `&` while a `&mut` is active." | Read-during-write tearing |
| `E0597` | "The thing this reference points to is already gone." | Use-after-free / dangling pointer |
| `E0106` | "I can't tell what lifetime the return value has." | Ambiguous reference lifetimes |
