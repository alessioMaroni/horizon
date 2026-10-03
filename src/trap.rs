//! # Trap Handling Module
//!
//! This module contains the trap handling process and logic for the RISC-V architecture.
//! It defines the external assembly entry point (`trap_entry`) and the main high-level
//! Rust trap dispatcher (`trap_handler`).

use crate::CONSOLE;

/// Main high-level trap handler called directly from the assembly stub.
///
/// # Arguments
/// * `mcause` - The cause of the trap, indicating whether it's an interrupt or an exception, 
///             along with the specific error/event code.
/// * `mepc`   - The program counter of the instruction that caused or was interrupted by the trap.
/// * `_frame` - A raw pointer to the saved register frame (trap frame) on the stack.
#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(mcause: usize, mepc: usize, _frame: *mut usize) {
    CONSOLE.write_str("[DEBUG] Trap Handler!\n");

    // Helper function, used for checking if it's an exeption or an interrupt
    // # Values
    // * `true`  - It's an interrupt
    // * `false` - It's an exeption
    let is_interrupt = (mcause & (1 << 31)) != 0;

    // Helper function, store the exep. or the inter. code (Es. 3)
    let cause_code = mcause & !(1 << 31);
    write!(CONSOLE, "[DEBUG] is_interrupt [{}]\n", is_interrupt);
    write!(CONSOLE, "[DEBUG] cause_code [{}]\n", cause_code);

    // Checking if it's an interrupt or an exeption
    if is_interrupt {
        match cause_code{
            // Unandled interrupt
            _ => panic!("Unhandled exception [ n.: {} ]", cause_code),
        }
    }
    else {
        match cause_code {
            // Breakpoint: An ebreak or c.ebreak instruction was executed,
            // no external debug host caught it.
            3 => handle_breakpoint(mepc),

            // Unandled exeption
            _ => panic!("Unhandled exception [ n.: [{}] at addr.: 0x{:08x} ]", cause_code, mepc),
        }
    }
}

/// Handles breakpoint exceptions (`ebreak`) triggered by the CPU.
///
/// This function inspects the instruction at the given program counter (`mepc`) 
/// to dynamically determine its length (2 bytes for compressed RVC instructions, 
/// or 4 bytes for standard 32-bit instructions). It then advances `mepc` past 
/// the breakpoint instruction and updates the `mepc` CSR, allowing execution 
/// to resume smoothly after an `mret` instruction.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the `ebreak` instruction.
fn handle_breakpoint(mepc: usize) {
    // Read the instruction at mepc to dynamically determine its length (2 or 4 bytes)
    let inst = unsafe { core::ptr::read_unaligned(mepc as *const u16) };
    let inst_len = if (inst & 0x3) != 0x3 { 2 } else { 4 };

    // Advance `mepc` by the correct instruction length
    let next_mepc = mepc + inst_len;
    unsafe {
        core::arch::asm!("csrw mepc, {val}", val = in(reg) next_mepc);
    }
}
