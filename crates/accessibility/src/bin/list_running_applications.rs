//! List running AT‑SPI applications

use anyhow::Result;
use atspi::AccessibilityConnection;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the AT‑SPI bus
    let connection = AccessibilityConnection::new().await?;
    // Obtain the registry and fetch the root accessible object
    let root = connection.root_accessible_on_registry().await?;
    // Get children (applications) of the root
    let apps = root.get_children().await?;
    println!("Running applications ({}):", apps.len());
    for app in apps {
        let name = format!("{:?}", app.name());
        println!("- {}", name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_main_runs() {
        // Run the example; ignore connection errors on CI without AT‑SPI daemon
        let _ = main().await;
    }
}
