// build.rs
fn main() {
    cc::Build::new()
        .file("src/double.c")
        .compile("libdouble.a");
}