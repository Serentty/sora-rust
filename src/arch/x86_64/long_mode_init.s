global long_mode_start

section .text
extern kernel_main
bits 64
long_mode_start:
	call kernel_main
	mov rax, 0x4f724f204f534f4f
	mov qword [0xb8000], rax
	mov rax, 0x4f724f754f744f65
	mov qword [0xb8008], rax
	mov rax, 0x4f214f644f654f6e
	mov qword [0xb8010], rax
	hlt
