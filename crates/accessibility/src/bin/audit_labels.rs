// ANCHOR: recipe
use anyhow::{Context, Result, bail};
use atspi::proxy::accessible::{AccessibleProxy, ObjectRefExt};
use atspi::{AccessibilityConnection, ObjectRefOwned, RelationType, Role, State, StateSet};
use std::env;

/// Stop after this many objects, in case an application exposes a very large tree.
const MAX_OBJECTS: usize = 5000;

#[tokio::main]
async fn main() -> Result<()> {
    let wanted = env::args()
        .nth(1)
        .context("usage: audit_labels <application name>")?;
    let a11y = AccessibilityConnection::new().await?;
    let conn = a11y.connection();
    let app = find_application(&a11y, &wanted).await?;

    let mut problems = 0;
    let mut stack = vec![(app, String::new())];
    let mut visited = 0;
    while let Some((object, path)) = stack.pop() {
        visited += 1;
        if visited > MAX_OBJECTS {
            println!("... stopped after {MAX_OBJECTS} objects");
            break;
        }
        let node = object.as_accessible_proxy(conn).await?;
        let states = node.get_state().await?;
        if !path.is_empty() && !states.contains(State::Showing) {
            continue;
        }
        let role = node.get_role().await?;
        let name = node.name().await?;
        let path = if path.is_empty() {
            describe(role, &name)
        } else {
            format!("{path} > {}", describe(role, &name))
        };

        // A screen reader can only say "button" for a control with no name and no label.
        if is_interactive(role, states)
            && name.trim().is_empty()
            && !has_label(&a11y, &node).await?
        {
            problems += 1;
            println!("unlabeled {role}: {path}");
        }

        let children = node.get_children().await?;
        stack.extend(
            children
                .into_iter()
                .rev()
                .map(|child| (child, path.clone())),
        );
    }

    if problems > 0 {
        bail!("{problems} control(s) have no accessible name");
    }
    println!("Every control on screen has an accessible name.");
    Ok(())
}

/// Controls a user operates, which a screen reader has to announce by name.
fn is_interactive(role: Role, states: StateSet) -> bool {
    // GTK 4 reports text fields as editable `text` rather than `entry`.
    (role == Role::Text && states.contains(State::Editable))
        || matches!(
            role,
            Role::Button
                | Role::ToggleButton
                | Role::CheckBox
                | Role::RadioButton
                | Role::ComboBox
                | Role::Entry
                | Role::PasswordText
                | Role::SpinButton
                | Role::Slider
                | Role::Link
                | Role::MenuItem
                | Role::PageTab
        )
}

/// Whether another object with text, such as a label next to an entry, labels this one.
/// A label with no text does not count: the screen reader still has nothing to say.
async fn has_label(a11y: &AccessibilityConnection, node: &AccessibleProxy<'_>) -> Result<bool> {
    for (relation, targets) in node.get_relation_set().await? {
        if relation != RelationType::LabelledBy {
            continue;
        }
        for label in targets {
            let text = label
                .as_accessible_proxy(a11y.connection())
                .await?
                .name()
                .await?;
            if !text.trim().is_empty() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn describe(role: Role, name: &str) -> String {
    if name.is_empty() {
        role.to_string()
    } else {
        format!("{role} {name:?}")
    }
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
// ANCHOR_END: recipe

#[cfg(test)]
mod tests {
    use super::{describe, is_interactive};
    use atspi::{Role, State, StateSet};

    #[test]
    fn controls_need_a_name_but_labels_do_not() {
        let none = StateSet::empty();
        assert!(is_interactive(Role::Button, none));
        assert!(is_interactive(Role::Entry, none));
        assert!(!is_interactive(Role::Label, none));
        assert!(!is_interactive(Role::Panel, none));
    }

    #[test]
    fn editable_text_is_a_field_but_read_only_text_is_not() {
        assert!(is_interactive(Role::Text, StateSet::new(State::Editable)));
        assert!(!is_interactive(Role::Text, StateSet::empty()));
    }

    #[test]
    fn describes_an_object_by_role_and_name() {
        assert_eq!(
            describe(Role::Button, "Save"),
            format!("{} \"Save\"", Role::Button)
        );
        assert_eq!(describe(Role::Button, ""), Role::Button.to_string());
    }
}
