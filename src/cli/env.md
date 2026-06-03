# Environment Variables
The [`env`] module lets you inspect and manipulate the process environment, including env vars, CLI
args, and working directories.

## Get Environment Variables
[`var`] fetches an env var from the current process. It returns `Result<String, VarError>`, so you
decide how to handle missing values:

```rust,edition2024,no_run
use std::env;

fn main() {
    // If not set, we use a sensible default value
    let mode = env::var("MODE").unwrap_or_else(|_| "INFO".to_string());
}
```

## Set Environment Variables

[`set_var`] is safe in single-threaded programs, and always safe on Windows. Avoid it in
multi-threaded programs on other OSes. [`See safety note`].

```rust,edition2024,no_run
use std::env;

fn main() {
    // SAFETY: No other thread is currently manipulating the environment
    unsafe {
        env::set_var("MODE", "debug");
    }
}
```

## Remove Environment Variables

Same threading rules apply as [`set_var`]. [`See safety note first`].

```rust,edition2024,no_run
use std::env;

fn main() {
    // SAFETY: No other thread is currently manipulating the environment
    unsafe {
        env::remove_var("MODE");
    }
}
```

## Loading Config from the Environment

A common pattern is loading config at startup from env vars, falling back to sensible defaults when
vars are missing or invalid. This approach keeps the program runnable without requiring every var to
be set, which is useful during development.

The chain `.ok().and_then(|v| ...)` is used here intentionally:
- `.ok()` converts Result to Option, discarding the error
- `.and_then()` applies a fallible transform, like parsing, flattening the result
- `.unwrap_or_else(|_| DEFAULT)` then supplies the fallback. [`unwrap_or_else`] only allocates
  during the evaluation of the else branch.

```rust,edition2024
use std::env;

#[derive(Debug)]
enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

// Handles conversion of string env variables to LogLevel enum.
impl TryFrom<String> for LogLevel {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        let level = match value.to_lowercase().as_str() {
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "trace" => LogLevel::Trace,
            other => return Err(format!("Unknown log level: {}", other)),
        };
        Ok(level)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
    server_url: String,
    log_level: LogLevel,
    server_port: u16,
}

const DEFAULT_SERVER_URL: &str = "http://127.0.0.1";
const DEFAULT_SERVER_PORT: u16 = 8080_u16;
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            server_url: DEFAULT_SERVER_URL.into(),
            log_level: DEFAULT_LOG_LEVEL,
            server_port: DEFAULT_SERVER_PORT,
        }
    }
}

impl Config {
    fn load() -> Self {
        let server_url =
            env::var("BACKEND_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.into());
        let server_port = env::var("BACKEND_SERVER_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(DEFAULT_SERVER_PORT);

        let log_level = env::var("BACKEND_LOG_LEVEL")
            .ok()
            .and_then(|v| v.try_into().ok())
            .unwrap_or(DEFAULT_LOG_LEVEL);

        Config {
            server_url,
            server_port,
            log_level,
        }
    }
}

fn main() {
    let cfg = Config::load();
    dbg!(cfg);
}
```

[`See safety note first`]: https://doc.rust-lang.org/std/env/fn.remove_var.html
[`See safety note`]: https://doc.rust-lang.org/std/env/fn.set_var.html
[`env`]: https://doc.rust-lang.org/std/env/index.html
[`set_var`]: https://doc.rust-lang.org/std/env/fn.set_var.html
[`unwrap_or_else`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
[`var`]: https://doc.rust-lang.org/std/env/fn.var.html
