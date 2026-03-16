use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A hand-rolled future that resolves after a deadline.
///
/// This is intentionally simple—no timer wheel, no reactor—just a
/// busy-polling future that shows the three things every custom
/// `Future` must handle:
///
/// 1. **`Pin<&mut Self>`** — guarantees our struct won't move in
///    memory.  Because `Delay` contains only `Instant` and
///    `Duration` (both `Unpin`), the compiler auto-implements
///    `Unpin` for us and pinning is a no-op here.  If the struct
///    held a self-referential borrow (like a hand-written async
///    state machine), pinning would *prevent* the struct from
///    being moved after the first poll, which would invalidate
///    the borrow.
///
/// 2. **`Poll::Pending` vs `Poll::Ready`** — returning `Pending`
///    tells the executor "I'm not done yet; wake me later."
///
/// 3. **`cx.waker()`** — the mechanism to *schedule* a re-poll.
///    Without calling `wake()`, the executor would never poll us
///    again and the future would hang.
struct Delay {
    /// When we should resolve.
    deadline: Instant,
}

impl Delay {
    fn new(dur: Duration) -> Self {
        Self {
            deadline: Instant::now() + dur,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.deadline {
            // Deadline reached — resolve the future.
            Poll::Ready(())
        } else {
            // Not ready yet.
            //
            // We MUST arrange for `wake()` to be called later or the
            // executor will never poll us again.  A production timer
            // would register with a reactor; here we just ask to be
            // re-polled immediately. The executor may yield to other
            // tasks before coming back to us.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    println!("waiting 10 ms …");

    // Use the custom future just like any other async expression.
    Delay::new(Duration::from_millis(10)).await;

    println!("done in {:?}", start.elapsed());
    assert!(start.elapsed() >= Duration::from_millis(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delay_completes_after_deadline() {
        let start = Instant::now();
        Delay::new(Duration::from_millis(50)).await;
        assert!(start.elapsed() >= Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_zero_delay_resolves_immediately() {
        let start = Instant::now();
        Delay::new(Duration::ZERO).await;
        assert!(start.elapsed() < Duration::from_millis(10));
    }
}
