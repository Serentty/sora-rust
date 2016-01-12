#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(braced_empty_structs)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
#[cfg(target_arch="x86_64")] #[path="arch/x86_64/mod.rs"]
pub mod arch;

mod terminal;
use terminal::Writer;

use core::fmt::Write;

#[no_mangle]
pub extern fn kernel_main() {
	arch::vga::WRITER.lock().clear_screen();
	arch::serial::WRITER.lock().setup();
	arch::serial::WRITER.lock().clear_screen();
	println!("Hello, world!");
	printlns!("Hello, serial!");
	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

