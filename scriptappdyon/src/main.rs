

#[macro_use]
extern crate dyon;

use std::sync::Arc;
//use dyon::{error, run};

//static mut count:i32 = 0;
static SOME_STR: &'static str = "A static string";
static mut N: i32 = 5;

//https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust
//https://doc.rust-lang.org/1.30.0/book/first-edition/const-and-static.html
fn main() {
    //error(run("main.dyon"));


    println!("{}", SOME_STR);

    unsafe {
        N += 1;
    
        println!("N: {}", N);
    }


    //let mut count = 0;
    //count = 0;
    //println!("{}", count);
    //count = 1;
    //println!("{}", count);

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
    module.add_str("count_display", count_display, Dfn::nl(vec![], Void));
    module.add_str("count_add", count_add, Dfn::nl(vec![], Void));
    module.add_str("count_test", count_test, Dfn::nl(vec![], Void));

    if error(load("loader.dyon", &mut module)) {
        None
    } else {
        Some(module)
    }
}

dyon_fn!{fn say_hello() {
    println!("hi! dyon fn!");
}}

dyon_fn!{fn count_display() {
    unsafe {
        println!("N: {}", N);
    }
}}

dyon_fn!{fn count_add() {
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}}

dyon_fn!{fn count_test() {
    unsafe {
        println!("test N: {}", N);
    }
}}