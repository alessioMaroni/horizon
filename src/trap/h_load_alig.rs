//! Load Address Misaligned Module
//!
//! Handles load address misaligned exceptions (cause 4) for the `trap_handler` function.

/// Handles load address misaligned exceptions (`mcause` = 4).
///
/// This exception occurs when a load instruction (e.g., `lw`, `lh`) attempts to read 
/// data from a memory address that is not naturally aligned to the data type size 
/// (e.g., reading a 32-bit word from an address not divisible by 4).
///
/// The `mtval` CSR contains the unaligned target memory address that triggered the fault.
/// Because unaligned memory reads violate architecture constraints when not emulated, 
/// this function retrieves both the faulting instruction address (`mepc`) and the unaligned 
/// target address (`mtval`), logs the error, and triggers a kernel panic.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the load instruction.
pub fn handle_load_misaligned(mepc: usize) {
    let mtval: usize;
    unsafe {
        core::arch::asm!("csrr {}, mtval", out(reg) mtval);
    }

    panic!(
        "Memory Alignment Fault (Load Address Misaligned):\n\
         - Faulting Instruction (mepc): 0x{:08x}\n\
         - Unaligned Target Read Address (mtval): 0x{:08x}",
        mepc, mtval
    );
}