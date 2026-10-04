OVMF_PATH ?= /usr/share/edk2/ovmf/OVMF_CODE.fd
TARGET-X86_64 := x86_64-unknown-uefi
TARGET-ARMv6-M := thumbv6m-none-eabi

.PHONY: build-ada build-x86_64 build run clean

setup:
	CC=gcc cargo install probe-rs-tools --force

build-ada:
	mkdir -p ada/time/obj ada/memory/obj
	gcc -c ada/time/src/time.adb -O2 -g0 -gnatp -mno-red-zone -fno-PIC -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-strict-aliasing -o ada/time/obj/time.o

	objcopy -I elf64-x86-64 -O pe-x86-64 ada/time/obj/time.o ada/time/obj/time.obj

build:
	mkdir -p .build
	cargo build --target riscv32imac-unknown-none-elf --release
	cargo objcopy --target riscv32imac-unknown-none-elf --release -- -O binary .build/kernel.bin
	python3 scripts/uf2conv.py .build/kernel.bin -f 0xe48bff56 -o .build/kernel.uf2

debug:
	mkdir -p .build
	cargo build --target riscv32imac-unknown-none-elf
	cargo objcopy --target riscv32imac-unknown-none-elf -- -O binary .build/kernel-debug.bin

test:
	mkdir -p .build
	cargo build --target riscv32imac-unknown-none-elf --release --features tests
	cargo objcopy --target riscv32imac-unknown-none-elf --release --features tests --bin RSC-V-kernel -- -O binary .build/kernel-test.bin

run: build
	qemu-system-riscv32 -M virt -bios none -nographic -kernel .build/kernel.bin

run-debug: debug
	qemu-system-riscv32 -M virt -bios none -nographic -kernel .build/kernel-debug.bin

run-test: test
	qemu-system-riscv32 -M virt -bios none -nographic -kernel .build/kernel-test.bin

clean:
	cargo clean
	rm -rf .build ada/time/obj ada/memory/obj
