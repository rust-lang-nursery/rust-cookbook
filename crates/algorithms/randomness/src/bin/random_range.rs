use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    println!("Integer: {}", rng.random_range(0..10));
    println!("Float: {}", rng.random_range(0.0..10.0));
}
