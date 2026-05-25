# Foreign Function Interface (FFI)

A **Foreign Function Interface (FFI)** is a bridge that lets one language call functions written in another. With Rust ↔ C this is especially common because the **C ABI** (the way C lays out function calls in memory) is the lingua franca that virtually every language can speak.

```
foreign_function_interface/
├── c_call_in_rust/         ← C library called from Rust
└── rust_call_in_c/         ← Rust library called from C
```

Two complete worked examples — one in each direction — both implementing the same trivial `add(a, b)` so you can focus on the **plumbing**, not the math.

---

## 1. What problem does FFI solve?

| Scenario | What you'd do without FFI | What FFI lets you do |
|---|---|---|
| You need a mature C library (OpenSSL, SQLite, FFmpeg) | Rewrite it in Rust 😱 | Call it directly from Rust. |
| You want to expose your Rust code to C / Python / Node / Ruby | Stuck — they can't read Rust crates. | Compile a C-compatible library and call it from anywhere. |
| Two languages must share the same data structures | Manual JSON / IPC | Pass pointers directly (with care). |

The C ABI is **stable across decades** and **stable across compilers**. It's the universal handshake.

---

## 2. The two sub-projects

### 2.a [`c_call_in_rust/`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust) — C from Rust

We build a C library (`int add(int, int)`) and call it from a Rust binary.

| You'll learn | Where |
|---|---|
| `unsafe` Rust, raw pointers (`*const T` / `*mut T`) | `rust_pkg/src/main.rs` |
| `extern "C" { fn … }` to declare a C symbol | same |
| `build.rs` + the `cc` crate to compile the C source | `rust_pkg/build.rs` |
| Safety contracts when crossing the language barrier | the full lesson |

→ Full walk-through: [`c_call_in_rust/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/README.md)

### 2.b [`rust_call_in_c/`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c) — Rust from C

We build a Rust library (`pub extern "C" fn add(...)`) as a system shared library and call it from a C program.

| You'll learn | Where |
|---|---|
| `#[no_mangle]` / `#[unsafe(no_mangle)]` (Rust 2024) | `rust_pkg/src/lib.rs` |
| `pub extern "C"` to use the C calling convention | same |
| `[lib] crate-type = ["cdylib"]` to produce `.so` / `.dylib` / `.dll` | `rust_pkg/Cargo.toml` |
| Linking from C with `gcc -L … -l … ` + `LD_LIBRARY_PATH` | `c_pkg/Makefile` |

→ Full walk-through: [`rust_call_in_c/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c/README.md)

---

## 3. Vocabulary you'll meet in both lessons

| Term | ELI5 |
|---|---|
| **ABI** (Application Binary Interface) | The contract for **how** functions pass arguments and return values at the machine level. The "C ABI" is the most widespread one. |
| `extern "C"` | "Use the C ABI for this function." Works on both sides — Rust uses it to **declare** or **export** functions. |
| **Name mangling** | The Rust compiler usually rewrites function names so two crates can share a name. C linkers can't deal with mangled names, so we turn it off with `#[no_mangle]`. |
| `unsafe` block | "I, the programmer, promise the invariants hold." Required for every FFI call because the compiler can't see across the language boundary. |
| **Raw pointer** (`*const T` / `*mut T`) | A C-style pointer — no borrow checking, no lifetimes. Creating one is safe; **dereferencing** one needs `unsafe`. |
| **`cdylib`** | The Cargo crate type that produces a **C-compatible** dynamic library. |
| **`build.rs`** | A Rust file that runs **before** your crate compiles — used to compile C sources, generate code, etc. |

---

## 4. Direction comparison

| | C → Rust calls C | Rust → C calls Rust |
|---|---|---|
| Project | `c_call_in_rust/` | `rust_call_in_c/` |
| Rust attribute | `extern "C" { fn add(...) }` (Rust **declares** the foreign fn) | `#[no_mangle] pub extern "C" fn add(...)` (Rust **exports** the fn) |
| Build helper | `build.rs` + `cc` crate (Rust compiles the C file) | `cargo build` (Rust produces a `.so`/`.dylib`/`.dll`); C is built separately by `gcc`/`make` |
| Crate type | default (binary `[[bin]]`) | `cdylib` |
| Glue file | `build.rs` | `Makefile` |
| Run with | `cargo run` | `make run` |

Same C ABI on both sides — only the direction of the function call (and therefore who exports vs. imports) changes.

---

## 5. Companion crates and resources

Tools that take the manual work out of FFI:

| Crate / Guide | What it does | Use when |
|---|---|---|
| [`bindgen`](https://crates.io/crates/bindgen) | Reads a C header (`.h`) and **auto-generates** the Rust `extern "C" { … }` declarations. | Calling **large** C libraries from Rust — you don't want to type 500 declarations by hand. |
| [`cbindgen`](https://crates.io/crates/cbindgen) | The **inverse**: reads your `#[no_mangle] pub extern "C"` Rust and emits a `.h` file. | Distributing a Rust library to C consumers. |
| [`cc`](https://crates.io/crates/cc) | Drives the C compiler from `build.rs`. Used in `c_call_in_rust/`. | Compiling C sources alongside Rust. |
| [`libc`](https://crates.io/crates/libc) | Pre-written Rust bindings to common C types (`c_int`, `c_char`) and libc functions (`malloc`, `free`). | Anywhere you'd type out C primitive types by hand. |
| [Rust FFI Omnibus](https://jakegoulding.com/rust-ffi-omnibus/) by Jake Goulding | Reference cookbook with idioms for Rust ↔ C / Python / Ruby / Node. | Looking up "how do I pass a struct?", "how do I return a string?", etc. |
| [The Rustonomicon — "FFI"](https://doc.rust-lang.org/nomicon/ffi.html) | The deep-dive on FFI safety. | When you're ready to read about variadic functions, callbacks, panic-across-FFI, etc. |

---

## 6. Cheat Sheet

| Task | Snippet |
|---|---|
| Declare a foreign C function in Rust | `unsafe extern "C" { fn add(a: i32, b: i32) -> i32; }` |
| Call it | `let n = unsafe { add(1, 2) };` |
| Export a Rust function for C | `#[unsafe(no_mangle)] pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }` |
| Build a dynamic system library | `[lib] crate-type = ["cdylib"]` in `Cargo.toml` |
| Compile C from a Rust build | `cc::Build::new().file("foo.c").compile("foo");` in `build.rs` |
| Link a Rust dynamic lib from C | `gcc -L path/to/target/debug -l rust_pkg main.c` |
| Run that binary | `LD_LIBRARY_PATH=path/to/target/debug ./a.out` (Linux) |

> Pick a direction below and dive in:
> - [`c_call_in_rust/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/README.md) — start here if you have an existing C library.
> - [`rust_call_in_c/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c/README.md) — start here if you want C / Python / Node to use your Rust code.
