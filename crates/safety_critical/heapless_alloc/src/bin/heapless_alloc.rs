use heapless::{String, Vec};

/// A fixed-capacity event log that never touches the heap.
///
/// In real-time and safety-critical systems the global allocator is
/// forbidden because allocation time is unbounded and fragmentation
/// can cause silent OOM failures in long-running firmware.
///
/// [`heapless::Vec`] stores up to `N` elements on the stack (or in a
/// `static`).  The capacity is fixed at compile time, so the memory
/// footprint is constant and predictable.
struct EventLog<const N: usize> {
    entries: Vec<Event, N>,
}

#[derive(Debug, Clone)]
struct Event {
    timestamp_ms: u32,
    code: u16,
}

impl<const N: usize> EventLog<N> {
    /// Creates an empty log.
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Records an event.  Returns `Err` if the log is full instead
    /// of panicking or allocating—the caller decides what to do.
    fn record(&mut self, timestamp_ms: u32, code: u16) -> Result<(), Event> {
        let event = Event { timestamp_ms, code };
        self.entries.push(event).map_err(|e| e)
    }

    /// Returns how many events have been recorded.
    fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns the most recent event, if any.
    fn latest(&self) -> Option<&Event> {
        self.entries.last()
    }
}

/// Formats a sensor label without heap allocation.
///
/// [`heapless::String<N>`] works like `std::string::String` but
/// stores up to `N` bytes on the stack.  `write!` returns `Err` if
/// the formatted text would exceed capacity.
fn format_label(sensor_id: u16, value: f32) -> Result<String<32>, core::fmt::Error> {
    use core::fmt::Write;
    let mut buf: String<32> = String::new();
    write!(buf, "S{sensor_id}={value:.1}")?;
    Ok(buf)
}

fn main() {
    // A log that holds at most 8 events — zero heap allocation.
    let mut log: EventLog<8> = EventLog::new();

    log.record(100, 0x01).expect("log not full");
    log.record(200, 0x02).expect("log not full");
    log.record(300, 0xFF).expect("log not full");

    println!("logged {} events", log.len());
    println!("latest: {:?}", log.latest().unwrap());

    // Stack-allocated string formatting.
    let label = format_label(42, 3.14).expect("fits in 32 bytes");
    println!("label: {label}");

    // Demonstrate capacity enforcement — the 9th push returns Err.
    let mut full_log: EventLog<2> = EventLog::new();
    full_log.record(0, 1).unwrap();
    full_log.record(1, 2).unwrap();
    let overflow = full_log.record(2, 3);
    assert!(overflow.is_err());
    println!("overflow correctly rejected");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_log_records_and_retrieves() {
        let mut log: EventLog<4> = EventLog::new();
        log.record(10, 0xAA).unwrap();
        log.record(20, 0xBB).unwrap();

        assert_eq!(log.len(), 2);
        assert_eq!(log.latest().unwrap().code, 0xBB);
    }

    #[test]
    fn test_event_log_rejects_overflow() {
        let mut log: EventLog<1> = EventLog::new();
        assert!(log.record(0, 1).is_ok());
        assert!(log.record(1, 2).is_err());
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn test_format_label() {
        let label = format_label(7, 25.0).unwrap();
        assert_eq!(label.as_str(), "S7=25.0");
    }

    #[test]
    fn test_format_label_overflow() {
        // heapless::String<4> can only hold 4 bytes — "S1=0.0" won't fit.
        use core::fmt::Write;
        let mut tiny: heapless::String<4> = heapless::String::new();
        let result = write!(tiny, "S1=99.9");
        assert!(result.is_err());
    }
}
