fn main() {
    cc::Build::new()
        .file("../c_reverse_str/src/main.c")
        .include("../c_reverse_str/src")
        .compile("c_reverse_str");
}
