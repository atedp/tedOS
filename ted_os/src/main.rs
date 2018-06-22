#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate volatile;
extern crate spin;

#[macro_use]
extern crate lazy_static;

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[macro_use]
mod vga_buffer;

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    loop{}
}


