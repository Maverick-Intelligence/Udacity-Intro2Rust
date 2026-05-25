# FFI — Calling C from Rust

A **Foreign Function Interface (FFI)** is a bridge that lets one language call functions written in another. Here we build a tiny C math library (`int add(int, int)`) and call it from a Rust binary. Along the way we meet raw pointers and `unsafe` Rust, which are the price of admission for crossing the language barrier.

```
c_call_in_rust/
├── c_pkg/                  ← the C side
│   ├── mathlib.h           ← header (declarations)
│   └── mathlib.c           ← implementation
└── rust_pkg/               ← the Rust side
    ├── Cargo.toml
    ├── build.rs            ← compiles the C file
    └── src/main.rs         ← declares and calls `add`
```

Run it:

```bash
cd rust_pkg
cargo run
```

```
== Chapter 4.3.2. Calling C Function ==
Calling C function add(a: i32, b: i32) with argument (21, 21) and the result is 42
```

---

## 1. Safe Rust holds your hand

Rust normally refuses things that **might** be wrong, even if they happen to be fine in your tiny program.

```rust
let mut num = 5;
let r1 = &num;        // immutable borrow
let r2 = &mut num;    // mutable borrow → compile error E0502
```

| Rule the borrow checker enforces | ELI5 |
|---|---|
| One mutable borrow, **or** any number of immutable borrows | If somebody can change the value, nobody else may even read it. |
| References must always point to live data | No dangling pointers, ever. |

This guarantees no data races, no dangling pointers, no use-after-free **at compile time** — but you don't get a way out of the rules unless you opt in.

---

## 2. Raw pointers — the same address, no safety net

You can cast a reference into a **raw pointer** with `as *const T` (read-only) or `as *mut T` (writable). Raw pointers don't participate in borrow checking.

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

println!("r1 is: {:#?}", r1);   // 0x00007ffd55e8120c
println!("r2 is: {:#?}", r2);   // 0x00007ffd55e8120c
```

| Type | What it is | Compile-time checks |
|---|---|---|
| `&T` / `&mut T` | A safe reference | Borrow checker enforces aliasing + lifetimes. |
| `*const T` / `*mut T` | A raw memory address | **None**. Could be null, dangling, or wrong type. |

Just **creating** a raw pointer is safe — printing one shows you a hex address. What's unsafe is **using** it.

---

## 3. The `unsafe` block — opt in to dangerous work

Dereferencing a raw pointer (reading the value at the address it holds) requires you to put on the safety helmet:

```rust
let deref_r1 = *r1;   // ❌ error[E0133]: dereference of raw pointer is unsafe …

let deref_r1 = unsafe { *r1 };   // ✅ ok — you've taken responsibility
```

| `unsafe { … }` means | ELI5 |
|---|---|
| "I, the programmer, promise this is sound." | The compiler stops checking; **you** must keep the invariants. |
| Not "turn off Rust" | All the type rules still apply — only the four extra abilities are unlocked. |

The four superpowers `unsafe` enables:

1. Dereference a raw pointer.
2. Call an `unsafe` function (including any `extern "C"` function).
3. Read or write a `static mut`.
4. Implement an `unsafe` trait.

See [`rust_pkg/src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/rust_pkg/src/main.rs) — function `unsafe_rust_example`.

---

## 4. The C side — [`c_pkg/mathlib.c`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/c_pkg/mathlib.c)

```c
// c_pkg/mathlib.c
#include "mathlib.h"

int add(int a, int b) {
    return a + b;
}
```

Ordinary C. The matching [`mathlib.h`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/c_pkg/mathlib.h) is where you'd normally declare `int add(int, int);` for any other C code that wants to use it.

---

## 5. Telling Rust the function exists — `extern "C"`

```rust
unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}
```

| Piece | ELI5 |
|---|---|
| `extern "C"` | "Linker, find a symbol called `add` that uses the C calling convention." |
| `unsafe` on the block (Rust 2024) | Acknowledges that **calling** anything inside crosses the language barrier — the compiler can't verify it for you. |
| `fn add(a: i32, b: i32) -> i32` | The Rust-side signature **must** match the C signature byte-for-byte. |

| Rust type | Matches C type | Width |
|---|---|---|
| `i32` | `int` (on virtually every modern platform) | 32 bits |
| `i64` | `long long` | 64 bits |
| `*const T` / `*mut T` | `const T*` / `T*` | pointer-sized |
| `c_char`, `c_int`, … (from `std::ffi`) | Platform-specific C ints, when in doubt | varies |

Then call it inside an `unsafe { … }` block:

```rust
let result = unsafe { add(21, 21) };
println!("The sum is: {}", result);    // 42
```

See [`rust_pkg/src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/rust_pkg/src/main.rs) — `main`.

---

## 6. Compiling the C file — `build.rs` + the `cc` crate

Rust needs the C file built into an object file and linked into the final binary. Cargo runs any `build.rs` script **before** compiling your crate; the `cc` crate does the heavy lifting.

```toml
# rust_pkg/Cargo.toml
[build-dependencies]
cc = "1.2.62"
```

```rust
// rust_pkg/build.rs
fn main() {
    cc::Build::new()
        .file("../c_pkg/mathlib.c")
        .include("../c_pkg")
        .compile("mathlib");
}
```

| Step | ELI5 |
|---|---|
| `[build-dependencies]` | Crates used **only** by `build.rs`, not by your runtime code. |
| `cc::Build::new()` | Start a C compilation. `cc` picks `clang`/`gcc`/MSVC for you. |
| `.file("../c_pkg/mathlib.c")` | Add a source file. |
| `.include("../c_pkg")` | Add an `-I` path so `#include "mathlib.h"` resolves. |
| `.compile("mathlib")` | Build it into a static library `libmathlib.a` and tell Cargo to link it. |

See [`rust_pkg/build.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/rust_pkg/build.rs).

---

## 7. Safety contract — your job at the border

The compiler is no longer watching. When you cross into C, **you** keep these promises:

| Promise | What can go wrong if you break it |
|---|---|
| Signatures match exactly | Wrong size / wrong calling convention → silent memory corruption. |
| Pointers are valid, aligned, and not null (unless explicitly allowed) | Segfault or UB. |
| Buffer lengths are honoured | Buffer overflow. |
| Ownership is clear (who frees memory?) | Double-free or leak. |
| Strings: only call `CString::from_raw` on a pointer you got from `CString::into_raw` | UB. If you own a `Vec<u8>` with a terminating `\0`, use `CString::from_vec_with_nul` instead. |
| Document the invariants right above the `unsafe` block | Reviewers can verify them. |

Rule of thumb: **wrap every FFI call in `unsafe { … }` and leave a `// SAFETY:` comment explaining what you guarantee.**

---

## 8. Cheat Sheet

| Concept | One-liner |
|---|---|
| FFI | Calling functions across language boundaries (Rust ↔ C here). |
| Raw pointer | `*const T` / `*mut T` — like a C pointer, no borrow-check. |
| `unsafe { … }` | "I promise these invariants; please skip the checks." |
| `extern "C" { fn … }` | Declare a function that uses the C calling convention. |
| `unsafe extern "C"` (Rust 2024) | Required spelling — acknowledges the FFI block. |
| `build.rs` | A Cargo build script. Runs before compiling the crate. |
| `cc` crate | Drives the C compiler from a build script. |
| `bindgen` | Auto-generates Rust `extern "C"` declarations from C headers (great companion crate for bigger libraries). |

| Command | Does |
|---|---|
| `cargo run` | Build C + Rust, link, run |
| `cargo build` | Build only |
| `cargo clean && cargo run` | Force a fresh C compile (after editing `mathlib.c`) |

> **Next**: the reverse direction — exposing a Rust function with `#[no_mangle] pub extern "C" fn …` so C code can call **into** Rust.
