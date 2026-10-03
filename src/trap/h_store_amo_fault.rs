//! Store Access Fault Module
//!
//! Handles store/AMO access fault exceptions (cause 7) for the `trap_handler` function.

/// Handles store/AMO access fault exceptions (`mcause` = 7).
///
/// This exception occurs when a store instruction or Atomic Memory Operation (AMO) 
/// attempts to write to an unmapped/non-existent memory address, violates Physical Memory 
/// Protection (PMP) write permissions, is blocked by ACCESSCTRL hardware, receives a bus 
/// error from a peripheral, or attempts an AMO on a memory region that does not support AHB5 
/// exclusive accesses.
///
/// The `mtval` CSR contains the illegal target write address that triggered the fault.
/// Because unauthorized or invalid writes violate memory safety and kernel integrity, 
/// execution cannot safely continue. This function retrieves both the faulting instruction 
/// address (`mepc`) and the invalid target address (`mtval`), logs the error, and triggers 
/// a kernel panic.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the faulting store/AMO instruction.
pub fn handle_store_access_fault(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Security/Memory Fault (Store/AMO Access Fault):\n\
         - Faulting Instruction (mepc): 0x{:08x}\n\
         - Invalid Write Target  (mtval): 0x{:08x}",
        mepc, mtval
    );
}