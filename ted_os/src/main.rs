#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![no_std]
#![feature(panic_implementation)]
#![cfg_attr(not(test), no_main)]

// adding my new library as a dependancy 
#[macro_use]
extern crate ted_os;

use core::panic::PanicInfo;

// entry point
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop{}
}

// called on panic
#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    loop{}
}