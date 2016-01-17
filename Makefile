arch ?= x86_64
target ?= $(arch)-unknown-sora-gnu
target_json ?= src/arch/$(arch)/$(arch)-unknown-sora-gnu.json
sora := target/$(target)/debug/libsora.a
kernel := build/kernel-$(arch).bin
iso := build/sora-$(arch).iso
sysroot_libs := sysroot/lib/rustlib/$(target)/lib

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.s)
assembly_object_files := $(patsubst src/arch/$(arch)/%.s, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso patch

all: $(kernel)

clean:
	@rm -rf target sysroot stdlib/libcore .patched

run: $(iso)
	qemu-system-x86_64 -no-reboot -serial stdio -cdrom $(iso)

iso: $(iso)

.patched: stdlib/libcore_nofp.patch
	@cp -rf stdlib/libcore-unpatched stdlib/libcore
	@cd stdlib; patch -p0 < libcore_nofp.patch
	touch .patched

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	@cp $(kernel) build/iso/boot/kernel.bin
	@cp $(grub_cfg) build/iso/boot/grub
	grub-mkrescue -d /usr/lib/grub/i386-pc/ -o $(iso) build/iso 2> /dev/null
	@rm -r build/iso

$(kernel): cargo $(sora) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(sora)

cargo: $(sysroot_libs)/libcore.rlib
	cargo rustc --verbose --target=$(target_json)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.s
	@mkdir -p $(shell dirname $@)
	nasm -f elf64 $< -o $@

stdlib/libcore/src/lib.rs: .patched

$(sysroot_libs)/libcore.rlib: stdlib/libcore/src/lib.rs
	@mkdir -p $(sysroot_libs)
	rustc -o $@ $< --target=$(target_json) --cfg disable_float