//! Rust Hello world sample.

use kernel::prelude::*;

module! {
    type: HelloWorld,
    name: "hello_world",
    license: "GPL",
}

struct HelloWorld;

impl kernel::Module for HelloWorld {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world!\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        Ok(HelloWorld)
    }
}

impl Drop for HelloWorld {
    fn drop(&mut self) {
        pr_info!("Hello world (exit)\n");
    }
}
