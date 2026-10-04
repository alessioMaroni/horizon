//! Instruction Access Fault Module
//!
//! Handles instruction access fault exceptions (cause 1) for the `trap_handler` function.

/// Handles instruction access fault exceptions (`mcause` = 1).
///
/// This exception occurs when the CPU attempts to fetch an instruction from an address
/// that does not support execution (such as APB/AHB peripheral space on the RP2350),
/// violates Physical Memory Protection (PMP) execute permissions, is restricted by
/// ACCESSCTRL hardware, or receives a bus error from the memory subsystem.
///
/// Because the target memory address is non-executable or forbidden, execution cannot safely 
/// continue. This function retrieves the faulting fetch address from the `mtval` CSR, 
/// logs diagnostic details, and triggers a kernel panic to prevent security violations.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the address that failed instruction fetch.
pub fn handle_inst_access_fault(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Security/Memory Fault (Instruction Access Fault):\n\
         - Faulting Instruction (mepc): 0x{:08x}\n\
         - Unexecutable Target Address (mtval): 0x{:08x}",
        mepc, mtval
    );
}