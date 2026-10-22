use std::process::Command;

#[test]
fn hashes_with_the_statically_linked_library() {
    let output = Command::new(env!("CARGO_BIN_EXE_cc-external-static"))
        .arg("hello")
        .output()
        .expect("failed to run binary");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    // Known FNV-1a 64-bit test vector.
    assert!(stdout.contains("a430d84680aabd0b"), "{stdout}");
}
