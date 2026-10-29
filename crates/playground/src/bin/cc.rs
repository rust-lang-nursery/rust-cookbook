// `cc` compiles C and C++ from a build script (build.rs), which the playground cannot
// run. This shows the builder you would put there; copy it into your own build.rs.
fn main() {
    let mut build = cc::Build::new();
    build
        .file("src/hello.c")
        .define("GREETING", "\"hi\"")
        .opt_level(2);
    // build.compile("hello"); // in build.rs: emits the cargo directives to link libhello.a
    println!("configured a cc::Build, see the comment above");
}
