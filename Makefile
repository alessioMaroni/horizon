ALR ?= .bin/alr-fhs
OVMF_PATH ?= /usr/share/edk2/ovmf/OVMF_CODE.fd

.PHONY: setup build-ada build debug test run run-debug run-test clean

build: build-ada
	mkdir -p .build
	cargo build --target riscv32imac-unknown-none-elf --release
	cargo objcopy --target riscv32imac-unknown-none-elf --release -- -O binary .build/kernel.bin
	python3 scripts/uf2conv.py .build/kernel.bin -f 0xe48bff56 -o .build/kernel.uf2

build-ada:
	echo "Buildin Ada......."

	mkdir -p ada/time/obj
	.bin/alr-fhs -C ada/time exec -- riscv64-elf-gcc -c src/time.adb \
		--RTS=$(PWD)/ada/rts \
		-gnatg \
		-gnatyN \
		-O2 \
		-g0 \
		-gnatp \
		-fno-PIC \
		-march=rv32imac \
		-mabi=ilp32 \
		-o obj/time.o
	echo "Done!"

debug:
	mkdir -p .build
	cargo build --target riscv32imac-unknown-none-elf
	cargo objcopy --target riscv32imac-unknown-none-elf -- -O binary .build/kernel-debug.bin

test:
	mkdir -p .build
	cargo build --target riscv32imac-unknown-none-elf --release --features tests
	cargo objcopy --target riscv32imac-unknown-none-elf --release --features tests --bin RSC-V-kernel -- -O binary .build/kernel-test.bin

run: build
	echo "Running......."

	qemu-system-riscv32 -M virt -bios none -nographic \
		-kernel target/riscv32imac-unknown-none-elf/release/RSC-V-kernel

	echo "\n\n"

run-debug: debug
	qemu-system-riscv32 -M virt -bios none -nographic -kernel .build/kernel-debug.bin

run-test: test
	qemu-system-riscv32 -M virt -bios none -nographic -kernel .build/kernel-test.bin

setup:
	CC=gcc cargo install probe-rs-tools --force

clean:
	cargo clean
	rm -rf .build ada/time/obj ada/memory/obj