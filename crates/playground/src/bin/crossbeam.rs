fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let (left, right) = numbers.split_at(3);

    // Scoped threads may borrow from the stack.
    let (a, b) = crossbeam::thread::scope(|s| {
        let a = s.spawn(|_| left.iter().sum::<i32>());
        let b = s.spawn(|_| right.iter().sum::<i32>());
        (a.join().unwrap(), b.join().unwrap())
    })
    .unwrap();
    println!("{a} + {b} = {}", a + b);
}
