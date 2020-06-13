## Read Environment Variable 

[![std-badge]][std] [![cat-os-badge]][cat-os]

Reads an environment variable via [std::env::var].

```rust,edition2018,no_run
use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // read `config_path` from the environment variable `CONFIG`.
    // If `CONFIG` isn't set, fall back to a default config path.
    let config_path = env::var("CONFIG")
        .unwrap_or("/etc/myapp/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
```

[std::env::var]: https://doc.rust-lang.org/std/env/fn.var.html
