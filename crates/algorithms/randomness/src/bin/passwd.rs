use rand::{distr::Alphanumeric, RngExt};

fn main() {
    let password = generate_password();
    println!("{password}");
}

fn generate_password() -> String {
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::rng();

    (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let password = generate_password();
        assert_eq!(password.len(), 30);
    }

    #[test]
    fn test_password_is_alphanumeric() {
        let password = generate_password();
        assert!(password.chars().all(|c| c.is_alphanumeric()));
    }
}
