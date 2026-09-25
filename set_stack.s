.section .text._start
.global _start

_start:
    // Set stack pointer at the end of 520K of SRAM (0x20000000 + 520 * 1024 = 0x20080000)
    li sp, 0x20080000

    // Jump to the main function in rust
    tail _main
