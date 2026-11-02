# Linux Accessibility (AT-SPI)

[![atspi-badge]][atspi] [![cat-accessibility-badge]][cat-accessibility]

<!-- needs-no-features: atspi, tokio -->

## Overview

The Assistive Technology Service Provider Interface (AT‑SPI) is the D‑Bus based accessibility framework used on Linux desktops. It enables assistive technologies (screen readers, magnifiers, etc.) to discover and control UI elements.

The **[`AccessibilityConnection`]** hosts a **[`Registry`]** that tracks running **[`Application`]** instances; each application exposes **[`Accessible`]** objects, which emit **[`EventListener`]** notifications.

## Connecting to the AT‑SPI Bus

```rust,no_run
{{#include ../../crates/accessibility/src/bin/find_accessible_objects.rs::1-21}}
```

## Listing Running Applications

This recipe demonstrates how to enumerate the running applications on the AT‑SPI bus. It obtains the `Registry` from the `AccessibilityConnection`, then queries the list of `Application` objects and prints each application's name.

```rust,no_run
{{#include ../../crates/accessibility/src/bin/list_running_applications.rs::1-20}}
```

> **Note:** Requires the AT‑SPI daemon to be running.

---

[atspi]: https://docs.rs/atspi/*/atspi/
[atspi-badge]: https://img.shields.io/crates/v/atspi.svg?label=atspi

[`AccessibilityConnection`]: https://docs.rs/atspi/latest/atspi/struct.AccessibilityConnection.html
[`Registry`]: https://docs.rs/atspi/latest/atspi/struct.Registry.html
[`Application`]: https://docs.rs/atspi/latest/atspi/struct.Application.html
[`Accessible`]: https://docs.rs/atspi/latest/atspi/struct.Accessible.html
[`EventListener`]: https://docs.rs/atspi/latest/atspi/struct.EventListener.html

{{#include ../links.md}}
