# Accessibility

Recipes for the Linux accessibility bus (AT-SPI), the interface screen readers
such as Orca use to read and follow what is on screen. They are included in
the book at `src/accessibility/linux-at-spi.md`.

| Binary | What it does |
|--------|--------------|
| `list_applications` | Lists the applications on the accessibility bus by name |
| `dump_tree <app>` | Prints an application's accessibility tree: roles, names and states |
| `audit_labels <app>` | Reports controls with no accessible name; exits with an error if it finds any |
| `track_focus` | Announces each control that receives keyboard focus, like a screen reader |

These need a Linux desktop session with accessibility enabled, for example:

```sh
gsettings set org.gnome.desktop.interface toolkit-accessibility true
cargo run --bin list_applications
cargo run --bin audit_labels gedit
```
