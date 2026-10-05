// File: save_regs.s
// Architecture: RISC-V (RV32I / RV32IMAC)
// Description: Macro definition for saving all general-purpose registers (GPRs)
//              onto the stack frame upon entering a trap, exception, or interrupt.

.macro SAVE_REGS
    // Stack Frame Allocation
    // Allocate 128 bytes on the stack by moving the Stack Pointer (sp) down.
    // 128 bytes accounts for 31 integer registers (124 bytes) plus 4 bytes 
    // of alignment padding to maintain mandatory 16-byte stack alignment.
    addi sp, sp, -128

    // Save General-Purpose Registers (x1 - x31) to Stack
    sw x1,   0(sp)    // ra
    sw x2,   4(sp)    // sp
    sw x3,   8(sp)    // gp
    sw x4,  12(sp)    // tp
    sw x5,  16(sp)    // t0
    sw x6,  20(sp)    // t1
    sw x7,  24(sp)    // t2
    sw x8,  28(sp)    // s0/fp
    sw x9,  32(sp)    // s1
    sw x10, 36(sp)    // a0
    sw x11, 40(sp)    // a1
    sw x12, 44(sp)    // a2
    sw x13, 48(sp)    // a3
    sw x14, 52(sp)    // a4
    sw x15, 56(sp)    // a5
    sw x16, 60(sp)    // a6
    sw x17, 64(sp)    // a7
    sw x18, 68(sp)    // s2
    sw x19, 72(sp)    // s3
    sw x20, 76(sp)    // s4
    sw x21, 80(sp)    // s5
    sw x22, 84(sp)    // s6
    sw x23, 88(sp)    // s7
    sw x24, 92(sp)    // s8
    sw x25, 96(sp)    // s9
    sw x26, 100(sp)   // s10
    sw x27, 104(sp)   // s11
    sw x28, 108(sp)   // t3
    sw x29, 112(sp)   // t4
    sw x30, 116(sp)   // t5
    sw x31, 120(sp)   // t6

    // Offsets 124-127 (4 bytes) remain unwritten as alignment padding.
.endm