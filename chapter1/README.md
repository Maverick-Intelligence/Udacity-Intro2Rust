# Chapter 1 — Ownership, Borrowing, and Lifetimes in Rust

---

## Part 1: Struct Definition

The `Book` struct holds a book's title (owned) and author (borrowed).

From [`src/main.rs`](file:///home/nvision/Workspace/Code/Source/Pri/Courses/Udacity/Intro2Rust/chapter1/src/main.rs):

```rust
struct Book<'author> {
    title: String,
    author: &'author str,
}
```

| Field | Type | Ownership | Lives where? |
|---|---|---|
| `title` | `String` | **Owned** — lives on the heap, freed when `Book` is dropped | Heap memory |
| `author` | `&'author str` | **Borrowed** — points to data owned elsewhere (the binary) | Program's static memory |

### Why `&'author str` and not `String`?

`author` borrows a string slice (`&str`). This is **cheaper** than `String` — no heap allocation, no copy. It just points to existing data. The `'author` part (see Part 4) is the lifetime annotation that tells Rust how long this reference is valid.

---

## Part 2: Ownership

**Ownership** means every piece of data has exactly **one owner**. When the owner drops, the data is freed.

### Owned: `String`

```rust
let title: String = String::from("Artificial Intelligence A Modern Approach");
```

- `String::from()` **allocates** the text on the heap.
- `title` is the **sole owner** of this data.
- When `title` goes out of scope, the heap memory is **automatically freed**.

### Borrowed: `&str`

```rust
let author: &str = "Stuart Russel & Peter Norvig";
```

- The string literal is stored in the **program binary** — no heap allocation.
- `author` is a **pointer** to that static data. It does not own it.
- Its lifetime is `'static` — it lives for the entire program.

### Moving `title` into `Book`

```rust
let book: Book = Book { title, author };
```

| Field | What happens |
|---|---|
| `title` | **Moved** into `book.title`. After this line, `title` is **no longer valid** — ownership transferred. |
| `author` | **Copied** (just a pointer copy, not the string data) into `book.author`. The underlying static data stays untouched. |

---

## Part 3: Borrowing

**Borrowing** lets a function read data without taking ownership.

From [`src/main.rs`](file:///home/nvision/Workspace/Code/Source/Pri/Courses/Udacity/Intro2Rust/chapter1/src/main.rs):

```rust
fn print_book_details(book: &Book) {
    println!("Book Name: {}", book.title);
    println!("Author Name: {}", book.author);
}
```

- `&Book` is an **immutable borrow** — the function can read `book` but not modify it.
- `print_book_details` **does not own** `book`. It cannot free its memory.
- The caller (`main`) keeps ownership and responsibility for cleanup.

```rust
print_book_details(&book);
```

- `&book` creates a reference (pointer) to the existing `Book`.
- The function reads `book.title` and `book.author` through this pointer.
- `book` remains valid in `main` after the call — borrowing doesn't consume it.

### Borrowing vs. Ownership Transfer

| Scenario | Syntax | Caller keeps ownership? | Can mutate? |
|---|---|---|---|
| Immutable borrow | `&Book` | ✅ Yes | ❌ No |
| Mutable borrow | `&mut Book` | ✅ Yes | ✅ Yes |
| Ownership transfer | `Book` (by value) | ❌ No | ✅ Yes (while owned) |

---

## Part 4: Lifetimes

**Lifetimes** are compile-time annotations that ensure borrowed references never outlive the data they point to.

### The Lifetime Parameter

```rust
struct Book<'author> {
    author: &'author str,
}
```

- `<'author>` declares a **generic lifetime parameter** — like a placeholder for "some duration I don't know yet."
- `&'author str` means: "this borrowed string must be valid for at least `'author`."
- The actual lifetime is filled in by the compiler at construction time.

### Why It Matters

Without `'author`, the compiler cannot verify that `author` is valid for the lifetime of `book`. The parameter ties the **borrowed reference's validity** to the struct.

### Inference in Practice

```rust
let author: &str = "Stuart Russel & Peter Norvig";  // &str = &'static str
let book: Book = Book { title, author };
```

- `author` has type `&'static str` — it lives in the binary forever.
- The compiler **infers** `'author = 'static`. No manual annotation needed.

### In Function Signatures

```rust
fn print_book_details(book: &Book) {
```

Rust applies **lifetime elision** rules. The compiler expands this to:

```rust
fn print_book_details<'a>(book: &'a Book<'a>)
```

The borrow of `book` must live at least as long as any borrowed data inside it. Since `book.author` is `'static`, this is trivially satisfied.

---

## Part 5: Main Function — All Concepts Together

[`main.rs`](file:///home/nvision/Workspace/Code/Source/Pri/Courses/Udacity/Intro2Rust/chapter1/src/main.rs) ties everything together:

```rust
fn main() {
    let title: String = String::from("Artificial Intelligence A Modern Approach");
    let author: &str = "Stuart Russel & Peter Norvig";
    let book: Book = Book { title, author };
    print_book_details(&book);
}
```

### Step-by-Step Flow

```
Line 9  ── title: String ──→ Heap data (owned by title)
              │
              ├─ moved into book.title ──→ owned by book
              │
              └─ freed when book drops

Line 10 ── author: &'static str ──→ Binary data (not owned, always valid)
              │
              ├─ copied into book.author ──→ referenced by book
              │
              └─ remains valid for the program

Line 13 ── &book ──→ Immutable borrow
              │
              └─ print_book_details reads book.title & book.author
                   book still owned by main()
```

### Compile-Time Guarantees

| Guarantee | Enforced By | Result |
|---|---|---|
| `title` is moved, not copied | Ownership rules | No double-free |
| `author` points to `'static` data | Lifetime elision | No dangling reference |
| `&book` borrow is valid during function call | Lifetime of `&Book` | No use-after-free |
| `book` outlives `&book` | Borrow checker | Reference is always valid |

### Module Organization

| Module | Path | What It Demonstrates |
|---|---|---|
| `main` | [`src/main.rs`](file:///home/nvision/Workspace/Code/Source/Pri/Courses/Udacity/Intro2Rust/chapter1/src/main.rs) | Ownership + Borrowing + Lifetimes in one file |

### Final Summary

| Concept | What It Solves | In One Line |
|---|---|---|
| **Ownership** | Who is responsible for freeing memory? | One owner per value |
| **Borrowing** | How do I read data without taking it? | `&T` = read-only reference |
| **Lifetimes** | How do I prevent dangling references? | `'a` = compile-time validity guarantee |
