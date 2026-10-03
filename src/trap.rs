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
    
    // Extract the exception/interrupt cause code by clearing the MSB (bit 31)
    let cause_code = mcause & !(1 << 31);

    // Check if the exception is a Breakpoint (cause code 3, triggered by 'ebreak')
    if cause_code == 3 {
        // Read the instruction at mepc to dynamically determine its length (2 or 4 bytes)
        let inst = unsafe { core::ptr::read_unaligned(mepc as *const u16) };
        let inst_len = if (inst & 0x3) != 0x3 { 2 } else { 4 };

        // Advance `mepc` by the correct instruction length
        let next_mepc = mepc + inst_len;
        unsafe {
            core::arch::asm!("csrw mepc, {val}", val = in(reg) next_mepc);
        }
    }
}
