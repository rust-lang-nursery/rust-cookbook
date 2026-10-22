use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // The external library keeps its own layout: public headers in `include/`
    // and implementation files in `src/`.
    let lib_dir = "vendor/fnv";

    cc::Build::new()
        .include(format!("{lib_dir}/include"))
        .file(format!("{lib_dir}/src/fnv.c"))
        // Third-party code is not ours to fix, so don't fail on its warnings.
        .warnings(false)
        .try_compile("fnv")?; // outputs `libfnv.a` and emits `cargo:rustc-link-lib=static=fnv`

    println!("cargo:rerun-if-changed={lib_dir}");
    Ok(())
}
