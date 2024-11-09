#![no_std]
#![cfg_attr(target_os = "linux", no_main)]

use noli::prelude::*;

fn main() {
    Api::write_string("Hello, world!\n");
    println!("Hello from println!");
    Api::exit(42);
}

entry_point!(main);
