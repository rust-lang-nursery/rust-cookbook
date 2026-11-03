use anyhow::Result;
use atspi::AccessibilityConnection;
use atspi::proxy::accessible::ObjectRefExt;

#[tokio::main]
async fn main() -> Result<()> {
    let a11y = AccessibilityConnection::new().await?;

    // Every application that supports accessibility is a child of the registry's root.
    let root = a11y.root_accessible_on_registry().await?;
    for app in root.get_children().await? {
        let app = app.as_accessible_proxy(a11y.connection()).await?;
        let name = app.name().await?;
        let windows = app.child_count().await?;
        if name.is_empty() {
            println!("(unnamed), {windows} window(s)");
        } else {
            println!("{name}, {windows} window(s)");
        }
    }
    Ok(())
}
