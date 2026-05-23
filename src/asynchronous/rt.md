# Async Runtime

When your program asks for something that takes time, e.g reading a file, fetching data from a
server, waiting for a timer, it has two choices: sit and wait doing nothing, or go do something
else while it waits.

`Blocking` means sitting and waiting. Your program freezes until the result comes back.
Nothing else can happen in the meantime.

`Non-blocking (async)` means your program starts the task, steps away, and comes back to collect
the result when it is ready. While it waits, it can work on other things.

In Rust, async code is written using two keywords:
- `async` marks a function as non-blocking. Calling it does not run it immediately. It gives you
back a task that is ready to be run.
- `.await` is where you hand control back and say "run this task, and come back to me when it is
done."

Before your non-blocking code can run, something needs to manage it, deciding which tasks run,
when they run, and what to do while they are waiting. That manager is called a runtime. 

Think of it like a coordinator at a call center; instead of one person sitting idle waiting for a
call back, the coordinator keeps everyone busy by assigning new work while others wait.

Tokio is that runtime for Rust. Without it, `async` functions and `.await` do nothing on their own.

{{#include rt/tokio-rt-macro.md}}
{{#include rt/tokio-rt-builder.md}}

{{#include ../links.md}}
