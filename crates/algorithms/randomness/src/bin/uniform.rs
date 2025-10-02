use rand_distr::{Distribution, Uniform};

fn main() {
    let mut rng = rand::rng();
    let die =
        Uniform::new_inclusive(1, 6).expect("Failed to create uniform distribution: invalid range");

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
