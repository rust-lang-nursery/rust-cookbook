use anyhow::Result;
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::new(
        &engine,
        r#"
        (module
            (func (export "add") (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add)
        )
        "#,
    )?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let result = add.call(&mut store, (5, 37))?;
    println!("5 + 37 = {result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() -> Result<()> {
        let engine = Engine::default();
        let module = Module::new(
            &engine,
            r#"
            (module
                (func (export "add") (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add)
            )
            "#,
        )?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
        assert_eq!(add.call(&mut store, (5, 37))?, 42);
        Ok(())
    }
}
