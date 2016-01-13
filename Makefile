arch ?= x86_64
target ?= $(arch)-unknown-linux-gnu
sora := target/$(target)/debug/libsora.a
kernel := build/kernel-$(arch).bin
iso := build/sora-$(arch).iso
librustc_unicode := build/librustc_unicode.rlib

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.s)
assembly_object_files := $(patsubst src/arch/$(arch)/%.s, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -no-reboot -serial stdio -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	@cp $(kernel) build/iso/boot/kernel.bin
	@cp $(grub_cfg) build/iso/boot/grub
	@grub-mkrescue -o $(iso) build/iso 2> /dev/null
	@rm -r build/iso

$(kernel): cargo $(sora) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(sora)

cargo:
	@cargo rustc --target $(target) --  -Z no-landing-pads

build/arch/$(arch)/%.o: src/arch/$(arch)/%.s
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@
