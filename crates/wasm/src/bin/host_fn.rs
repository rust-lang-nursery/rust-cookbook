use anyhow::Result;
use wasmtime::{Caller, Engine, Extern, Func, Instance, Module, Store};

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::new(
        &engine,
        r#"
        (module
            (import "host" "print" (func $print (param i32 i32)))
            (memory (export "memory") 1)
            (data (i32.const 0) "Hello from WebAssembly!")
            (func (export "run")
                (call $print (i32.const 0) (i32.const 23)))
        )
        "#,
    )?;

    let mut store = Store::new(&engine, ());
    let print_fn = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
            if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
                let data = mem.data(&caller);
                let bytes = &data[ptr as usize..(ptr + len) as usize];
                if let Ok(s) = std::str::from_utf8(bytes) {
                    println!("[wasm] {s}");
                }
            }
        },
    );

    let imports = [print_fn.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    run.call(&mut store, ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_host_fn_receives_message() -> Result<()> {
        let engine = Engine::default();
        let module = Module::new(
            &engine,
            r#"
            (module
                (import "host" "print" (func $print (param i32 i32)))
                (memory (export "memory") 1)
                (data (i32.const 0) "Hello from WebAssembly!")
                (func (export "run")
                    (call $print (i32.const 0) (i32.const 23)))
            )
            "#,
        )?;

        let captured = Arc::new(Mutex::new(String::new()));
        let captured_clone = captured.clone();
        let mut store = Store::new(&engine, ());
        let print_fn = Func::wrap(
            &mut store,
            move |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
                if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
                    let data = mem.data(&caller);
                    let bytes = &data[ptr as usize..(ptr + len) as usize];
                    if let Ok(s) = std::str::from_utf8(bytes) {
                        *captured_clone.lock().unwrap() = s.to_string();
                    }
                }
            },
        );

        let imports = [print_fn.into()];
        let instance = Instance::new(&mut store, &module, &imports)?;
        let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
        run.call(&mut store, ())?;

        assert_eq!(*captured.lock().unwrap(), "Hello from WebAssembly!");
        Ok(())
    }
}
