use spin::Mutex;

use ::terminal::Writer;
use ::arch::io::*;

const PORT: u16 = 0x3F8;

pub unsafe fn setup() {
	outb(PORT + 1, 0x00);
	outb(PORT + 3, 0x80);
	outb(PORT + 0, 0x03);
	outb(PORT + 1, 0x00);
	outb(PORT + 3, 0x03);
	outb(PORT + 2, 0xC7);
	outb(PORT + 4, 0x0B);
}

#[inline]
unsafe fn is_signal_received() -> bool {
	(inb(PORT + 5) & 1) != 0
}

#[inline]
unsafe fn is_transmit_empty() -> bool {
	(inb(PORT + 5) & 0x20) != 0
}

unsafe fn receive_byte() -> u8 {
	while !is_signal_received() {} // Wait until something comes on the serial port
	inb(PORT)
}

unsafe fn send_byte(data: u8) {
	while !is_transmit_empty() {} // Wait for transmit to be ready
	outb(PORT, data);
}

unsafe fn send_string(data: &str) {
	for byte in data.bytes() {
		send_byte(byte);
	}
}

pub struct SerialWriter {}

impl SerialWriter {
	pub fn new() -> SerialWriter {
		SerialWriter {}
	}

	pub fn setup(&mut self) {
		unsafe { setup() }
	}
}

impl Writer for SerialWriter {
	fn write_string(&mut self, string: &str) {
		unsafe { send_string(string); }
	}

	fn write_character(&mut self, character: u8) {
		unsafe { send_byte(character); }
	}

	fn new_line(&mut self) {
		unsafe { send_byte(0x0A); }
	}

	fn clear_screen(&mut self) {
		unsafe { send_byte(0x0C); }
	}

	fn scroll(&mut self) {
		// Do nothing
	}
}

pub static WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter {});

impl ::core::fmt::Write for SerialWriter {
	fn write_str(&mut self, string: &str) -> ::core::fmt::Result {
		for byte in string.bytes() {
			self.write_character(byte);
		}
		Ok(())
	}
}