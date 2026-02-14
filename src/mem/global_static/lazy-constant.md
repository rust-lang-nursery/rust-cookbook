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

[![std-badge]][std] [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns]

The [`LazyCell`] type can be used to defer the initialization of a value until the first time it is accessed.
It is useful for expensive computations that might not be needed during the entire execution of a scope.

In this example, we use [`LazyCell`] to lazily compute a set of permissions based on a user ID.
Unlike `static` lazy types, [`LazyCell`] can capture local variables from its surrounding scope.

```rust,edition2021
use std::cell::LazyCell;

fn main() {
    let user_id = 42;

    // The closure is not executed yet.
    let permissions = LazyCell::new(|| {
        println!("--- Fetching permissions from database for ID {} ---", user_id);
        // Simulate an expensive operation
        vec!["read".to_string(), "write".to_string()]
    });

    println!("User {} session started.", user_id);

    // The initialization happens only when we dereference permissions for the first time.
    if true { // Imagine a conditional check here
        println!("Permissions: {:?}", *permissions);
    }
    
    // Subsequent accesses use the already initialized value.
    println!("First permission: {}", permissions[0]);
}
```

[`LazyCell`]: https://doc.rust-lang.org/std/cell/struct.LazyCell.html

## `std::sync::LazyLock`

[![std-badge]][std] [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns]

The [`LazyLock`] type is a thread-safe version of [`LazyCell`] that can be used in `static` contexts.
It is the standard library alternative to the `lazy_static` crate for global, lazily initialized data.

The example shows how to use [`LazyLock`] to create a global configuration that is loaded once from 
the environment upon first access.

```rust,edition2024
use std::sync::LazyLock;

struct Config {
    api_key: String,
    timeout: u64,
}

// Global configuration initialized on first use.
static APP_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    println!("Loading configuration...");
    Config {
        api_key: std::env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string()),
        timeout: 30,
    }
});

fn main() {
    println!("App started.");

    // The initialization happens here.
    let timeout = APP_CONFIG.timeout;

    println!("Timeout is: {}s", timeout);
    println!("API Key is hidden: {}", APP_CONFIG.api_key.len() > 0);
}
```

[`LazyLock`]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
