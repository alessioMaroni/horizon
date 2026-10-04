//! # Trap Handling Helper Module
//!
//! Provides utility functions, CSR registers reading helpers, and formatted 
//! diagnostic printers for RISC-V interrupt and exception trap routines.

#[allow(unused_imports)]
use crate::CONSOLE;
#[allow(unused_imports)]
use crate::trap::TrapFrame;

/// Reads the current value of the Machine Trap Value register (`mtval`).
///
/// On exceptions, `mtval` typically holds the faulting memory address 
/// or the instruction opcode that caused the trap.
///
/// # Returns
///
/// The raw value stored in the `mtval` CSR register.
#[inline(always)]
#[cfg(any(feature = "debug", feature = "tests"))]
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
#[cfg(any(feature = "debug", feature = "tests"))]
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
#[cfg(any(feature = "debug", feature = "tests"))]
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

/// Prints a detailed diagnostic report for trap inspection over the system console.
///
/// Formats and displays key RISC-V CSRs (`mepc`, `mtval`, `mstatus`, `mcause`) alongside
/// a snapshot of general-purpose registers retrieved from the stack frame (`TrapFrame`).
///
/// # Parameters
///
/// * `mepc` - Machine Exception Program Counter (address of the faulting instruction or return target).
/// * `frame_ptr` - Pointer to the [`TrapFrame`] saved on the stack by the assembly entry vector.
/// * `is_interrupt` - Flag indicating whether the trap was triggered by an interrupt.
/// * `cause_code` - The specific trap cause code extracted from `mcause`.
#[cfg(any(feature = "debug", feature = "tests"))]
pub fn print_debug(mepc: usize, frame_ptr: *mut TrapFrame, is_interrupt: bool, cause_code: usize) {
    let mtval = read_mtval();
    let mstatus = read_mstatus();

    CONSOLE.write_str("\n==================== [ TRAP DIAGNOSTIC REPORT ] ====================\n");
    CONSOLE.write_fmt(format_args!(" Type         : {}\n", if is_interrupt { "INTERRUPT" } else { "EXCEPTION" }));
    CONSOLE.write_fmt(format_args!(" Cause Code   : {:#010x} ({})\n", cause_code, get_cause_name(cause_code, is_interrupt)));
    CONSOLE.write_fmt(format_args!(" Program Ctr  : {:#010x} (mepc)\n", mepc));
    CONSOLE.write_fmt(format_args!(" Target Value : {:#010x} (mtval: BadAddr or Opcode)\n", mtval));
    CONSOLE.write_fmt(format_args!(" CPU Status   : {:#010x} (mstatus)\n", mstatus));

    if !frame_ptr.is_null() {
        let frame = unsafe { &*frame_ptr };
        CONSOLE.write_str(" --- Saved Registers (TrapFrame) ---\n");
        CONSOLE.write_fmt(format_args!(
            " RA : {:#010x}  SP : {:#010x}  GP : {:#010x}  TP : {:#010x}\n",
            frame.ra, frame.sp, frame.gp, frame.tp
        ));
        CONSOLE.write_fmt(format_args!(
            " A0 : {:#010x}  A1 : {:#010x}  A2 : {:#010x}  A3 : {:#010x}\n",
            frame.a0, frame.a1, frame.a2, frame.a3
        ));
        CONSOLE.write_fmt(format_args!(
            " A4 : {:#010x}  A5 : {:#010x}  A6 : {:#010x}  A7 : {:#010x}\n",
            frame.a4, frame.a5, frame.a6, frame.a7
        ));
        CONSOLE.write_fmt(format_args!(
            " T0 : {:#010x}  T1 : {:#010x}  T2 : {:#010x}  T3 : {:#010x}\n",
            frame.t0, frame.t1, frame.t2, frame.t3
        ));
        CONSOLE.write_fmt(format_args!(
            " T4 : {:#010x}  T5 : {:#010x}  T6 : {:#010x}  S0 : {:#010x}\n",
            frame.t4, frame.t5, frame.t6, frame.s0
        ));
        CONSOLE.write_fmt(format_args!(
            " S1 : {:#010x}  S2 : {:#010x}  S3 : {:#010x}  S4 : {:#010x}\n",
            frame.s1, frame.s2, frame.s3, frame.s4
        ));
        CONSOLE.write_fmt(format_args!(
            " S5 : {:#010x}  S6 : {:#010x}  S7 : {:#010x}  S8 : {:#010x}\n",
            frame.s5, frame.s6, frame.s7, frame.s8
        ));
        CONSOLE.write_fmt(format_args!(
            " S9 : {:#010x}  S10: {:#010x}  S11: {:#010x}\n",
            frame.s9, frame.s10, frame.s11
        ));
    }
    CONSOLE.write_str("====================================================================\n\n");
}