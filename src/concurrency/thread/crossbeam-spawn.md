## 短命のスレッドの生成

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

この例では並列プログラミングのためのデータ型と関数を提供する[crossbeam]クレートを使います。[`Scope::spawn`]は[`crossbeam::scope`]関数を通ったクロージャが終了する前にリターンすることが保証されたスレッドを生成します。つまり、呼ばれた関数からデータを参照できることを意味します。


```rust
extern crate crossbeam;

fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
  
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
  
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
  
        let min_l = thread_l.join().unwrap()?;
        let min_r = thread_r.join().unwrap()?;
  
        Some(min_l.max(min_r))
    }).unwrap()
}
```

[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn
