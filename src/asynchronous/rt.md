# Async Runtime

Before your non-blocking code can run, something needs to manage it, deciding which tasks run,
when they run, and what to do while they are waiting. That manager is called a runtime. 

Think of it like a coordinator at a call center; instead of one person sitting idle waiting for a
call back, the coordinator keeps everyone busy by assigning new work while others wait.

Tokio is that runtime for Rust. Without it, `async` functions and `.await` do nothing on their own.

{{#include rt/tokio-rt-macro.md}}
{{#include rt/tokio-rt-builder.md}}

{{#include ../links.md}}
