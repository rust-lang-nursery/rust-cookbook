use chrono::{DateTime, Duration, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();
    println!("now:      {}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("tomorrow: {}", (now + Duration::days(1)).to_rfc3339());
}
