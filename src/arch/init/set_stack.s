// Define the entry point section in the text segment
    .section .text._start
    // Export the _start symbol as the global entry point
    .global _start
    // Save current assembler options
    .option push
    // Disable linker relaxation to ensure correct absolute addressing for stack/symbols
    .option norelax

    _start:
        // Clear the MIE (Machine Interrupt Enable) bit in mstatus (bit 3, value 0x8)
        csrc mstatus, 0x8

        // Initialize the stack pointer (sp) to the top/end of the stack region
        la sp, _stack_end
        // Restore previous assembler options
        .option pop
        
        // Load start and end addresses of the BSS segment (.bss)
        la t0, _sbss
        la t1, _ebss
    1:
        // If t0 >= t1, the BSS zeroing is complete; jump to loop 2
        bgeu t0, t1, 2f
        // Store zero into the current word of the BSS section
        sw zero, 0(t0)
        // Advance the BSS pointer by 4 bytes (32-bit word)
        addi t0, t0, 4
        // Repeat the loop
        j 1b

    2:
        // Load source (ROM: _sidata) and destination (RAM: _sdata, _edata) for initialized data segment
        la t0, _sidata
        la t1, _sdata
        la t2, _edata
    3:
        // If destination pointer (t1) >= end address (t2), data copying is complete; jump to loop 4
        bgeu t1, t2, 4f
        // Load a word from the initialization source
        lw t3, 0(t0)
        // Store the word into the data RAM destination
        sw t3, 0(t1)
        // Advance source pointer by 4 bytes
        addi t0, t0, 4
        // Advance destination pointer by 4 bytes
        addi t1, t1, 4
        // Repeat the loop
        j 3b

    4:
        // Jump to the high-level system setup/entry point function
        j _setup