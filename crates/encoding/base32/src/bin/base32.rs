use data_encoding::{BASE32_NOPAD, BASE32_NOPAD_VISUAL, DecodeError};

fn main() -> Result<(), DecodeError> {
    // Generate the password from random bytes.
    let initial_entropy = rand::random::<[u8; 10]>(); // 80 bits of entropy
    let shown_password = make_password(&initial_entropy);

    // Show the password to the user.
    println!("{shown_password}");

    // Read the password from the user (may have visual mistakes).
    let entered_password = alter_visually(&shown_password);
    println!("{entered_password}");

    // Reconstitute the random bytes (correcting the visual errors).
    let final_entropy = read_entropy(&entered_password)?;
    assert_eq!(final_entropy, initial_entropy);

    Ok(())
}

fn make_password(entropy: &[u8]) -> String {
    BASE32_NOPAD.encode(entropy)
}

fn read_entropy(password: &str) -> Result<Vec<u8>, DecodeError> {
    BASE32_NOPAD_VISUAL.decode(password.as_bytes())
}

fn alter_visually(input: &str) -> String {
    fn alter_char(c: char) -> char {
        match c {
            // O may be confused with 0
            'O' if rand::random::<bool>() => '0',
            // I may be confused with 1 or l
            'I' if rand::random::<bool>() => {
                if rand::random::<bool>() {
                    '1'
                } else {
                    'l'
                }
            }
            // B may be confused with 8
            'B' if rand::random::<bool>() => '8',
            _ => c,
        }
    }
    input.chars().map(alter_char).collect()
}
