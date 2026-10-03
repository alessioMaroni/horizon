//! Environment Call (Syscall) Modules
//!
//! Handles U-mode (cause 8) and M-mode (cause 11) ecall traps for the `trap_handler`.

/// Handles system calls originating from User Mode (`mcause` = 8).
///
/// Executes requested kernel services initiated by user space applications via `ecall`.
/// System call arguments and identifiers are passed via core ABI registers (`a0`-`a7`).
///
/// To prevent re-triggering the `ecall` instruction upon returning with `mret`,
/// this handler advances `mepc` by 4 bytes (the fixed size of the `ecall` instruction).
///
/// # Arguments
/// * `mepc` - The program counter at the faulting `ecall` instruction.
pub fn handle_syscall_umode(mepc: usize) {
    // TODO: Dispatch syscall according to register ABI (e.g. reading syscall ID from a7)

    // Advance mepc past the 4-byte ecall instruction
    let next_mepc = mepc + 4;
    unsafe {
        core::arch::asm!("csrw mepc, {val}", val = in(reg) next_mepc);
    }
}

/// Handles system calls or supervisor requests in Machine Mode (`mcause` = 11).
///
/// Executes machine-level service calls initiated within M-mode context.
///
/// To prevent infinite trap loops upon execution of `mret`, this handler 
/// advances `mepc` by 4 bytes (the fixed size of the `ecall` instruction).
///
/// # Arguments
/// * `mepc` - The program counter at the faulting `ecall` instruction.
pub fn handle_syscall_mmode(mepc: usize) {
    // TODO: Handle internal M-mode service request

    // Advance mepc past the 4-byte ecall instruction
    let next_mepc = mepc + 4;
    unsafe {
        core::arch::asm!("csrw mepc, {val}", val = in(reg) next_mepc);
    }
}