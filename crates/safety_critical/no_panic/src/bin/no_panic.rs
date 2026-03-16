use no_panic::no_panic;

/// Sums a slice without any operation that could panic.
///
/// In safety-critical code, a `panic!` is a catastrophic failure.
/// Standard indexing like `slice[i]` inserts a bounds check that
/// calls `panic!` on out-of-bounds access.  Iterators avoid this
/// entirely—the compiler can prove no panic path exists.
#[no_panic]
fn safe_sum(values: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for &v in values {
        total += v as i64;
    }
    total
}

/// Looks up a value by index, returning `None` instead of panicking.
///
/// Using `get()` + exhaustive pattern matching guarantees every code
/// path is handled.  The `#[no_panic]` attribute makes the compiler
/// **prove** it at link time—if any hidden panic path remains, the
/// build fails.
#[no_panic]
fn safe_lookup(data: &[u8], index: usize) -> Option<u8> {
    match data.get(index) {
        Some(&val) => Some(val),
        None => None,
    }
}

/// Clamps a sensor reading into a valid range without panicking.
///
/// Real-world example: an ADC returns a raw `u16` that must be
/// mapped to 0–100 %.  Using `clamp` and simple arithmetic keeps
/// the function panic-free.
#[no_panic]
fn normalize_sensor(raw: u16, min: u16, max: u16) -> f32 {
    if max == min {
        return 0.0;
    }
    let clamped = raw.clamp(min, max);
    (clamped - min) as f32 / (max - min) as f32
}

fn main() {
    let readings = [10, 20, 30, 40, 50];

    let total = safe_sum(&readings);
    println!("sum = {total}");
    assert_eq!(total, 150);

    let val = safe_lookup(&[0xAA, 0xBB, 0xCC], 1);
    println!("lookup index 1 = {val:?}");
    assert_eq!(val, Some(0xBB));

    let miss = safe_lookup(&[0xAA, 0xBB, 0xCC], 99);
    println!("lookup index 99 = {miss:?}");
    assert_eq!(miss, None);

    let pct = normalize_sensor(2048, 0, 4095);
    println!("sensor = {pct:.2}%");
    assert!((pct - 0.5).abs() < 0.01);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_sum_empty() {
        assert_eq!(safe_sum(&[]), 0);
    }

    #[test]
    fn test_safe_sum_values() {
        assert_eq!(safe_sum(&[1, 2, 3]), 6);
    }

    #[test]
    fn test_safe_lookup_in_bounds() {
        assert_eq!(safe_lookup(&[10, 20, 30], 2), Some(30));
    }

    #[test]
    fn test_safe_lookup_out_of_bounds() {
        assert_eq!(safe_lookup(&[10, 20, 30], 5), None);
    }

    #[test]
    fn test_normalize_sensor_mid() {
        let pct = normalize_sensor(2048, 0, 4095);
        assert!((pct - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_normalize_sensor_equal_bounds() {
        assert_eq!(normalize_sensor(100, 100, 100), 0.0);
    }
}
