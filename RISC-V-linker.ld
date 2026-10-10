/* ========================================================================== */
/* RISC-V Kernel Linker Script                                */
/* Target: Bare-metal RV32IMAC (e.g., RP2350)                                  */
/* ========================================================================== */

/* Define the mandatory execution entry point of the kernel */
ENTRY(_start)

/* Define the physical memory layout and access attributes */
MEMORY
{
    FLASH   (rx)  : ORIGIN = 0x80000000, LENGTH = 2M     /* Non-volatile program memory (Read/Execute) */
    RAM     (rwx) : ORIGIN = 0x80200000, LENGTH = 512K   /* Primary system data memory (Read/Write/Execute) */
    SCRATCH (rwx) : ORIGIN = 0x80280000, LENGTH = 8K     /* Specialized scratchpad RAM */
    UART    (rw)  : ORIGIN = 0x10000000, LENGTH = 4K     /* Memory-mapped I/O region for UART peripherals */
}

/* Export symbolic references for machine timer control registers (CLINT subsystem) */
PROVIDE(__MTIME_ADDR__    = 0x0200BFF8);
PROVIDE(__MTIMECMP_ADDR__ = 0x02004000);

SECTIONS
{
    /* Set the initial location counter to the base of FLASH */
    . = 0x80000000;

    /* Code and Read-Only Data Section */
    .text : {
        KEEP(*(.text._start)) /* Force retention of the startup assembly routine */
        *(.text .text.*)      /* All kernel executable code */
        *(.rodata .rodata.*)  /* Read-only constants and strings */
        . = ALIGN(4);         /* Enforce 32-bit word alignment */
    } > FLASH

    /* Capture load address of the initialized data segment in ROM */
    _sidata = LOADADDR(.data);

    /* Initialized Data Section (LMA in FLASH, VMA in RAM) */
    .data : {
        . = ALIGN(4);
        _sdata = .;           /* Start symbol of data RAM destination */
        *(.data .data.*)      /* Initialized global/static variables */
        . = ALIGN(4);
        _edata = .;           /* End symbol of data RAM destination */
    } > RAM AT > FLASH

    /* Uninitialized Data (BSS) Section */
    .bss : {
        . = ALIGN(4);
        _sbss = .;            /* Start symbol of BSS segment (to be zeroed out) */
        *(.bss .bss.*)        /* Uninitialized global/static variables */
        *(COMMON)             /* Common symbols */
        . = ALIGN(4);
        _ebss = .;            /* End symbol of BSS segment */

        _heap_start = .;      /* Boundary marker (unused under static allocation policies) */
    } > RAM

    /* Total RAM boundary calculation reference */
    _heap_end = ORIGIN(RAM) + LENGTH(RAM);

    /* Memory-Mapped UART Section (No load, direct hardware mapping) */
    .uart (NOLOAD) : {
        _uart_start = .;
        *(.uart .uart.*)
    } > UART

    /* Hardware Stack Guard Region (Memory protection buffer against overflows) */
    .stack_guard (NOLOAD) : ALIGN(512)
    {
        _stack_guard_start = .;
        . += 512;             /* Allocate 512 bytes for stack overflow detection */
        _stack_guard_end = .;
    }

    /* Primary Execution Stack Section */
    .stack (NOLOAD) : ALIGN(16)
    {
        _stack_start = .;
        . += 8192;            /* Allocate 8 KB for the main kernel execution stack */
        _stack_end = .;       /* Top of the stack (stack pointer initializes here) */
    }

    /* Emergency/Fault Trap Stack Section (Isolated stack for handling critical traps/panics) */
    .emergency_stack (NOLOAD) : ALIGN(16)
    {
        _emergency_stack_bottom = .;
        . += 512;             /* Allocate 512 bytes for fault handling */
        _emergency_stack_top = .;
    }
}