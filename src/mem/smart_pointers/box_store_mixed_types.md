## Store different types in one vector with `Box`
[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

Defines the trait `Notification` that requires the implementation of the function `send(&self)`. Implement `Notification` trait for `Email` and `Sms`.
Creates a vector that holds different types that implement the same trait `Notification` and [`Box<T>`]. Iterate over that vector calling the method `send(&self)`.

Trait objects such as `dyn Notification` do not have a size known at compile time, so they cannot be stored directly by value. [`Box<T>`] stores the value behind
a pointer with a known size, making it possible to use different `Notification` implementations.

```rust,edition2021
trait Notification {
    fn send(&self);
}

struct Email {
    address: String,
}

struct Sms {
    phone_number: String,
}

impl Notification for Email {
    fn send(&self) {
        println!("Sending email notification to {}", self.address);
    }
}

impl Notification for Sms {
    fn send(&self) {
        println!("Sending sms notification to {}", self.phone_number);
    }
}

fn main() {
    let email = Email { address: "example@mail.com".to_string() };
    let sms = Sms { phone_number: "1-800-555-0100".to_string() };

    let notifications: Vec<Box<dyn Notification>> = vec![Box::new(email), Box::new(sms)];

    for notification in notifications {
        notification.send();
    }
}
```

[`Box<T>`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
