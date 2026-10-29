use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    println!("a die roll: {}", rng.random_range(1..=6));
    println!("a float:    {}", rng.random::<f64>());
    println!("a coin:     {}", rng.random_bool(0.5));
}
