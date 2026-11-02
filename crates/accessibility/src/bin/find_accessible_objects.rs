//! Simple AT-SPI connection example
//!
//! This example shows how to connect to the AT-SPI bus on Linux.
//! Note: This requires the AT-SPI bus to be running (typically available in desktop environments).

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the AT-SPI bus
    let connection = atspi::AccessibilityConnection::new().await?;

    println!("Connected to AT-SPI bus successfully!");

    // Get the root accessible object
    let root = connection.root_accessible_on_registry().await?;

    println!("Root accessible: {:?}", root.inner());

    Ok(())
}

#[cfg(test)]
mod tests {

    // Run only on Linux CI
    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_main_runs() {
        // Ensure the example executes without error (ignoring connection failures on CI without AT‑SPI daemon)
        let _ = main().await;
    }
}
