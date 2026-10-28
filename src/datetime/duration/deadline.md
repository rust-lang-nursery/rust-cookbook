## Deadline arithmetic with `Instant`

[![std-badge]][std] [![cat-time-badge]][cat-time]

Calculates a one second [`Duration`]. Computes `deadline` one second from [`Instant::now`].
Sleeps for one second and compares `deadline` with [`Instant::now`] to check if the deadline has passed.

```rust,edition2024
# use std::thread;
use std::time::{Duration, Instant};
# fn sleep(duration: Duration) {
#    thread::sleep(duration);
# }

fn main() {
    let one_second = Duration::from_secs(1);
    let deadline = Instant::now() + one_second;

    sleep(Duration::from_secs(1));
    if Instant::now() > deadline {
        println!("Deadline has passed");
    }
}
```
[`Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
[`Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
