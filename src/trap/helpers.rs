//! # Trap Handling Helper Module
//!
//! Provides utility functions and CSR registers reading helpers
//! for RISC-V interrupt and exception trap routines.

use crate::CONSOLE;

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


/// Outputs a comprehensive diagnostic telemetry report for a trap event (interrupt or exception).
///
/// This function formats and writes critical machine state parameters directly to the system console (`CONSOLE`).
/// It is designed for post-mortem analysis in bare-metal and safety-critical environments, providing 
/// engineers with precise context regarding what caused the CPU core to trap.
///
/// # Arguments
///
/// * `is_interrupt` - A boolean flag indicating whether the trap was triggered by an asynchronous 
///                    hardware interrupt (`true`) or a synchronous exception/fault (`true` / `false`).
/// * `cause_code`   - The specific numerical identifier/code of the trap (extracted from the `mcause` CSR).
/// * `mepc`         - The Machine Exception Program Counter, representing the virtual/physical memory 
///                    address of the instruction where execution was interrupted or faulted.
///
/// # Side Effects
///
/// Reads live RISC-V Control and Status Registers (`mtval` and `mstatus`) and streams formatted 
/// ANSI-colored diagnostic strings over the serial/console interface.
pub fn write_info(is_interrupt: bool, cause_code: usize, mepc: usize) {
    // Stream a high-level summary header line indicating the trap type, cause code, and program counter.
    CONSOLE.write_fmt(format_args!(
        "\n\x1b[1;33m[TRAP TRIGGERED]\x1b[0m Type: \x1b[1;36m{}\x1b[0m | Code: \x1b[1;33m{:#010x}\x1b[0m | mepc: \x1b[1;33m{:#010x}\x1b[0m\n",
        if is_interrupt { "INTERRUPT" } else { "EXCEPTION" },
        cause_code,
        mepc
    ));

    // Read additional low-level hardware context from RISC-V CSRs:
    // - mtval: Contains supplementary fault-specific information (e.g., bad memory address or illegal instruction opcode).
    let mtval = read_mtval();
    // - mstatus: Contains the global interrupt enable bits and previous privilege mode flags.
    let mstatus = read_mstatus();

    // Log a detailed, multi-line diagnostic block containing all core execution context parameters
    // alongside human-readable descriptions for rapid debugging.
    CONSOLE.write_fmt(format_args!(
        "\nType         : {}\n\
         Cause Code   : {:#010x} ({})\n\
         Program Ctr  : {:#010x} (mepc)\n\
         Target Value : {:#010x} (mtval: BadAddr or Opcode)\n\
         CPU Status   : {:#010x} (mstatus)\n\n\
         ",
        if is_interrupt { "INTERRUPT" } else { "EXCEPTION" },
        cause_code,
        get_cause_name(cause_code, is_interrupt),
        mepc,
        mtval,
        mstatus
    ));
}