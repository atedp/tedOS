#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![no_std]
#![feature(panic_implementation)]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

extern crate volatile;
extern crate spin;

#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate array_init;

#[macro_use]
mod vga_buffer;

#[macro_use]
extern crate lazy_static;

#[cfg(not(test))]   //only compiles when the test flag is not set
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop{}
}


