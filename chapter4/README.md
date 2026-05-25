# Chapter 4 — Macros and Foreign Function Interface

Two flavours of macros (the `macro_rules!` kind and the procedural kind) plus calling into other languages through the C ABI.

| What | Where |
|---|---|
| **Content — Declarative Macros** — `macro_rules!`, fragment specifiers, repetition, `stringify!`, `#[macro_export]` + `$crate`, scoping, match-all / callbacks / TT munchers | [`content4/declarative_macros/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/declarative_macros/README.md) |
| **Content — Procedural Macros** — `#[proc_macro]`, `#[proc_macro_attribute]`, Cargo workspaces, `syn` + `quote` | [`content4/procedural_macros/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/procedural_macros/README.md) |
| **Content — Foreign Function Interface** — what FFI is, both directions | [`content4/foreign_function_interface/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/README.md) |
| └ Calling C from Rust — `extern "C" { … }`, `unsafe`, `build.rs`, the `cc` crate | [`c_call_in_rust/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/c_call_in_rust/README.md) |
| └ Calling Rust from C — `#[no_mangle]`, `cdylib`, linker flags | [`rust_call_in_c/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/content4/foreign_function_interface/rust_call_in_c/README.md) |
| **Exercise — `#[validate]` Attribute Macro** — proc-macro workspace + consumer | [`exercise4/procedural_macros/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/procedural_macros/README.md) |
| **Exercise — Reverse a String via FFI** — Rust calls a C `reverse_string` through a `*mut c_char` | [`exercise4/ffi_c_in_rust/README.md`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/ffi_c_in_rust/README.md) |

## Numbering used in the runnable examples

| Section | Topic | Code-output prefix |
|---|---|---|
| 4.1 | Declarative Macros | `=== 4.1.x …` |
| 4.2 | Procedural Macros | `== 4.2.x …` |
| 4.3 | Foreign Function Interface | `== Chapter 4.3.x …` |

Run any example:

```bash
cd content4/declarative_macros && cargo run
cd content4/procedural_macros  && cargo run -p proc_macros_bin
cd content4/foreign_function_interface/c_call_in_rust/rust_pkg && cargo run
cd content4/foreign_function_interface/rust_call_in_c/c_pkg     && make run
```

Run the exercises:

```bash
cd exercise4/procedural_macros && cargo run -p field_validator
cd exercise4/ffi_c_in_rust/rust_reverse_str && cargo run
```
