## Declare lazily evaluated constant

[![lazy_static-badge]][lazy_static] [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns]

Declares a lazily evaluated constant [`HashMap`]. The [`HashMap`] will
be evaluated once and stored behind a global static reference.

```rust,edition2018
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}

fn main() {
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);

    show_access("Jim");
}
```

[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html

## Std:cell

[`OnceCell`] is included in the standard library as an alternative.

```rust,edition2021
use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| {
        "Hello, World!".to_string()
    });
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());
}
```

[`OnceCell`]: https://doc.rust-lang.org/beta/std/cell/struct.OnceCell.html

## `std::cell::LazyCell`
[`LazyCell`] and its thread-safe counterpart [`LazyLock`] can be used to create a value which is initialized on the first access.
```rust,edition2021
use std::cell::LazyCell;

struct User {
    id: u32,
    username: String,
    // We defer the expensive permission calculation
    permissions: LazyCell<Vec<String>>,
}

impl User {
    fn new(id: u32, username: String) -> Self {
        Self {
            id,
            username,
            permissions: LazyCell::new(|| {
                println!("--- Fetching permissions from database for ID {} ---", id);
                // Simulate a heavy operation
                vec!["read".to_string(), "write".to_string()]
            }),
        }
    }
}

fn main() {
    let user = User::new(1, "ferris_rustacean".into());

    println!("User {} loaded.", user.username);

    // If we never access user.permissions, the closure is never run.

    if true { // Imagine a conditional check here
        println!("Permissions: {:?}", *user.permissions);
    }
```

[`LazyCell`]: https://doc.rust-lang.org/std/cell/struct.LazyCell.html

## `std::sync::LazyLock`

The [`LazyLock`] type is a thread-safe alternative to [`LazyCell`].
```rust,edition2024
use std::sync::LazyLock;
use std::collections::HashMap;

struct Config {
    api_key: String,
    timeout: u64,
}

// Imagine loading this from a .env file or a vault
static APP_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    println!("Loading configuration...");
    Config {
        api_key: std::env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string()),
        timeout: 30,
    }
});

fn main() {
    println!("App started.");

    // The closure above isn't run until we access APP_CONFIG here.
    let timeout = APP_CONFIG.timeout;

    println!("Timeout is: {}s", timeout);
    println!("API Key is hidden: {}", APP_CONFIG.api_key.len() > 0);
}
```
[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
