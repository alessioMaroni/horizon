/* TODO: Document */

ENTRY(_start)

MEMORY
{
    FLASH   (rx)  : ORIGIN = 0x80000000, LENGTH = 2M
    RAM     (rwx) : ORIGIN = 0x80200000, LENGTH = 512K
    SCRATCH (rwx) : ORIGIN = 0x80280000, LENGTH = 8K
    UART    (rw)  : ORIGIN = 0x10000000, LENGTH = 4K
}

SECTIONS
{
    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > FLASH

    _sidata = LOADADDR(.data);

    .data : {
        . = ALIGN(4);
        _sdata = .;
        *(.data .data.*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH

    .bss : {
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;

        _heap_start = .;
    } > RAM

    _heap_end = ORIGIN(RAM) + LENGTH(RAM);

    .uart (NOLOAD) : {
        _uart_start = .;
        *(.uart .uart.*)
    } > UART

    .stack_guard (NOLOAD) : ALIGN(512)
    {
        _stack_guard_start = .;
        . += 512;
        _stack_guard_end = .;
    }

    .stack (NOLOAD) : ALIGN(16)
    {
        _stack_start = .;
        . += 8192;
        _stack_end = .;
    }
}
