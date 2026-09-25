MEMORY
{
    FLASH (rx)  : ORIGIN = 0x10000000, LENGTH = 2M
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 520K
}

SECTIONS
{
    .text : {
        KEEP(*(.picobin_block))
        
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
