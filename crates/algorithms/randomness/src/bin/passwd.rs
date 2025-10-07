use rand::Rng;
use rand_distr::Alphanumeric;

fn main() {
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    println!("{password}");
}
