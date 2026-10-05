// File: restore_regs.s
// Architecture: RISC-V (RV32I / RV32IMAC)
// Description: Macro definition for restoring general-purpose registers (GPRs)
//              from the stack frame and restoring the original stack pointer.
.macro RESTORE_REGS
    // Restore General-Purpose Registers (x1 - x31) from Stack
    lw x1,   0(sp)    // ra
    lw x2,   4(sp)    // sp
    lw x3,   8(sp)    // gp
    lw x4,  12(sp)    // tp
    lw x5,  16(sp)    // t0
    lw x6,  20(sp)    // t1
    lw x7,  24(sp)    // t2
    lw x8,  28(sp)    // s0/fp
    lw x9,  32(sp)    // s1
    lw x10, 36(sp)    // a0
    lw x11, 40(sp)    // a1
    lw x12, 44(sp)    // a2
    lw x13, 48(sp)    // a3
    lw x14, 52(sp)    // a4
    lw x15, 56(sp)    // a5
    lw x16, 60(sp)    // a6
    lw x17, 64(sp)    // a7
    lw x18, 68(sp)    // s2
    lw x19, 72(sp)    // s3
    lw x20, 76(sp)    // s4
    lw x21, 80(sp)    // s5
    lw x22, 84(sp)    // s6
    lw x23, 88(sp)    // s7
    lw x24, 92(sp)    // s8
    lw x25, 96(sp)    // s9
    lw x26, 100(sp)   // s10
    lw x27, 104(sp)   // s11
    lw x28, 108(sp)   // t3
    lw x29, 112(sp)   // t4
    lw x30, 116(sp)   // t5
    lw x31, 120(sp)   // t6

    // Stack Frame Deallocation
    // Reclaim the 128 allocated bytes on the stack frame by advancing 'sp'
    addi sp, sp, 128
.endm