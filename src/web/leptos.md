# Full Stack Web

[Leptos][book] is a full stack web framework. The first recipe renders HTML
on the server only. The second adds hydration so the client can re-run the
same components as WebAssembly for interactivity.

[book]: https://book.leptos.dev/

## Return filtered results as HTML

[![leptos-badge]][leptos] [![leptos-router-badge]][leptos-router] [![leptos-axum-badge]][leptos-axum] [![axum-badge]][axum] [![tokio-badge]][tokio] [![cat-net-badge]][cat-net]

The crate uses [`leptos`][leptos] with the `ssr` feature, plus
[`leptos_router`][leptos-router], [`leptos_axum`][leptos-axum],
[`axum`][axum], and [`tokio`][tokio]. It lives outside the main workspace
because `leptos` needs [`resolver = "3"`][resolver-3].

[resolver-3]: https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions

### Define the router

[`Router`][router-component] is responsible for what page is shown when the
browser hits the app. The [`path!`][path-macro] macro is used to define what
URL matches. The `:query` is a template — it matches anything and stores the
value as `query` for the component to read later.

[router-component]: https://docs.rs/leptos_router/*/leptos_router/components/fn.Router.html
[path-macro]: https://docs.rs/leptos_router/*/leptos_router/macro.path.html

```rust,ignore
{{#include ../../crates/web_leptos/src/main.rs:10:23}}
```

[`<A>`][a-component] anchors links while `<Routes fallback=...>` is used to
provide a default.

[a-component]: https://docs.rs/leptos_router/*/leptos_router/components/fn.A.html

More in the Leptos book: [Defining `<Routes/>`][book-routes] and
[Nested Routing][book-nested].

[book-routes]: https://book.leptos.dev/router/16_routes.html
[book-nested]: https://book.leptos.dev/router/17_nested_routing.html

### Read the parameter and fetch on the server

[`use_params`][use-params] holds the values from the `Router`.
[`Resource::new`][resource-new] uses a closure to fetch async. The closure
runs on the server and [`<Suspense>`][suspense] lets the user know the server
is processing.

[use-params]: https://docs.rs/leptos_router/*/leptos_router/hooks/fn.use_params.html
[resource-new]: https://docs.rs/leptos/*/leptos/prelude/struct.Resource.html#method.new
[suspense]: https://book.leptos.dev/async/11_suspense.html

```rust,ignore
{{#include ../../crates/web_leptos/src/main.rs:33:61}}
```

`filter_crates` is a plain `async fn` that runs on the server. It resolves
the data file via [`env!`][env-macro]`("CARGO_MANIFEST_DIR")` so the path
works regardless of where the binary runs from.

[env-macro]: https://doc.rust-lang.org/std/macro.env.html

```rust,ignore
{{#include ../../crates/web_leptos/src/main.rs:73:84}}
```

More in the Leptos book: [Params and Queries][book-params],
[Loading Data with Resources][book-resources], and [Suspense][book-suspense].

[book-params]: https://book.leptos.dev/router/18_params_and_queries.html
[book-resources]: https://book.leptos.dev/async/10_resources.html
[book-suspense]: https://book.leptos.dev/async/11_suspense.html

### Write a reusable component

Components are functions annotated with `#[component]`. Props are ordinary
function arguments.

```rust,ignore
{{#include ../../crates/web_leptos/src/main.rs:63:71}}
```

More in the Leptos book: [Components and Props][book-components] and
[Passing Children to Components][book-children].

[book-components]: https://book.leptos.dev/view/03_components.html
[book-children]: https://book.leptos.dev/view/09_component_children.html

### Wire up Axum

`generate_route_list(App)` walks the `Routes` tree to enumerate every path the
router knows about. `leptos_routes` registers those paths as Axum GET handlers
that render the shell on each request.

```rust,ignore
{{#include ../../crates/web_leptos/src/main.rs:86:120}}
```

More in the Leptos book: [The Life of a Page Load][book-life] and
[`cargo-leptos`][book-cargo-leptos] (the tool most real apps graduate to).

[book-life]: https://book.leptos.dev/ssr/22_life_cycle.html
[book-cargo-leptos]: https://book.leptos.dev/ssr/21_cargo_leptos.html

Run it:

```shell
cargo run --manifest-path crates/web_leptos/Cargo.toml
# http://127.0.0.1:3000/
# http://127.0.0.1:3000/filter/serde
# http://127.0.0.1:3000/filter/tokio
```

## Synchronize component state with the server

[![leptos-badge]][leptos] [![leptos-axum-badge]][leptos-axum] [![axum-badge]][axum] [![tokio-badge]][tokio] [![wasm-bindgen-badge]][wasm-bindgen] [![cat-net-badge]][cat-net]

Same `data/crates.txt` as the first recipe, but the user types in a text
box and results update as they type — no page reload. The file stays on
the server; the query round-trips through a server function.

The recipe uses [`cargo-leptos`][cargo-leptos] as the build tool and a
[`wasm32-unknown-unknown`][wasm-target] target:

```shell
cargo install cargo-leptos
rustup target add wasm32-unknown-unknown
```

[cargo-leptos]: https://github.com/leptos-rs/cargo-leptos
[wasm-target]: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html

### Split ssr and hydrate with feature flags

One crate, two builds. `ssr` pulls in the server deps and `hydrate` turns on
the WASM client. [`cargo-leptos`][cargo-leptos] drives both from
[`[package.metadata.leptos]`][metadata-leptos].

[metadata-leptos]: https://github.com/leptos-rs/cargo-leptos#parameters-reference

```toml
{{#include ../../crates/web_leptos_hydrate/Cargo.toml:10:41}}
```

### Put the filter behind a server function

[`#[server]`][server-macro] makes `filter_crates` a function the client can
call. The body only compiles into the `ssr` build and on `hydrate` the macro
leaves a stub that POSTs to the server. [`tokio::fs`][tokio-fs] reads the
file on the server as before.

[server-macro]: https://docs.rs/leptos/*/leptos/attr.server.html
[tokio-fs]: https://docs.rs/tokio/*/tokio/fs/index.html

```rust,ignore
{{#include ../../crates/web_leptos_hydrate/src/lib.rs:10:20}}
```

### Bind an input to a resource

The `App` component is plain reactive Leptos — not behind [`ssr`][feat-ssr]
nor [`hydrate`][feat-hydrate]. [`signal`][signal] holds the query,
[`event_target_value`][etv] pulls the string out of each keystroke, and
[`Resource::new`][resource-new] reruns `filter_crates` whenever the query
changes. [`<Suspense>`][suspense] paints results or the "Searching..."
fallback.

[feat-ssr]: https://book.leptos.dev/ssr/index.html
[feat-hydrate]: https://book.leptos.dev/ssr/22_life_cycle.html
[signal]: https://docs.rs/leptos/*/leptos/prelude/fn.signal.html
[etv]: https://docs.rs/leptos/*/leptos/prelude/fn.event_target_value.html

```text
{{#include ../../crates/web_leptos_hydrate/src/lib.rs:22:50}}
```

### Server side: render the shell with `HydrationScripts`

[`<HydrationScripts>`][hydration-scripts] emits the `<script>` tag that loads
the WASM bundle. [`<AutoReload>`][auto-reload] ships a reload script when
`cargo leptos watch` is running.

[hydration-scripts]: https://docs.rs/leptos/*/leptos/hydration/fn.HydrationScripts.html
[auto-reload]: https://docs.rs/leptos/*/leptos/hydration/fn.AutoReload.html

```rust,ignore
{{#include ../../crates/web_leptos_hydrate/src/lib.rs:52:67}}
```

### Client side: `hydrate` the body

[`hydrate_body`][hydrate-body] runs the same `App` function in the browser
and attaches it to the server-rendered DOM. `#[wasm_bindgen]` marks it as
the entry point the generated JS will call.

[hydrate-body]: https://docs.rs/leptos/*/leptos/mount/fn.hydrate_body.html

```rust,ignore
{{#include ../../crates/web_leptos_hydrate/src/lib.rs:69:74}}
```

### Wire up Axum behind the `ssr` feature

[`get_configuration`][get-configuration] reads the
`[package.metadata.leptos]` section. `leptos_routes` also registers the
server function endpoint automatically.

[get-configuration]: https://docs.rs/leptos/*/leptos/config/fn.get_configuration.html

```rust,ignore
{{#include ../../crates/web_leptos_hydrate/src/main.rs}}
```

Run it:

```shell
cargo leptos watch --manifest-path crates/web_leptos_hydrate/Cargo.toml
# http://127.0.0.1:3000/
```

More in the Leptos book: [Server Functions][book-server-fn] and
[`cargo-leptos`][book-cargo-leptos-2].

[book-server-fn]: https://book.leptos.dev/server/25_server_functions.html
[book-cargo-leptos-2]: https://book.leptos.dev/ssr/21_cargo_leptos.html

{{#include ../links.md}}
