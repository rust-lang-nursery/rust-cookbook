use anyhow::Result;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

bindgen!({
    world: "greeter",
    path: "../wasm_component_guest/wit/world.wit",
});

struct State {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(
        &engine,
        "crates/wasm_component_guest/target/wasm32-wasip2/release/greeter.wasm",
    )?;

    let mut linker: Linker<State> = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    let wasi = WasiCtxBuilder::new().build();
    let mut store = Store::new(&engine, State {
        table: ResourceTable::new(),
        wasi,
    });

    let greeter = Greeter::instantiate(&mut store, &component, &linker)?;
    let message = greeter.call_greet(&mut store, "WebAssembly")?;
    println!("{message}");
    Ok(())
}
