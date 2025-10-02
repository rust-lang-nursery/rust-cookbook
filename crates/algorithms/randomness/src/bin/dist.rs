use rand_distr::{Distribution, LogNormal, Normal};

fn main() {
    let mut rng = rand::rng();
    let normal = Normal::new(2.0, 3.0).expect("Failed to create normal distribution");
    let log_normal = LogNormal::new(1.0, 0.5).expect("Failed to create log-normal distribution");

    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    let v = log_normal.sample(&mut rng);
    println!("{} is from an ln N(1, 0.25) distribution", v);
}
