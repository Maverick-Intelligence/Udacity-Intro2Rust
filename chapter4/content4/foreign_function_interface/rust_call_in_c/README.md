# FFI — Calling Rust from C

The opposite direction of the previous lesson. We write a tiny Rust library (`add(a, b)`), compile it as a **system shared library**, and call it from a plain C program.

```
rust_call_in_c/
├── rust_pkg/                ← the Rust side (the library)
│   ├── Cargo.toml
│   └── src/lib.rs           ← #[no_mangle] pub extern "C" fn add(...)
└── c_pkg/                   ← the C side (the consumer)
    ├── main.c               ← extern int add(int, int); int main() { ... }
    └── Makefile             ← builds and runs c_pkg, linking the Rust .so
```

Run it (Linux/macOS):

```bash
cd rust_pkg && cargo build     # produces target/debug/librust_pkg.so
cd ../c_pkg && make run        # gcc → executable → prints 3
```

---

## 1. The Rust side — [`rust_pkg/src/lib.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c/rust_pkg/src/lib.rs)

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

| Piece | ELI5 |
|---|---|
| `#[no_mangle]` (or `#[unsafe(no_mangle)]` in Rust 2024) | "Rust compiler, **don't rename** this symbol." Rust normally mangles function names so two crates can have functions with the same name; C linkers can't handle that. |
| `pub extern "C"` | "Use the **C calling convention** for this function — same way C compilers pass arguments and return values." |
| `fn add(a: i32, b: i32) -> i32` | Boring Rust — but every type **must match** the C side byte-for-byte (`i32` ↔ `int`, etc.). |

### Why the 2024 spelling `#[unsafe(no_mangle)]`?

In Rust 2024, attributes that disable safety checks at link/symbol level (like `no_mangle`, `link_section`, `export_name`) must be wrapped in `unsafe(…)`. It's the same idea as `unsafe { … }` blocks — you're acknowledging that you're stepping out of Rust's protected world. Older editions still accept the bare `#[no_mangle]`.

---

## 2. Cargo.toml — produce a system library, not a Rust library

```toml
# rust_pkg/Cargo.toml
[lib]
crate-type = ["cdylib"]
```

| Crate type | What it builds | Used for |
|---|---|---|
| `rlib` (default) | A Rust-only static lib (`.rlib`) | Other Rust crates |
| `cdylib` | A C-compatible **dynamic** library (`.so` / `.dylib` / `.dll`) | Other languages (C, Python, …) |
| `staticlib` | A C-compatible **static** archive (`.a` / `.lib`) | Embedding into another C binary |
| `proc-macro` | A compiler plugin | Procedural macros (seen earlier this chapter) |

Pick `cdylib` when you want to be `dlopen`-able / linked dynamically from C.

### What gets built per OS

```bash
cd rust_pkg && cargo build
```

| OS | File produced in `target/debug/` |
|---|---|
| Linux | `librust_pkg.so` |
| macOS | `librust_pkg.dylib` |
| Windows | `rust_pkg.dll` |

The **basename** (`rust_pkg`) is taken from `Cargo.toml`'s `[package] name`. Linkers look for `lib<name>.so` (etc.) when you pass `-l<name>`.

---

## 3. The C side — [`c_pkg/main.c`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c/c_pkg/main.c)

```c
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

extern int add(int a, int b);

int main(void) {
    int sum = add(1, 2);
    printf("%d\n", sum);          // 3
}
```

| Piece | ELI5 |
|---|---|
| `extern int add(int, int);` | "Linker, the function lives in some other object file — find it." This is the C-side mirror of Rust's `extern "C" { fn add(...) }`. |
| `add(1, 2)` | A plain C call. The linker resolves it to the symbol exported by the Rust library. |
| `int` ↔ `i32` | On every modern platform these are both 32-bit; that's why the call works without any glue. |

---

## 4. The Makefile — [`c_pkg/Makefile`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c/c_pkg/Makefile)

```makefile
.PHONY: run
run:
	gcc -o c_pkg main.c -L ../rust_pkg/target/debug -l rust_pkg
	LD_LIBRARY_PATH=../rust_pkg/target/debug ./c_pkg
```

Two steps:

| Step | Flag | Means |
|---|---|---|
| Compile + link | `-o c_pkg` | Output binary name. |
| | `-L ../rust_pkg/target/debug` | "Look in this directory for libraries." (build-time hint) |
| | `-l rust_pkg` | "Link against `librust_pkg.so`." (the `lib` prefix and `.so` suffix are added automatically) |
| Run | `LD_LIBRARY_PATH=../rust_pkg/target/debug` | "At runtime, also look in this directory for the `.so`." Without this, the executable would launch but fail to find the library. |

### Per-OS twist

| OS | What you change |
|---|---|
| **Linux** | Set `LD_LIBRARY_PATH` (as shown). |
| **macOS** | Set `DYLD_LIBRARY_PATH` instead, or `install_name_tool` the binary. |
| **Windows** | Copy `rust_pkg.dll` next to the `.exe` — Windows looks beside the binary first. |

---

## 5. Picture: what `make run` actually does

```
┌─────────────────────────┐                         ┌────────────────────────────┐
│  rust_pkg/src/lib.rs    │  cargo build            │  target/debug/             │
│  #[no_mangle]           │ ──────────────────────► │  librust_pkg.so (Linux)    │
│  pub extern "C" fn add  │                         │  librust_pkg.dylib (mac)   │
└─────────────────────────┘                         │  rust_pkg.dll (Win)        │
                                                    └─────────────┬──────────────┘
                                                                  │
                                                                  ▼
┌─────────────────────────┐  gcc -L… -l rust_pkg    ┌────────────────────────────┐
│  c_pkg/main.c           │ ──────────────────────► │  c_pkg/c_pkg (executable)  │
│  extern int add(...)    │                         │                            │
│  printf("%d", add(1,2)) │                         │  $ ./c_pkg                 │
└─────────────────────────┘                         │  3                         │
                                                    └────────────────────────────┘
```

---

## 6. FFI Resources

| Crate / Guide | What it does |
|---|---|
| [`bindgen`](https://crates.io/crates/bindgen) | Read a C header, **auto-generate** the Rust `extern "C" { … }` declarations. The companion to the previous lesson. |
| [`cbindgen`](https://crates.io/crates/cbindgen) | The **inverse** of `bindgen` — read your `#[no_mangle] pub extern "C"` Rust code and generate a `.h` file for C consumers. Perfect for this project. |
| [`libc`](https://crates.io/crates/libc) | Pre-written Rust bindings to common C types and libc functions (`malloc`, `free`, `c_int`, `c_char`, …). |
| [Rust FFI Omnibus](https://jakegoulding.com/rust-ffi-omnibus/) (Jake Goulding) | Reference cookbook with idioms for Rust↔C↔Python↔Ruby FFI. |

---

## 7. Cheat Sheet

| Concept | One-liner |
|---|---|
| Stop name mangling | `#[no_mangle]` (or `#[unsafe(no_mangle)]` in Rust 2024). |
| Use C ABI | `pub extern "C" fn …`. |
| Build a system library | `[lib] crate-type = ["cdylib"]` in `Cargo.toml`. |
| Output file | `lib<name>.so` (Linux) / `.dylib` (macOS) / `<name>.dll` (Windows). |
| Tell linker where to find it | `gcc -L <dir>` (build) **and** `-l <name>` (link). |
| Tell loader where to find it at runtime | `LD_LIBRARY_PATH` (Linux), `DYLD_LIBRARY_PATH` (macOS), or copy DLL next to .exe (Windows). |
| Auto-generate the C header | `cbindgen` crate. |

| Command | Does |
|---|---|
| `cd rust_pkg && cargo build` | Build the `.so` / `.dylib` / `.dll` |
| `cd c_pkg && make run` | gcc compiles `main.c`, links the Rust lib, runs it |
| `nm -D ../rust_pkg/target/debug/librust_pkg.so \| grep add` | Verify the `add` symbol is exported (Linux) |

> **Recap of the whole FFI chapter**: Rust ↔ C in both directions. From Rust side: `extern "C" { fn … }` + `unsafe { … }` (lesson 1). From C side: `#[no_mangle] pub extern "C" fn …` + `crate-type = "cdylib"` (this lesson). The C ABI is the lingua franca — any language that speaks it can be bridged in either direction.
