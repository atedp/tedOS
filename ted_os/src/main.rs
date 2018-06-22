#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![no_std]
#![feature(panic_implementation)]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

extern crate volatile;
extern crate spin;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate array_init;

#[macro_use]
mod vga_buffer;

#[macro_use]
mod serial;

#[macro_use]
extern crate lazy_static;

#[cfg(not(test))]   //only compiles when the test flag is not set
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    serial_println!("Hello Host{}", "!");

    unsafe { exit_qemu(); }

    loop{}
}


