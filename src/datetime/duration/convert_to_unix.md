## Convert `SystemTime` to Unix timestamp

[![std-badge]][std] [![cat-time-badge]][cat-time]

Converts the current [`SystemTime`] to seconds since the Unix epoch, then computes the original time
by adding the unix timestamp to [`SystemTime::UNIX_EPOCH`] constant.

```rust,edition2024
use std::time::{SystemTime, SystemTimeError};

fn main() -> Result<(), SystemTimeError> {
    let now = SystemTime::now();
    let since_unix_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;
    println!("Unix timestamp: {}", since_unix_epoch.as_secs());

    let from_unix_timestamp = SystemTime::UNIX_EPOCH + since_unix_epoch;
    println!("Back to SystemTime: {:?}", from_unix_timestamp);

    Ok(())
}
```
[`SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`SystemTime::UNIX_EPOCH`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#associatedconstant.UNIX_EPOCH
