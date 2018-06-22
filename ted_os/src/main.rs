#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[macro_use]
extern crate lazy_static;

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop{}
}


