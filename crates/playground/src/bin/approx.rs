use approx::{assert_relative_eq, relative_eq};

fn main() {
    let a = 0.1 + 0.2;
    println!("0.1 + 0.2 == 0.3: {}", a == 0.3);
    println!("relative_eq:      {}", relative_eq!(a, 0.3));

    assert_relative_eq!(1.0e10, 1.0e10 + 1.0, max_relative = 1e-9);
    println!("close enough");
}
