//! Store Address Misaligned Module
//!
//! Handles store address misaligned exceptions (cause 6) for the `trap_handler` function.

/// Handles store/AMO address misaligned exceptions (`mcause` = 6).
///
/// This exception occurs when a store instruction (e.g., `sw`, `sh`) or an Atomic 
/// Memory Operation (AMO) attempts to write data to a memory address that is not 
/// naturally aligned to the data type size (e.g., writing a 32-bit word to an address 
/// not divisible by 4).
///
/// The `mtval` CSR contains the unaligned target write address that triggered the fault.
/// Because unaligned write or atomic operations violate architecture constraints and 
/// risk memory corruption, execution cannot safely continue. This function retrieves both 
/// the faulting instruction address (`mepc`) and the unaligned target address (`mtval`), 
/// logs the error, and triggers a kernel panic.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the store instruction.
pub fn handle_store_misaligned(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Memory Alignment Fault (Store/AMO Address Misaligned):\n\
         - Faulting Instruction (mepc): 0x{:08x}\n\
         - Unaligned Target Write Address (mtval): 0x{:08x}",
        mepc, mtval
    );
}