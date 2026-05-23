use anyhow::{anyhow, Result};
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::new(
        &engine,
        r#"
        (module
            (memory (export "memory") 1)
            (func (export "add_from_memory") (result i32)
                (i32.add
                    (i32.load (i32.const 0))
                    (i32.load (i32.const 4))))
        )
        "#,
    )?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or_else(|| anyhow!("memory export not found"))?;
    let add = instance.get_typed_func::<(), i32>(&mut store, "add_from_memory")?;

    let data = memory.data_mut(&mut store);
    data[0..4].copy_from_slice(&17i32.to_le_bytes());
    data[4..8].copy_from_slice(&25i32.to_le_bytes());

    let result = add.call(&mut store, ())?;
    println!("17 + 25 = {result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_from_memory() -> Result<()> {
        let engine = Engine::default();
        let module = Module::new(
            &engine,
            r#"
            (module
                (memory (export "memory") 1)
                (func (export "add_from_memory") (result i32)
                    (i32.add
                        (i32.load (i32.const 0))
                        (i32.load (i32.const 4))))
            )
            "#,
        )?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow!("memory export not found"))?;
        let add = instance.get_typed_func::<(), i32>(&mut store, "add_from_memory")?;
        let data = memory.data_mut(&mut store);
        data[0..4].copy_from_slice(&10i32.to_le_bytes());
        data[4..8].copy_from_slice(&32i32.to_le_bytes());
        assert_eq!(add.call(&mut store, ())?, 42);
        Ok(())
    }
}
