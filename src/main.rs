#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
	loop {}
}




#[no_mangle]
#[macro_use]
mod vga_buffer;
pub extern "C" fn _start() -> ! {
	println!("Hello World{}","!");	
	loop{}
}

extern crate volatile;

#[macro_use]
extern crate lazy_static;
extern crate spin;
