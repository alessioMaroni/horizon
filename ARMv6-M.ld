/* ARMv6-M linker script */

ENTRY(_start)

MEMORY
{
    REGION_TEXT   (rx)  : ORIGIN = 0x00000000, LENGTH = 256K
    REGION_RODATA (r)   : ORIGIN = 0x00040000, LENGTH = 64K
    REGION_DATA   (rw)  : ORIGIN = 0x00050000, LENGTH = 64K
}

SECTIONS
{
    .text : ALIGN(4K) 
     {
        *(.vectors)
        *(.text)
        *(.text.*)
    } > REGION_TEXT

    .rodata : ALIGN(4K) 
     {
        *(.rodata)
        *(.rodata.*)
        rodata_end = .;
    } > REGION_RODATA

    .data : AT(rodata_end) ALIGN(4K) 
     {
        data_start = .;
        *(.data)
        *(.data.*)
    } > REGION_DATA
}
