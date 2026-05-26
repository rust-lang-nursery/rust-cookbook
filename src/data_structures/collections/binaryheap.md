## Process Tasks in Priority Order

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

Uses [`BinaryHeap`] as a max-priority queue: [`push`] inserts an item and [`pop`] always removes the highest-priority one first. Wrapping items in [`Reverse`] turns it into a min-priority queue without a custom comparator.

```rust,edition2021
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // Max-heap: highest priority number is processed first
    let mut queue: BinaryHeap<(u32, &str)> = BinaryHeap::new();
    queue.push((3, "send report"));
    queue.push((1, "check email"));
    queue.push((5, "fix production bug"));
    queue.push((2, "review PR"));

    println!("Max-heap — processing order:");
    while let Some((priority, task)) = queue.pop() {
        println!("  [{priority}] {task}");
    }

    // Min-heap: wrap with Reverse so lowest number is processed first
    let mut min_queue: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    min_queue.push(Reverse((3, "send report")));
    min_queue.push(Reverse((1, "check email")));
    min_queue.push(Reverse((5, "fix production bug")));

    println!("Min-heap — processing order:");
    while let Some(Reverse((priority, task))) = min_queue.pop() {
        println!("  [{priority}] {task}");
    }
}
```

[`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
[`push`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.push
[`pop`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.pop
[`Reverse`]: https://doc.rust-lang.org/std/cmp/struct.Reverse.html
