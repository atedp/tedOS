#![feature(panic_implementation)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate ted_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

use ted_os::exit_qemu;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn _start() -> ! {
    ted_os::gdt::init();
    init_idt();

    fn stack_overflow() {
        stack_overflow();
    }

    //trigger stack overflow here
    stack_overflow();

    serial_println!("failed");
    serial_println!("no exception occured");

    unsafe {
        exit_qemu();
    }

    loop{}
}

//only called on panic
#[cfg(not(test))]
#[no_mangle]
#[panic_implementation]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }

    loop{}
}

use x86_64::structures::idt::{ExceptionStackFrame, Idt};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(ted_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut ExceptionStackFrame, _error_code: u64)
    {
        serial_println!("ok");

        unsafe {
            exit_qemu();
        }
        loop {}
    }
