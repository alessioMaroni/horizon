.section .text._start
.global _start

_start:
    la sp, 0x82000000
    
    j _main