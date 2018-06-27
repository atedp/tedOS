#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![no_std]
#![feature(panic_implementation)]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]

// adding my new library as a dependancy 
#[macro_use]
extern crate ted_os;

extern crate x86_64;
use x86_64::structures::idt::{Idt, ExceptionStackFrame};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

use core::panic::PanicInfo;

// entry point
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    init_idt();

    //invoke a breakpoint exception 
    x86_64::instructions::int3();

    println!("It did not crash!{}", "");
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