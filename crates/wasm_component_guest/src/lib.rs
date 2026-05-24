wit_bindgen::generate!({ world: "greeter" });

struct Component;

impl Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello, {name}!")
    }
}

export!(Component);
