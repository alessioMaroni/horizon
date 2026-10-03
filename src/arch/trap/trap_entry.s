// This file saves the general-purpose registers and invokes `trap_handler` function.
.global trap_entry

// Aligning at 4 bytes (the pointer written to mtvec must be word-aligned)
.align 4

trap_entry:
    // Making space on the stack to store all the 31 registers
    // 31 (n. regs) * 4 (size) = 124, rounded to 128 for 16-byte stack alignment
    addi sp, sp, -128

    // Saving all the 31 regs on the stack (x1 to x31, excluding x0)
    sw x1,   0(sp)   // ra  (Return Address)
    sw x2,   4(sp)   // sp  (Original Stack Pointer)
    sw x3,   8(sp)   // gp
    sw x4,  12(sp)   // tp
    sw x5,  16(sp)   // t0
    sw x6,  20(sp)   // t1
    sw x7,  24(sp)   // t2
    sw x8,  28(sp)   // s0 / fp
    sw x9,  32(sp)   // s1
    sw x10, 36(sp)   // a0
    sw x11, 40(sp)   // a1
    sw x12, 44(sp)   // a2
    sw x13, 48(sp)   // a3
    sw x14, 52(sp)   // a4
    sw x15, 56(sp)   // a5
    sw x16, 60(sp)   // a6
    sw x17, 64(sp)   // a7
    sw x18, 68(sp)   // s2
    sw x19, 72(sp)   // s3
    sw x20, 76(sp)   // s4
    sw x21, 80(sp)   // s5
    sw x22, 84(sp)   // s6
    sw x23, 88(sp)   // s7
    sw x24, 92(sp)   // s8
    sw x25, 96(sp)   // s9
    sw x26, 100(sp)  // s10
    sw x27, 104(sp)  // s11
    sw x28, 108(sp)  // t3
    sw x29, 112(sp)  // t4
    sw x30, 116(sp)  // t5
    sw x31, 120(sp)  // t6
    // Last 4 bytes (124-128) are used as padding for stack alignment

    // Preparing function params
    csrr a0, mcause
    csrr a1, mepc
    mv   a2, sp

    // Calling Rust function defined in src/trap.rs
    call trap_handler

    // Exiting phase: Restoring all 31 registers with EXACT matching offsets
    lw x1,   0(sp)
    lw x2,   4(sp)
    lw x3,   8(sp)
    lw x4,  12(sp)
    lw x5,  16(sp)
    lw x6,  20(sp)
    lw x7,  24(sp)
    lw x8,  28(sp)
    lw x9,  32(sp)
    lw x10, 36(sp)
    lw x11, 40(sp)
    lw x12, 44(sp)
    lw x13, 48(sp)
    lw x14, 52(sp)
    lw x15, 56(sp)
    lw x16, 60(sp)
    lw x17, 64(sp)
    lw x18, 68(sp)
    lw x19, 72(sp)
    lw x20, 76(sp)
    lw x21, 80(sp)
    lw x22, 84(sp)
    lw x23, 88(sp)
    lw x24, 92(sp)
    lw x25, 96(sp)
    lw x26, 100(sp)
    lw x27, 104(sp)
    lw x28, 108(sp)
    lw x29, 112(sp)
    lw x30, 116(sp)
    lw x31, 120(sp)

    // Reset stack pointer
    addi sp, sp, 128

    mret
