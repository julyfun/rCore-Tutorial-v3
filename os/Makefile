# Variables
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := ../target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := ../bootloader/rustsbi-qemu.bin

# QEMU
QEMU := qemu-system-riscv64
QEMU_ARGS := -machine virt \
	-nographic \
	-bios $(BOOTLOADER) \
	-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
	-s -S

# Building targets
.PHONY: run build clean FORCE

run: $(KERNEL_BIN)
	$(QEMU) $(QEMU_ARGS)

build: $(KERNEL_BIN)

$(KERNEL_BIN): $(KERNEL_ELF)
	rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

$(KERNEL_ELF): FORCE
	cargo build --release

clean:
	cargo clean