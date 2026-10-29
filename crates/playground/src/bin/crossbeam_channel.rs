use std::thread;

use crossbeam_channel::unbounded;

fn main() {
    let (sender, receiver) = unbounded();

    let producer = thread::spawn(move || {
        for n in 1..=5 {
            if sender.send(n * n).is_err() {
                break;
            }
        }
    });

    // The loop ends when every sender is dropped.
    for square in receiver {
        println!("got {square}");
    }
    if producer.join().is_err() {
        println!("producer panicked");
    }
}
