// ANCHOR: recipe
use anyhow::Result;
use atspi::events::object::StateChangedEvent;
use atspi::proxy::accessible::ObjectRefExt;
use atspi::{AccessibilityConnection, Event, ObjectEvents, Role, State};
use futures_util::StreamExt;
use std::pin::pin;

#[tokio::main]
async fn main() -> Result<()> {
    let a11y = AccessibilityConnection::new().await?;
    // Ask applications to send state changes; focus is one of them.
    a11y.register_event::<StateChangedEvent>().await?;
    let mut events = pin!(a11y.event_stream());

    println!("Move the keyboard focus in any application. Press Ctrl+C to stop.");
    while let Some(event) = events.next().await {
        let Ok(Event::Object(ObjectEvents::StateChanged(change))) = event else {
            continue;
        };
        if change.state != State::Focused || !change.enabled {
            continue;
        }
        // The object can be gone by the time we ask about it, such as a closed menu.
        let Ok(object) = change.item.as_accessible_proxy(a11y.connection()).await else {
            continue;
        };
        if let (Ok(role), Ok(name)) = (object.get_role().await, object.name().await) {
            println!("{}", announcement(role, &name));
        }
    }
    Ok(())
}

/// What a screen reader says when focus lands on an object, such as `Save, button`.
fn announcement(role: Role, name: &str) -> String {
    if name.trim().is_empty() {
        format!("unlabeled {role}")
    } else {
        format!("{name}, {role}")
    }
}
// ANCHOR_END: recipe

#[cfg(test)]
mod tests {
    use super::announcement;
    use atspi::Role;

    #[test]
    fn announces_the_name_then_the_role() {
        assert_eq!(
            announcement(Role::Button, "Save"),
            format!("Save, {}", Role::Button)
        );
    }

    #[test]
    fn calls_out_a_control_without_a_name() {
        assert_eq!(
            announcement(Role::Button, " "),
            format!("unlabeled {}", Role::Button)
        );
    }
}
