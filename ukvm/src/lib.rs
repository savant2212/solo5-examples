#![no_std]

#[macro_use]
extern crate solo5;

use solo5::alloc::boxed::Box;
use core::mem;

static TEST : u64 = 0xB16B00B5;

#[no_mangle]
pub fn rust_main(cmdline : &str) -> isize{
    println!("hello");
    println!("cmdline: {}",  cmdline);

    println!("alloc first");
    let p = Box::new(123 as u32);
    println!("alloc second");
    let q = Box::new(456 as u32);
    
    println!("addresses:");
    println!("boxed value: {}",  p);
    println!("boxed value: {}",  q);
    unsafe {
        println!("box addr: {:?}",  mem::transmute ::<Box<u32>, *const u8>(p) );
        println!("box addr: {:?}",  mem::transmute ::<Box<u32>, *const u8>(q) );
        println!("console addr: {:?}",  mem::transmute ::<_, *const u8>(&solo5::CONSOLE) );
        println!("TEST addr: {:?}",  mem::transmute ::<_, *const u8>(&TEST) );
    }

    return 0;
}

