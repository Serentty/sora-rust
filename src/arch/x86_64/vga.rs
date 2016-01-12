use spin::Mutex;

use ::terminal::Writer;

const VGA_WIDTH: u8 = 80;
const VGA_HEIGHT: u8 = 25;

#[repr(u8)]
pub enum Colour {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGrey  = 7,
    DarkGrey   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

const fn get_colour_code(foreground: Colour, background: Colour) -> u8 {
	(background as u8) << 4 | (foreground as u8)
}

const fn get_vga_entry(character: u8, colour_code: u8) -> u16 {
	(colour_code as u16) << 8 | (character as u16)
}

unsafe fn write_vga_entry(vga_entry: u16, x: u8, y: u8) {
	if (x > (VGA_WIDTH - 1)) | (y > (VGA_HEIGHT - 1)) {
		return; // We don't want to write to somewhere that isn't part of the VGA buffer!
	}

	*(((0xb8000 as usize) + (((y as usize) * (VGA_WIDTH as usize)) + (x as usize)) * 2) as *mut _) = vga_entry;
}

unsafe fn read_vga_entry(x: u8, y: u8) -> u16 {
	*(((0xb8000 as usize) + (((y as usize) * (VGA_WIDTH as usize)) + (x as usize)) * 2) as *mut _)
}

pub struct VGAWriter {
	cursor_x: u8,
	cursor_y: u8,
	cursor_colour: u8,
}

impl VGAWriter {
	pub fn new(foreground: Colour, background: Colour) -> VGAWriter {
		VGAWriter {
			cursor_x: 0,
			cursor_y: 0,
			cursor_colour: get_colour_code(foreground, background),
		}
	}
}

impl Writer for VGAWriter {
	fn write_string(&mut self, string: &str) {
		for byte in string.bytes() {
			self.write_character(byte);
		}
	}

	fn write_character(&mut self, character: u8) {
		if character == '\n' as u8 {
			self.new_line();
		}
		else {
			unsafe { write_vga_entry(get_vga_entry(character, self.cursor_colour), self.cursor_x, self.cursor_y); }
			self.cursor_x += 1;
			if self.cursor_x >= VGA_WIDTH {
				self.new_line();
			}
		}
	}

	fn new_line(&mut self) {
		self.cursor_x = 0;
		if self.cursor_y < VGA_HEIGHT {
			self.cursor_y += 1;
		}
		else {
			self.scroll();
		}
	}

	fn clear_screen(&mut self) {
		let blank_space = get_vga_entry(' ' as u8, self.cursor_colour);
		for i in 0..VGA_WIDTH {
			for j in 0..VGA_HEIGHT {
				unsafe { write_vga_entry(blank_space, i, j); }
			}
		}
	}

	fn scroll(&mut self) {
		let blank_space = get_vga_entry(' ' as u8, self.cursor_colour);
		for i in 0..VGA_WIDTH {
			for j in 0..(VGA_HEIGHT - 1) { // Skip the last row
				unsafe { write_vga_entry(read_vga_entry(i, j + 1), i, j); }
			}
		}

		let last_row = VGA_HEIGHT - 1;
		for i in 0..VGA_WIDTH { // Clear the last row
			unsafe { write_vga_entry(blank_space, i, last_row); }
		}
	}
}

pub static WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
	cursor_x: 0,
	cursor_y: 0,
	cursor_colour: get_colour_code(Colour::White, Colour::Blue),
});

impl ::core::fmt::Write for VGAWriter {
	fn write_str(&mut self, string: &str) -> ::core::fmt::Result {
		for byte in string.bytes() {
			self.write_character(byte);
		}
		Ok(())
	}
}