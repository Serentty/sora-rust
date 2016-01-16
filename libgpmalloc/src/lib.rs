#![feature(allocator)]
#![allocator]

#![feature(no_std)]
#![no_std]

#![feature(lang_items)]

#![crate_name = "gpmalloc"]
#![crate_type = "rlib"]

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
	0x20000 as *mut u8
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
	// No need for free() with this much RAM!
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
	unsafe { __rust_allocate(size, align) }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> usize {
	old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
	size
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }