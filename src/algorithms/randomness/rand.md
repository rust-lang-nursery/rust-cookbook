## 乱数の生成

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

[`rand::thread_rng`]からrandom-number generator [`rand::Rng`]を使って乱数を作ります。各スレッドは初期化されたジェネレータを持っています。整数は型の範囲内で生成され、浮動小数点数は0から1の間（ただし1は含まない）の値が生成されます。
```rust
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
```

[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand::thread_rng`]: https://docs.rs/rand/*/rand/fn.thread_rng.html
