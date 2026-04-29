# Structured Concurrency
When your program runs multiple tasks at the same time, you need a way to keep track of them, 
know when they finish, collect their results, and clean up if something goes wrong. 
This section shows how to do that.

## Join Sets

[![tokio-badge]][tokio]

A [`JoinSet`] is a container that holds a group of non-blocking tasks running at the same time.
You add tasks to it, and then you can wait for them to finish one by one. When a task finishes,
you can grab its result. If you throw away the JoinSet before all tasks are done, every
unfinished task is cancelled automatically.

In this example, five jobs are created and added to a JoinSet. Each job does some work on its own,
and the program waits until all of them are done before moving on.

```rust,edition2018
use tokio::task::JoinSet;

struct Job {
    id: usize,
    data: String,
}

impl Job {
    fn new(id: usize) -> Self {
        let data = "d".repeat(id);
        Self { id, data }
    }

    async fn process(&mut self) {
        // do some work, like fetching more data over the network
        self.data = self.data.replace("d", "x");
        println!("Id {} -> {}", self.id, self.data);
    }
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    const JOB_COUNT: usize = 5;
    let jobs = (0..JOB_COUNT).map(Job::new).collect::<Vec<Job>>();

    for mut job in jobs {
        set.spawn(async move { job.process().await });
    }

    let mut complete = 0usize;
    while set.join_next().await.is_some() { // wait for each job to finish
        complete += 1;
    }

    assert_eq!(complete, JOB_COUNT);
}
```

> Add `tokio` to `Cargo.toml` with the [`macros`] and [`rt`] features enabled.
> ```toml
> [dependencies]
> tokio = { version = "*", features = ["macros", "rt"] }
> ```

{{#include ../links.md}}

[`JoinSet`]: https://docs.rs/tokio/*/tokio/task/struct.JoinSet.html
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt`]: https://docs.rs/crate/tokio/*/features#rt
