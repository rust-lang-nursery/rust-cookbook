use std::error::Error;

unsafe extern "C" {
    fn fnv1a_64(data: *const u8, len: usize) -> u64;
}

/// Safe wrapper around the unsafe `fnv1a_64` C function.
fn fnv1a(data: &[u8]) -> u64 {
    unsafe { fnv1a_64(data.as_ptr(), data.len()) }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "hello".to_string());
    println!("fnv1a-64({input:?}) = {:016x}", fnv1a(input.as_bytes()));
    Ok(())
}
