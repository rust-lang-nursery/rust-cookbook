## Share mutable structure between owners with `Rc` and `RefCell`
[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

Uses [`Rc<T>`] to share ownership of one task between multiple dependents and [`RefCell<T>`] to mutate that shared task through [`RefCell::try_borrow_mut`]. 
This recipe updates one shared task and reads the result through each dependent using [`RefCell::borrow`].

This pattern is useful when several parts of a single-threaded program need access to the same owned value, but mutations have borrows checked through `RefCell`.

```rust,edition2021
use std::cell::{BorrowMutError, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct Task {
    name: String,
    done: bool,
    dependencies: Vec<Rc<RefCell<Task>>>,
}

fn new_task(name: &str) -> Rc<RefCell<Task>> {
    Rc::new(RefCell::new(Task {
        name: name.to_owned(),
        done: false,
        dependencies: Vec::new(),
    }))
}

fn main() -> Result<(), BorrowMutError> {
    let generate_api_contract = new_task("Generate API contract");
    let implement_endpoint_1 = new_task("Implement endpoint 1 from API contract");
    let implement_endpoint_2 = new_task("Implement endpoint 2 from API contract");
    
    {
        let mut borrowed_task_1 = implement_endpoint_1.try_borrow_mut()?;
        borrowed_task_1.dependencies.push(Rc::clone(&generate_api_contract));

        let mut borrowed_task_2 = implement_endpoint_2.try_borrow_mut()?;
        borrowed_task_2.dependencies.push(Rc::clone(&generate_api_contract));
    
        generate_api_contract.borrow_mut().done = true;
    }

    assert!(implement_endpoint_1.borrow().dependencies[0].borrow().done);
    assert!(implement_endpoint_2.borrow().dependencies[0].borrow().done);

    println!("{:?}", implement_endpoint_1.borrow().dependencies);
    println!("{:?}", implement_endpoint_2.borrow().dependencies);
    Ok(())
}
```

[`Rc<T>`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`RefCell<T>`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`RefCell::try_borrow_mut`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow_mut
[`RefCell::borrow`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow
