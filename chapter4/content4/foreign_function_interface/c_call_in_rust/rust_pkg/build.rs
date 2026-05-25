fn main() {
    cc::Build::new()
        .file("../c_pkg/mathlib.c")
        .include("../c_pkg")
        .compile("mathlib");
}
