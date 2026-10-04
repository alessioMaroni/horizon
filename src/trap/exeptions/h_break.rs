//! Break Module
//!
//! Handle breakpoint exeption for `trap_handler` function

/// Handles breakpoint exceptions (`ebreak`) triggered by the CPU.
///
/// This function inspects the instruction at the given program counter (`mepc`) 
/// to dynamically determine its length (2 bytes for compressed RVC instructions, 
/// or 4 bytes for standard 32-bit instructions). It then advances `mepc` past 
/// the breakpoint instruction and updates the `mepc` CSR, allowing execution 
/// to resume smoothly after an `mret` instruction.
///
/// # Arguments
/// * `mepc` - The Machine Exception Program Counter pointing to the `ebreak` instruction.
pub fn handle_breakpoint(mepc: usize) {
    // Read the instruction at mepc to dynamically determine its length (2 or 4 bytes)
    let inst = unsafe { core::ptr::read_unaligned(mepc as *const u16) };
    let inst_len = if (inst & 0x3) != 0x3 { 2 } else { 4 };

    // Advance `mepc` by the correct instruction length
    let next_mepc = mepc + inst_len;
    unsafe {
        core::arch::asm!("csrw mepc, {val}", val = in(reg) next_mepc);
    }
}

