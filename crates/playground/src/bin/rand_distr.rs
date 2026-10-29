use rand_distr::{Distribution, Normal};

fn main() -> Result<(), rand_distr::NormalError> {
    let mut rng = rand::rng();
    let normal = Normal::new(2.0, 3.0)?;
    for _ in 0..3 {
        println!("{:.3}", normal.sample(&mut rng));
    }
    Ok(())
}
