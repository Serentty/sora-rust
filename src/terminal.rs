pub trait Writer {
	fn write_string(&mut self, string: &str);
	fn write_character(&mut self, character: u8);
	fn new_line(&mut self);
	fn clear_screen(&mut self);
	fn scroll(&mut self);
}