//! # Trap Handling Module
//!
//! This module contains the trap handling process and logic for the RISC-V architecture.
//! It defines the external assembly entry point (`trap_entry`) and the main high-level
//! Rust trap dispatcher (`trap_handler`).

pub mod h_break;
pub mod h_inst_alig;
pub mod h_inst_featch_fault;
pub mod h_illegal_inst;
pub mod h_load_alig;
pub mod h_load_fault;

use crate::trap::h_break::handle_breakpoint;
use crate::trap::h_inst_alig::handle_bad_inst_alig;
use crate::trap::h_inst_featch_fault::handle_inst_access_fault;
use crate::trap::h_illegal_inst::handle_illegal_instruction;
use crate::trap::h_load_alig::handle_load_misaligned;
use crate::trap::h_load_fault::handle_load_access_fault;

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
    CONSOLE.write_fmt(format_args!("[DEBUG] is_interrupt [{:?}]\n", is_interrupt));
    CONSOLE.write_fmt(format_args!("[DEBUG] cause_code [{:?}]\n", cause_code));

    // Checks whether it's an interrupt or an exception
    if is_interrupt {
        match cause_code {
            // Unhandled interrupt
            _ => panic!("[PANIC!] Unhandled interrupt [ No.: {} ]", cause_code),
        }
    } else {
        match cause_code {
            // Instruction alignment: Does not occur on RP2350, because 16-bit compressed instructions are
            // implemented, and it is impossible to jump to a byte-aligned addres.
            //
            // This can still happen if the core jumps to an odd address.
            0x0 => {
                CONSOLE.write_str("[CRITICAL!] Instruction Address Misaligned!\n");
                handle_bad_inst_alig(mepc);
            }

            // Instruction fetch fault: Attempted to fetch from an address that does not support instruction fetch,
            // or lacks PMP execute permission, or is forbidden by ACCESSCTRL, or
            // returned a fault from the memory device itself.
            0x1 => {
                CONSOLE.write_str("[CRITICAL!] Instruction Access Fault (Fetch Violation)\n");
                handle_inst_access_fault(mepc);
            }

            // Illegal instruction: Encountered an instruction that was not a valid RISC-V opcode implemented by this
            // processor, or attempted to access a nonexistent CSR, or attempted to execute a privileged instruction or
            // access a privileged CSR without sufficient privilege.
            0x2 => {
                CONSOLE.write_str("[CRITICAL!] Illegal Instruction Exception\n");
                handle_illegal_instruction(mepc);
            }

            // Breakpoint: An ebreak or c.ebreak instruction was executed,
            // and no external debug host caught it.
            0x3 => handle_breakpoint(mepc),

            // Load alignment: Attempted to load from an address that was not a multiple of access size.
            0x4 => {
                CONSOLE.write_str("[CRITICAL!] Load Address Misaligned Exception\n");
                handle_load_misaligned(mepc);
            }

            // Load fault: Attempted to load from an address that does not exist, or lacks PMP read permissions, or is
            // forbidden by ACCESSCTRL, or returned a fault from a peripheral.
            0x5 => {
                CONSOLE.write_str("[CRITICAL] Load Access Fault (Read Violation)\n");
                handle_load_access_fault(mepc);
            }

            // Unhandled exception
            _ => panic!("[PANIC!] Unhandled exception [ No.: {} at addr: 0x{:08x} ]", cause_code, mepc),
        }
    }
}