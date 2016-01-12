pub unsafe fn inb(port: u16) -> u8 {
	let ret: u8;
	asm!("inb $1, $0" : "={al}"(ret) : "{dx}N"(port));
	return ret;
}

pub unsafe fn inw(port: u16) -> u16 {
	let ret: u16;
	asm!("inw $1, $0" : "={ax}"(ret) : "{dx}N"(port));
	return ret;
}

pub unsafe fn inl(port: u16) -> u32 {
	let ret: u32;
	asm!("inl $1, $0" : "={eax}"(ret) : "{dx}N"(port));
	return ret;
}

pub unsafe fn outb(port: u16, data: u8) {
	asm!("outb $0, $1" : : "{al}"(data), "{dx}N"(port));
}

pub unsafe fn outw(port: u16, data: u16) {
	asm!("outw $0, $1" : : "{ax}"(data), "{dx}N"(port));
}

pub unsafe fn outl(port: u16, data: u32) {
	asm!("outl $0, $1" : : "{eax}"(data), "{dx}N"(port));
}