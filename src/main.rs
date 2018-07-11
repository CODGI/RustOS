#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
	loop {}
}


extern crate volatile;

#[macro_use]
extern crate lazy_static;
extern crate spin;

#[cfg(not(test))]
#[no_mangle]
#[macro_use]
mod vga_buffer;
pub extern "C" fn _start() -> ! {
	loop{}
}

