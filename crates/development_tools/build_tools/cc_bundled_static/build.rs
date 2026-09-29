fn main() {
    cc::Build::new().file("src/hello.c").compile("hello"); // outputs `libhello.a`
}
