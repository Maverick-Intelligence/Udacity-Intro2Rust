# Exercise — Reverse a String via FFI

A C function reverses a string **in place**, and Rust calls it. Same FFI plumbing as the lesson, with two extras: the string is mutated through a `*mut c_char` pointer, and we have to hand the buffer to C **and** take it back.

```
ffi_c_in_rust/
├── c_reverse_str/
│   └── src/main.c              ← char *reverse_string(char *s, int len)
└── rust_reverse_str/
    ├── Cargo.toml
    ├── build.rs                ← compiles the C file
    └── src/main.rs             ← turns a String into a C buffer, calls C, reads it back
```

Run it:

```bash
cd rust_reverse_str
cargo run
```

```
Reversed Word: 4 retpahC ni elpmaxE IFF tsuR
```

---

## 1. The C side — [`c_reverse_str/src/main.c`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/ffi_c_in_rust/c_reverse_str/src/main.c)

```c
char *reverse_string(char *s, int len) {
    int i, j;
    char temp;
    for (i = 0, j = len - 1; i < j; i++, j--) {
        temp = s[i];
        s[i]  = s[j];
        s[j]  = temp;
    }
    return s;
}
```

| Piece | ELI5 |
|---|---|
| `char *s` | A pointer to the **start** of a character buffer. C strings have no built-in length, that's why… |
| `int len` | …you also pass the length explicitly. |
| Two-pointer swap loop | Walk `i` forward and `j` backward, swap as you go. |
| Mutates `s` in place | The buffer the caller (Rust) owns is rewritten. Nothing is allocated here. |

---

## 2. Compiling the C file — [`build.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/ffi_c_in_rust/rust_reverse_str/build.rs)

```rust
fn main() {
    cc::Build::new()
        .file("../c_reverse_str/src/main.c")
        .include("../c_reverse_str/src")
        .compile("c_reverse_str");
}
```

Same shape as the lesson: the `cc` crate compiles `main.c` into a static library and links it into our Rust binary at build time. The `[build-dependencies] cc = "1.2.62"` in `Cargo.toml` makes the crate available **only** during the build.

---

## 3. The Rust side — [`rust_reverse_str/src/main.rs`](https://github.com/Maverick-Intelligence/Udacity-Intro2Rust/blob/trunk/chapter4/exercise4/ffi_c_in_rust/rust_reverse_str/src/main.rs)

### Declare the foreign function

```rust
use std::os::raw::c_char;

unsafe extern "C" {
    fn reverse_string(s: *mut c_char, l: usize) -> c_char;
}
```

| Piece | ELI5 |
|---|---|
| `*mut c_char` | A writable C-style pointer to bytes. The C function will **mutate** through it, hence `*mut`, not `*const`. |
| `c_char` | Platform-specific alias for "what C calls `char`" — usually `i8`. Always use `c_char` over `i8` so it stays correct on weird platforms. |
| `usize` for length | Maps to C's `size_t`-ish; the source uses `int`, but `usize` is fine on every common platform for our sizes. |
| `unsafe extern "C"` (Rust 2024) | Required wording — acknowledges that calling anything inside crosses the language barrier. |

### Convert a Rust `String` to a C-friendly buffer

```rust
let rust_string = String::from("Rust FFI Example in Chapter 4");
let c_string = CString::new(rust_string).expect("CString::new failed!");

let length = c_string.as_bytes().len();
let c_ptr  = c_string.into_raw();      // hand ownership of the buffer to C-land
```

| Step | ELI5 |
|---|---|
| `CString::new(rust_string)` | Build a **null-terminated** buffer from the Rust `String`. Fails if the input already contained an interior `\0`. |
| `c_string.as_bytes().len()` | Get the length **without** the trailing `\0`. That's what `reverse_string` expects so the `\0` stays at the end. |
| `c_string.into_raw()` | Give up ownership and get a `*mut c_char` to the buffer. From this moment, **you** are responsible for freeing it. |

### Call the C function

```rust
unsafe { reverse_string(c_ptr, length) };
```

The buffer is mutated in place — `c_ptr` still points at it, just with bytes reversed.

### Take ownership back and read the result

```rust
let reversed_word = unsafe {
    CString::from_raw(c_ptr)
        .into_string()
        .expect("CString::from_raw failed!")
};
println!("Reversed Word: {}", reversed_word);
```

| Step | ELI5 |
|---|---|
| `CString::from_raw(c_ptr)` | **Reclaim** ownership of the buffer. From now on Rust will drop it normally. ⚠️ Only legal because the pointer was originally produced by `CString::into_raw`. |
| `.into_string()` | Validate UTF-8 and produce a `String`. Returns an error if the buffer isn't valid UTF-8 — reversing ASCII is always safe; reversing multibyte UTF-8 isn't (you'd split a codepoint). |

---

## 4. The hand-off pattern in one picture

```
  Rust String                                C-mutable buffer
  ─────────────                              ────────────────
  "Rust FFI Example in Chapter 4"
        │
        ▼ CString::new                       (heap-allocated, null-terminated)
  CString  ──── .into_raw() ─────────► *mut c_char ───┐
                                                       │
                                  unsafe extern "C"    │
                                  reverse_string()  ◄──┘
                                  (mutates in place)
                                                       │
        ┌─── CString::from_raw() ◄─────── *mut c_char ─┘
        ▼
  CString  ──── .into_string() ──────────► String
                                            "4 retpahC ni elpmaxE IFF tsuR"
```

The crucial property: **the same pointer** flows out of `into_raw` and back into `from_raw`. Breaking that pairing is undefined behaviour.

---

## 5. Safety contract — what you promised by writing `unsafe`

| Promise | What it protects |
|---|---|
| `c_ptr` came from `CString::into_raw` | Required for `CString::from_raw` to be sound. |
| `length` is exactly the byte length (without `\0`) | Prevents C from swapping past the end or skipping the last byte. |
| The buffer isn't freed by C | Rust still owns the allocation — C only borrows it for the swap. |
| Final bytes are valid UTF-8 | Required for `into_string()`. Reversing ASCII keeps it valid; arbitrary text would need different handling. |

If any of these break, the program enters **undefined behaviour** — anything from a wrong answer to a segfault.

---

## 6. Cheat Sheet

| Task | Snippet |
|---|---|
| Declare a foreign C function in Rust 2024 | `unsafe extern "C" { fn name(args) -> ret; }` |
| Convert `String` → C buffer | `CString::new(s)?.into_raw()` (and remember the length!) |
| Convert C buffer → `String` | `unsafe { CString::from_raw(ptr).into_string()? }` |
| Use C's `char` type portably | `std::os::raw::c_char` |
| Compile bundled C source | `cc::Build::new().file("foo.c").compile("foo");` in `build.rs` |

| Command | Does |
|---|---|
| `cargo run` | Compile C + Rust, link, run |
| `cargo clean && cargo run` | Force a fresh C compile after editing `main.c` |
