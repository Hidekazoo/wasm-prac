#[allow(warnings)]
mod bindings;
use bindings::Guest;


struct Component;

impl bindings::exports::wasi::cli::run::Guest for Component {
    /// This function is exported to the component.
    fn run() -> Result<(),()> {
        println!("Hello, World! from run");
        Ok(())
    }
}

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
