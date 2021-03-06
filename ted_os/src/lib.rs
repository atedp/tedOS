#![feature(const_fn)]
#![no_std]
#![feature(abi_x86_interrupt)]

extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;


#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod vga_buffer;
pub mod serial;
pub mod gdt;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[cfg(not(test))]
#[cfg(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!!");

    println!("It did not crash!");
    loop{}
}