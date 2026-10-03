//! Instruction Alignment Module
//!
//! Handles instruction address misaligned exceptions for the `trap_handler` function.

/// Handles instruction address misaligned exceptions triggered by invalid jumps or fetches.
///
/// This function reads the Machine Trap Value (`mtval`) CSR to capture the unaligned target
/// address that caused the exception. Because an unaligned instruction fetch indicates a 
/// severe control flow integrity violation (e.g., corrupted function pointer, invalid branch, 
/// or stack corruption), execution cannot safely resume. The function logs the faulting program 
/// counter (`mepc`) along with the bad target address (`mtval`), then triggers a kernel panic 
/// to isolate the failure and protect the system.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the instruction that attempted the unaligned fetch or jump.
pub fn handle_bad_inst_alig(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Control Flow Fault:\n\
         - Faulting Instruction (mepc): 0x{:08x}\n\
         - Bad Target Address  (mtval): 0x{:08x}",
        mepc, mtval
    );
}