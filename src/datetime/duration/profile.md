## 二つのコード間の速度計測

[![std-badge]][std] [![cat-time-badge]][cat-time]

[`time::Instant::now`]からの経過人を計測する。

[`time::Instant::elapsed`]を呼び、このサンプルが実行し終わる時間である[`time::Duration`]取得し表示する。
このメソッドは[`time::Instant`]に変換したりリセットできない。

```rust
use std::time::{Duration, Instant};
# use std::thread;
#
# fn expensive_function() {
#     thread::sleep(Duration::from_secs(1));
# }

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
```

[`time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
[`time::Instant::elapsed`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
[`time::Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`time::Instant`]:https://doc.rust-lang.org/std/time/struct.Instant.html
