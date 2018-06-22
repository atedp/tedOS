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

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), "Some numbers: {} {}", 42, 1.337).unwrap();

    loop{}
}


