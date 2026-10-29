use std::time::Duration;

#[tokio::main]
async fn main() {
    let slow = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        "slow task done"
    });
    let fast = tokio::spawn(async { "fast task done" });

    println!("{}", fast.await.unwrap_or("fast task failed"));
    println!("{}", slow.await.unwrap_or("slow task failed"));
}
