#####
## BUILD
#####
CC=riscv64-unknown-elf-gcc
CFLAGS=-Wall -Wextra -pedantic -Wextra -O0 -g
CFLAGS+=-static -ffreestanding -nostdlib -fno-rtti -fno-exceptions
CFLAGS+=-march=rv64gc -mabi=lp64
INCLUDES=
LINKER_SCRIPT=-Tsrc/lds/virt.lds
TYPE=debug
RUST_TARGET=./target/riscv64gc-unknown-none-elf/$(TYPE)
TARGET=riscv64gc-unknown-none-elf
KERNEL_ELF=$(RUST_TARGET)/tisuos
LIBS=-L$(RUST_TARGET)
SOURCES_ASM=$(wildcard src/asm/*.S)
LIB=-l tisuos -lgcc
OUT=os.elf

#####
## QEMU
#####
QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=4
MEM=512M
DISK=img
DEVICE = -device virtio-tablet-device -device virtio-keyboard-device\
# -drive if=none,format=raw,file=$(DISK),id=fo1 -device virtio-blk-device,scsi=off,drive=fo1

NET_DEVICE = -device virtio-net-device
GPU_DEVICE = -device virtio-gpu-device

build: env
	RUSTFLAGS='-Clink-arg=-Tsrc/lds/virt.lds' cargo +nightly build --target=riscv64gc-unknown-none-elf

build_opensbi: env
	RUSTFLAGS='-Clink-arg=-Tsrc/lds/opensbi.lds' cargo +nightly build --target=riscv64gc-unknown-none-elf --no-default-features --features qemu_opensbi
# $(CC) $(CFLAGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB)

$(DISK):
#	@cd ../user_lib && make build

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	rustup override set nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview

run: build $(DISK)
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) $(DEVICE) $(NET_DEVICE) \
	-nographic -serial mon:stdio -bios none -kernel $(KERNEL_ELF) -rtc base=localtime

run_opensbi: build_opensbi
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) $(DEVICE) $(NET_DEVICE) \
	-nographic -serial mon:stdio -bios bins/fw_jump.bin -kernel $(KERNEL_ELF) -rtc base=localtime

debug_opensbi: build_opensbi $(DISK) objdump
	$(QEMU) -s -S -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) $(DEVICE) \
	-nographic -serial mon:stdio -bios bins/fw_jump.bin -kernel $(KERNEL_ELF) -rtc base=localtime

debug: build $(DISK) objdump
	$(QEMU) -s -S -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) $(DEVICE) \
	-nographic -serial mon:stdio -bios none -kernel $(KERNEL_ELF) -rtc base=localtime

graphic: build
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) $(DEVICE) $(GPU_DEVICE) \
	-serial mon:stdio -bios none -kernel $(KERNEL_ELF)

gdb:
	@echo file $(KERNEL_ELF)
	@echo target remote localhost:1234
	riscv64-unknown-elf-gdb

objdump:
	riscv64-unknown-elf-objdump -d target/riscv64gc-unknown-none-elf/debug/tisuos > os.txt

.PHONY: clean
clean:
	cargo clean
