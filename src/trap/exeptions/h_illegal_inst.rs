//! Illegal Instruction Module
//!
//! Handles illegal instruction exceptions (cause 2) for the `trap_handler` function.

/// Handles illegal instruction exceptions (`mcause` = 2).
///
/// This exception occurs when the CPU encounters an unallocated/unsupported opcode, 
/// attempts to access a non-existent Control and Status Register (CSR), or tries to execute 
/// a privileged instruction/CSR access from a context with insufficient privilege levels.
///
/// On RISC-V hardware like Hazard3, the `mtval` CSR typically holds the raw instruction 
/// word that triggered the fault (or zero if hardware capture is not supported). Because 
/// executing invalid or unauthorized instructions violates system integrity, this function 
/// retrieves the faulting instruction address (`mepc`) and opcode (`mtval`), logs the error, 
/// and triggers a kernel panic.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing directly to the illegal instruction.
pub fn handle_illegal_instruction(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Execution Fault (Illegal Instruction):\n\
         - Faulting Instruction Address (mepc): 0x{:08x}\n\
         - Raw Instruction / Bad CSR Opcode (mtval): 0x{:08x}",
        mepc, mtval
    );
}