#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //run tests
    loop{}
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}