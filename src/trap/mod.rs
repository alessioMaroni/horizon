//! # Trap Handling Module
//!
//! This module contains the trap handling process and logic for the RISC-V architecture.
//! It defines the external assembly entry point (`trap_entry`) and the main high-level
//! Rust trap dispatcher (`trap_handler`).

pub mod h_break;
use crate::trap::h_break::handle_breakpoint;

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

    // Determines whether the trap is an interrupt or an exception:
    // * `true`  - It's an interrupt
    // * `false` - It's an exception
    let is_interrupt = (mcause & (1 << 31)) != 0;

    // Extracts the exception or interrupt code (e.g., 3)
    let cause_code = mcause & !(1 << 31);
    write!(CONSOLE, "[DEBUG] is_interrupt [{}]\n", is_interrupt);
    write!(CONSOLE, "[DEBUG] cause_code [{}]\n", cause_code);

    // Checks whether it's an interrupt or an exception
    if is_interrupt {
        match cause_code {
            // Unhandled interrupt
            _ => panic!("Unhandled interrupt [ No.: {} ]", cause_code),
        }
    } else {
        match cause_code {
            // Breakpoint: An ebreak or c.ebreak instruction was executed,
            // and no external debug host caught it.
            3 => handle_breakpoint(mepc),

            // Unhandled exception
            _ => panic!("Unhandled exception [ No.: {} at addr: 0x{:08x} ]", cause_code, mepc),
        }
    }
}