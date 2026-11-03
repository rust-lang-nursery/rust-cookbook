// ANCHOR: recipe
use anyhow::{Context, Result, bail};
use atspi::proxy::accessible::ObjectRefExt;
use atspi::{AccessibilityConnection, ObjectRefOwned, State, StateSet};
use std::env;

/// Stop after this many objects, in case an application exposes a very large tree.
const MAX_OBJECTS: usize = 2000;

#[tokio::main]
async fn main() -> Result<()> {
    let wanted = env::args()
        .nth(1)
        .context("usage: dump_tree <application name>")?;
    let a11y = AccessibilityConnection::new().await?;
    let conn = a11y.connection();
    let app = find_application(&a11y, &wanted).await?;

    // Depth-first, so each object is printed under its parent.
    let mut stack = vec![(app, 0)];
    let mut visited = 0;
    while let Some((object, depth)) = stack.pop() {
        visited += 1;
        if visited > MAX_OBJECTS {
            println!("... stopped after {MAX_OBJECTS} objects");
            break;
        }
        let node = object.as_accessible_proxy(conn).await?;
        let states = node.get_state().await?;
        // Skip what is not on screen, such as closed menus: a screen reader skips it too.
        if depth > 0 && !states.contains(State::Showing) {
            continue;
        }
        let role = node.get_role().await?;
        let name = node.name().await?;
        // The application object itself has no meaningful states (it is never "sensitive").
        let states = if depth == 0 {
            String::new()
        } else {
            notable_states(states)
        };
        println!("{:indent$}{role} {name:?}{states}", "", indent = depth * 2);
        let children = node.get_children().await?;
        stack.extend(children.into_iter().rev().map(|child| (child, depth + 1)));
    }
    Ok(())
}

/// Finds a running application by its accessible name, ignoring case.
async fn find_application(a11y: &AccessibilityConnection, wanted: &str) -> Result<ObjectRefOwned> {
    let root = a11y.root_accessible_on_registry().await?;
    for app in root.get_children().await? {
        let name = app
            .as_accessible_proxy(a11y.connection())
            .await?
            .name()
            .await?;
        if name.eq_ignore_ascii_case(wanted) {
            return Ok(app);
        }
    }
    bail!("no application named {wanted:?} on the accessibility bus")
}

/// The states a user cares about, such as ` [focusable, checked]`, or nothing.
fn notable_states(states: StateSet) -> String {
    let mut shown: Vec<String> = [
        State::Focusable,
        State::Focused,
        State::Checked,
        State::Editable,
    ]
    .into_iter()
    .filter(|state| states.contains(*state))
    .map(|state| state.to_string())
    .collect();
    if !states.contains(State::Sensitive) {
        shown.push("disabled".into());
    }
    if shown.is_empty() {
        String::new()
    } else {
        format!(" [{}]", shown.join(", "))
    }
}
// ANCHOR_END: recipe

#[cfg(test)]
mod tests {
    use super::notable_states;
    use atspi::{State, StateSet};

    #[test]
    fn lists_only_the_states_a_user_cares_about() {
        let states = StateSet::new(State::Focusable | State::Sensitive | State::Showing);
        assert_eq!(notable_states(states), " [focusable]");
    }

    #[test]
    fn says_when_a_control_is_disabled() {
        assert_eq!(notable_states(StateSet::new(State::Showing)), " [disabled]");
    }
}
