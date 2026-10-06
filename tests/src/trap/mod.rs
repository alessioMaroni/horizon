//! # Exception Trigger Test Suite
//!
//! **WARNING: FOR TESTING PURPOSES ONLY.**
//!
//! This module provides helper routines designed strictly to invoke hardware-level 
//! RISC-V exceptions (cause codes `0x0` through `0xB`). 
//!
//! Each function executes raw inline assembly (`asm!`) to force a specific hardware 
//! trap condition. These triggers are used to validate the correct behavior, register 
//! state saving, and recovering mechanisms of the kernel's trap handling routines.

unsafe extern "C" {
    static _stack_guard_start: u8;
}

/// Triggers Exception `0x0`: **Instruction Address Misaligned**.
///
/// Forces an unaligned instruction fetch by executing a jump (`jr`) to an unaligned 
/// memory address (`0x2000_0001`).
#[inline(always)]
pub fn trigger_0x0_inst_misaligned() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20000001",
            "jr t0",
            out("t0") _
        );
    }
}

/// Triggers Exception `0x1`: **Instruction Access Fault**.
///
/// Forces a execution access fault by jumping (`jr`) to an unmapped or restricted 
/// memory address (`0x4000_0000`).
#[inline(always)]
pub fn trigger_0x1_inst_access_fault() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x40000000",
            "jr t0",
            out("t0") _
        );
    }
}

/// Triggers Exception `0x2`: **Illegal Instruction**.
///
/// Emits an invalid 32-bit opcode (`0x00000000`), forcing the CPU decoder to fault.
#[inline(always)]
pub fn trigger_0x2_illegal_instruction() {
    unsafe {
        core::arch::asm!(".4byte 0x00000000");
    }
}

/// Triggers Exception `0x3`: **Breakpoint**.
///
/// Executes the hardware `ebreak` instruction to trigger a software breakpoint exception.
#[inline(always)]
pub fn trigger_0x3_breakpoint() {
    unsafe {
        core::arch::asm!("ebreak");
    }
}

/// Triggers Exception `0x4`: **Load Address Misaligned**.
///
/// Attempts to load a 32-bit word (`lw`) from an unaligned memory address (`0x2000_0001`).
#[inline(always)]
pub fn trigger_0x4_load_misaligned() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20000001",
            "lw t1, 0(t0)",
            out("t0") _,
            out("t1") _
        );
    }
}

/// Triggers Exception `0x5`: **Load Access Fault**.
///
/// Attempts to load a 32-bit word (`lw`) from an invalid or protected memory address (`0x0000_0000`).
#[inline(always)]
pub fn trigger_0x5_load_access_fault() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x00000000",
            "lw t1, 0(t0)",
            out("t0") _,
            out("t1") _
        );
    }
}

/// Triggers Exception `0x6`: **Store/AMO Address Misaligned**.
///
/// Attempts to store a 32-bit word (`sw`) to an unaligned memory address (`0x2000_0003`).
#[inline(always)]
pub fn trigger_0x6_store_misaligned() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20000003",
            "sw x0, 0(t0)",
            out("t0") _
        );
    }
}

/// Triggers Exception `0x7`: **Store/AMO Access Fault**.
///
/// Attempts to store a 32-bit word (`sw`) to a NULL/unmapped address (`0x0000_0000`).
/*
#[inline(always)]
pub fn trigger_0x7_store_access_fault() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x00000000",
            "sw x0, 0(t0)",
            out("t0") _
        );
    }
}
*/

/// Triggers Exception `0x7`: **Store/AMO Access Fault**.
///
/// Attempts to store a 32-bit word (`sw`) directly into the hardware-locked 
/// PMP Guard Page (`_stack_guard_start`), violating the `pmp0cfg` rule (`0x98`: R=0, W=0, X=0, L=1).
#[inline(always)]
pub fn trigger_0x7_store_access_fault() {
    unsafe {
        let guard_ptr = core::ptr::addr_of!(_stack_guard_start) as *mut u32;
        core::arch::asm!(
            "sw x0, 0({addr})",
            addr = in(reg) guard_ptr,
        );
    }
}

/// Triggers Exception `0x8`: **Environment Call from U-mode**.
///
/// Issues an `ecall` instruction while operating in User Mode.
#[inline(always)]
pub fn trigger_0x8_syscall_umode() {
    unsafe {
        core::arch::asm!("ecall");
    }
}

/// Triggers Exception `0xB`: **Environment Call from M-mode**.
///
/// Issues an `ecall` instruction while operating in Machine Mode.
#[inline(always)]
pub fn trigger_0xb_syscall_mmode() {
    unsafe {
        core::arch::asm!("ecall");
    }
}