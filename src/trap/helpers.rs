//! # Trap Handling Helper Module
//!
//! Provides utility functions and CSR registers reading helpers
//! for RISC-V interrupt and exception trap routines.

/// Reads the current value of the Machine Trap Value register (`mtval`).
///
/// On exceptions, `mtval` typically holds the faulting memory address 
/// or the instruction opcode that caused the trap.
///
/// # Returns
///
/// The raw value stored in the `mtval` CSR register.
#[inline(always)]
pub fn read_mtval() -> usize {
    let val: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) val);
    }
    val
}

/// Reads the current value of the Machine Status register (`mstatus`).
///
/// `mstatus` keeps track of the global interrupt enable flags, privilege modes, 
/// and CPU operating state.
///
/// # Returns
///
/// The raw value stored in the `mstatus` CSR register.
#[inline(always)]
pub fn read_mstatus() -> usize {
    let val: usize;
    unsafe {
        core::arch::asm!("csrr {}, mstatus", out(reg) val);
    }
    val
}

/// Translates a raw RISC-V cause code into a human-readable description string.
///
/// Handles both hardware interrupt causes and exception cause codes according 
/// to the privileged architecture specification (Machine Mode).
///
/// # Parameters
///
/// * `code` - The numeric cause value extracted from `mcause`.
/// * `is_interrupt` - `true` if the trap is an asynchronous interrupt, `false` for exceptions.
///
/// # Returns
///
/// A static string slice representing the trap name.
pub fn get_cause_name(code: usize, is_interrupt: bool) -> &'static str {
    if is_interrupt {
        match code {
            3 => "Machine Software Interrupt (MSIP)",
            7 => "Machine Timer Interrupt (MTIP)",
            11 => "Machine External Interrupt (MEIP)",
            _ => "Unknown Interrupt",
        }
    } else {
        match code {
            0x0 => "Instruction Address Misaligned",
            0x1 => "Instruction Access Fault",
            0x2 => "Illegal Instruction",
            0x3 => "Breakpoint (ebreak)",
            0x4 => "Load Address Misaligned",
            0x5 => "Load Access Fault",
            0x6 => "Store/AMO Address Misaligned",
            0x7 => "Store/AMO Access Fault",
            0x8 => "Environment Call from U-mode",
            0xb => "Environment Call from M-mode",
            _   => "Reserved / Unknown Exception",
        }
    }
}