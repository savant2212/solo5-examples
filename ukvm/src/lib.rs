#![no_std]

#[macro_use]
extern crate solo5;

#[no_mangle]
pub fn rust_main(cmdline : &str) -> isize{
    println!("hello");
    println!("cmdline: {}",  cmdline);

    return 1;
}

