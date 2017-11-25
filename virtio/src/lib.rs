#![no_std]

#[macro_use]
extern crate solo5;

use solo5::alloc::boxed::Box;
use core::mem;

#[no_mangle]
pub fn rust_main(cmdline : &str) -> isize{
    println!("hello");
    println!("cmdline: {}",  cmdline);
    
    let p = Box::new(123 as u32);
    
    println!("boxed value: {}",  p);
    unsafe {
        println!("box addr: {:?}",  mem::transmute ::<Box<u32>, *const u8>(p) );
        println!("console addr: {:?}",  mem::transmute ::<_, *const u8>(&solo5::CONSOLE) );
    }

    return 1;
}

