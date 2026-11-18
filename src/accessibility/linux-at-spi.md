# Linux Accessibility (AT-SPI)

Screen readers such as Orca, magnifiers and switch-access tools on Linux learn
what is on screen through AT-SPI, a D-Bus service that applications publish
their user interface to. Every application is a tree of accessible objects,
each with a [`Role`] (button, text field, menu item), a name a screen reader
reads aloud, and a set of [`State`]s (focused, checked, disabled). The
[`atspi`] crate connects to that bus with [`AccessibilityConnection`] and talks
to each object through an [`AccessibleProxy`].

These recipes run on a Linux desktop with accessibility enabled (GNOME and KDE
turn it on when an assistive technology runs; `gsettings set
org.gnome.desktop.interface toolkit-accessibility true` turns it on for GTK
applications).

> This recipe requires the [`macros`] and [`rt-multi-thread`] feature flags of `tokio` to be enabled in `Cargo.toml`.

## List the applications on the accessibility bus

[![atspi-badge]][atspi] [![tokio-badge]][tokio] [![cat-accessibility-badge]][cat-accessibility]

The registry's root object has one child per application that publishes its
user interface. [`ObjectRefExt::as_accessible_proxy`] turns each child
reference into a proxy, which reads the application's name and how many
windows it has.

```rust,no_run
{{#include ../../crates/accessibility/src/bin/list_applications.rs}}
```

## Print the accessibility tree of an application

[![atspi-badge]][atspi] [![tokio-badge]][tokio] [![cat-accessibility-badge]][cat-accessibility]

This is what a screen reader knows about an application: each object's role,
name and the states a user cares about. The walk skips objects without the
`Showing` state, such as closed menus, just as a screen reader does. Run it
with the application's name from the previous recipe, such as
`cargo run --bin dump_tree gedit`.

```rust,no_run
{{#include ../../crates/accessibility/src/bin/dump_tree.rs:recipe}}
```

## Find controls a screen reader cannot name

[![atspi-badge]][atspi] [![tokio-badge]][tokio] [![cat-accessibility-badge]][cat-accessibility]

An icon button without an accessible name is announced as just "button", so a
screen reader user cannot tell what it does. This audit walks the controls on
screen and reports every button, field, tab or link that has neither a name
nor a label pointing at it through a [`RelationType::LabelledBy`] relation. It
exits with an error when it finds any, so it can run as a check in CI, such as
`cargo run --bin audit_labels gedit`.

```rust,no_run
{{#include ../../crates/accessibility/src/bin/audit_labels.rs:recipe}}
```

## Announce the focused control like a screen reader

[![atspi-badge]][atspi] [![tokio-badge]][tokio] [![cat-accessibility-badge]][cat-accessibility]

A screen reader speaks whatever receives keyboard focus. Registering for
[`StateChangedEvent`] asks applications to report state changes, and the event
stream from [`AccessibilityConnection::event_stream`] delivers them; read with
[`StreamExt::next`], each one that turns `Focused` on is announced with the
object's name and role. Press <kbd>Tab</kbd> in any application to hear it.

```rust,no_run
{{#include ../../crates/accessibility/src/bin/track_focus.rs:recipe}}
```

[`AccessibilityConnection::event_stream`]: https://docs.rs/atspi-connection/*/atspi_connection/struct.AccessibilityConnection.html#method.event_stream
[`AccessibilityConnection`]: https://docs.rs/atspi-connection/*/atspi_connection/struct.AccessibilityConnection.html
[`AccessibleProxy`]: https://docs.rs/atspi-proxies/*/atspi_proxies/accessible/struct.AccessibleProxy.html
[`ObjectRefExt::as_accessible_proxy`]: https://docs.rs/atspi-proxies/*/atspi_proxies/accessible/trait.ObjectRefExt.html#tymethod.as_accessible_proxy
[`RelationType::LabelledBy`]: https://docs.rs/atspi-common/*/atspi_common/relation_type/enum.RelationType.html#variant.LabelledBy
[`Role`]: https://docs.rs/atspi-common/*/atspi_common/role/enum.Role.html
[`State`]: https://docs.rs/atspi-common/*/atspi_common/state/enum.State.html
[`StateChangedEvent`]: https://docs.rs/atspi-common/*/atspi_common/events/object/struct.StateChangedEvent.html
[`StreamExt::next`]: https://docs.rs/futures-util/*/futures_util/stream/trait.StreamExt.html#method.next
[`atspi`]: https://docs.rs/atspi/*/atspi/
[`macros`]: https://docs.rs/crate/tokio/*/features#macros
[`rt-multi-thread`]: https://docs.rs/crate/tokio/*/features#rt-multi-thread
[atspi-badge]: https://img.shields.io/crates/v/atspi.svg?label=atspi
[atspi]: https://docs.rs/atspi/*/atspi/

{{#include ../links.md}}
