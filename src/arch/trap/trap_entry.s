// File: trap_entry.s
// Architecture: RISC-V (RV32I / RV32IMAC - Machine Mode)
// Description: Main trap entry point for exception and interrupt handling.
//              Saves full CPU context, prepares parameters for the high-level
//              handler (`trap_handler`), restores context, and returns from trap.

// Export `trap_entry` as a global symbol so it can be referenced in Rust/C
// (e.g., when setting the Machine Trap-Vector Base-Address Register `mtvec`).
.global trap_entry

// Ensure 4-byte (word) alignment.
// RISC-V requires the base address in `mtvec` to be at least 4-byte aligned.
.align 4

trap_entry:
    // Expand macro to allocate 128 bytes on the stack and store all integer
    // registers (x1 - x31) into the newly created stack frame.
    SAVE_REGS

    // Argument 1 (a0): Read `mcause` CSR (Machine Cause Register)
    // Contains the trap reason (interrupt vs. exception and exact cause code).
    csrr a0, mcause

    // Argument 2 (a1): Read `mepc` CSR (Machine Exception Program Counter)
    // Contains the virtual/physical memory address of the trapped instruction.
    csrr a1, mepc

    // Argument 3 (a2): Pass current Stack Pointer (`sp`)
    // Points to the base of the saved register structure (`TrapFrame`) on stack.
    mv   a2, sp

    // Invoke the higher-level trap dispatcher defined in Rust/C:
    // `fn trap_handler(mcause: usize, mepc: usize, frame: *mut TrapFrame)`
    call trap_handler

    // Expand macro to reload all registers (x1 - x31) from the stack frame
    // and reclaim the 128 bytes of allocated stack space.
    RESTORE_REGS

    // Return from Machine-mode trap handler:
    // - Sets Program Counter (PC) to the address currently in `mepc`.
    // - Restores interrupt enable state (copies MPIE to MIE in `mstatus`).
    // - Restores previous privilege level (MPP in `mstatus`).
    mret