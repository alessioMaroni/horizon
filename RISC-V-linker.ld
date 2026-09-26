ENTRY(_start)

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x80000000, LENGTH = 16M
    RAM   (rwx) : ORIGIN = 0x81000000, LENGTH = 16M
}

SECTIONS
{
    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    } > FLASH

    .rodata : {
        *(.rodata .rodata.*)
    } > FLASH

    .data : {
        *(.data .data.*)
    } > RAM AT > FLASH

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } > RAM
}