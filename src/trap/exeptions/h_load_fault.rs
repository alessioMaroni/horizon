//! Load Access Fault Module
//!
//! Handles load access fault exceptions (cause 5) for the `trap_handler` function.

/// Handles load access fault exceptions (`mcause` = 5).
///
/// This exception occurs when a load instruction attempts to read from a memory address 
/// that does not exist, lacks Physical Memory Protection (PMP) read permissions, 
/// is restricted by ACCESSCTRL hardware, or triggers a bus error from a peripheral.
///
/// The `mtval` CSR contains the illegal or restricted memory address that was accessed.
/// Because reading from forbidden or non-existent memory violates system integrity and 
/// safety boundaries, execution cannot safely continue. This function retrieves both 
/// the faulting instruction address (`mepc`) and the target read address (`mtval`), 
/// logs the critical failure, and triggers a kernel panic.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the load instruction that caused the fault.
pub fn handle_load_access_fault(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Security/Memory Fault (Load Access Fault):\n\
         - Faulting Instruction (mepc): 0x{:08x}\n\
         - Invalid Read Target   (mtval): 0x{:08x}",
        mepc, mtval
    );
}