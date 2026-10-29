use mdbook_chips::chips;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = chips::run_preprocess(&args) {
        eprintln!("mdbook-chips: {e}");
        std::process::exit(1);
    }
}
