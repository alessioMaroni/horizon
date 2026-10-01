ENTRY(_start)

MEMORY
{
    FLASH   (rx)  : ORIGIN = 0x80000000, LENGTH = 2M
    RAM     (rwx) : ORIGIN = 0x80200000, LENGTH = 512K
    SCRATCH (rwx) : ORIGIN = 0x80280000, LENGTH = 8K
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
}
