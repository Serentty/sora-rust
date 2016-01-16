#![feature(lang_items)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(braced_empty_structs)]
#![feature(no_std)]
#![feature(collections)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate alloc;
extern crate collections;

extern crate gpmalloc;

#[macro_use]
#[cfg(target_arch="x86_64")] #[path="arch/x86_64/mod.rs"]
pub mod arch;

mod terminal;
use terminal::Writer;

#[macro_use]
mod stdlib_macros;

use core::fmt::Write;
use collections::*;

#[no_mangle]
pub extern fn kernel_main() {
	arch::vga::WRITER.lock().clear_screen();
	arch::serial::WRITER.lock().setup();
	let v = vec![1, 2, 3, 4];
	println!("Sora - now with vectors! Here's one:");
	for item in v {
		println!("{}\n", item)
	}
	loop {}
}


