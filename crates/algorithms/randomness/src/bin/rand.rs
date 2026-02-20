use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();
    println!("Random number: {random_number}");
}
