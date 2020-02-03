

#[macro_use]
extern crate dyon;

use std::sync::Arc;
//use dyon::{error, run};

fn main() {
    //error(run("main.dyon"));

    use dyon::{error, Runtime};
    let mut dyon_runtime = Runtime::new();
    let dyon_module = load_module().unwrap();
    if error(dyon_runtime.run(&Arc::new(dyon_module))) {
        return
    }
}

fn load_module() -> Option<dyon::Module> {
    use dyon::{error, load, Dfn, Module};
    use dyon::Type::*;

    let mut module = Module::new();
    module.add_str("say_hello", say_hello, Dfn::nl(vec![], Void));

    if error(load("loader.dyon", &mut module)) {
        None
    } else {
        Some(module)
    }
}

dyon_fn!{fn say_hello() {
    println!("hi! dyon fn!");
}}
