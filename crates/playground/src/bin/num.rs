use num::BigUint;

fn main() {
    let two = BigUint::from(2u32);
    println!("2^100 = {}", num::pow(two, 100));

    let c = num::Complex::new(1.0, 2.0) * num::Complex::new(3.0, -1.0);
    println!("(1+2i)(3-i) = {c}");
}
