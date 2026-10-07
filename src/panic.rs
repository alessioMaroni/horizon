//! # Kernel Panic Handler
//!
//! Provides the top-level `#![no_std]` panic handler for the kernel.
//!
//! When an unrecoverable runtime error occurs (such as an explicit `panic!`,
//! a failed `unwrap()`, or an out-of-bounds array access), execution drops
//! into this module to print diagnostic details before halting the system.

use crate::CONSOLE;
use crate::trap::helpers::*;

/// Global panic handler for bare-metal execution.
///
/// Formats and outputs the panic location and message via the system [`CONSOLE`],
/// then enters an infinite loop to halt CPU execution safely.
///
/// # Parameters
///
/// * `info` - A structure containing panic details, such as the location in code
///            and the message payload.
///
/// # Note
///
/// This function is conditionally compiled under `#[cfg(not(test))]` to avoid
/// conflicting with standard test frameworks during unit testing.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    CONSOLE.write_fmt(format_args!(
        "\x1b[1;31m[PANIC!] Kernel panic!\x1b[0m\n\x1b[31m{}\x1b[0m\n",
        info
    ));

    loop {
        core::hint::spin_loop();
    }
}

/// Emergency trap handler invoked when stack pointer corruption or stack overflow 
/// is detected at the entry of the trap vector.
///
/// This function acts as a **Double Fault mitigation** mechanism. Instead of attempting 
/// to push a standard trap frame onto a corrupted or unmapped stack (which would trigger 
/// a catastrophic double fault and lock up the hardware), the low-level assembly pivots 
/// execution onto an isolated pre-allocated emergency stack and jumps here.
///
/// # Arguments
///
/// * `corrupted_sp` - The untrusted/compromised Stack Pointer (`sp`) value captured from `mscratch` 
///                    at the exact moment of the trap entry.
/// * `mcause`       - The Machine Cause Register value indicating why the core trapped.
/// * `mepc`         - The Machine Exception Program Counter value pointing to the instruction 
///                    that was executing when the trap occurred.
///
/// # Behavior
///
/// 1. Outputs a prominent ANSI-colored fatal error banner indicating emergency stack engagement.
/// 2. Dumps the exact value of the corrupted stack pointer for post-mortem forensics.
/// 3. Parses `mcause` to separate interrupt flags from exception cause codes.
/// 4. Invokes [`write_info`] to stream complete low-level RISC-V register telemetry.
/// 5. Triggers a terminal kernel panic to halt CPU execution securely within an infinite loop.
#[unsafe(no_mangle)]
pub extern "C" fn handle_stack_overflow_panic(corrupted_sp: usize, mcause: usize, mepc: usize) -> ! {
    // Output a high-priority, colored alert indicating that double-fault mitigation has engaged.
    CONSOLE.write_str("\x1b[1;31m\n[FATAL!] Emergency Stack Overflow Handler reached (Double Fault mitigation)!\x1b[0m\n");
    
    // Log the exact corrupted stack pointer value to aid in tracking down memory exhaustion or buffer overflows.
    CONSOLE.write_fmt(format_args!(
        "\x1b[33mCorrupted SP was : {:#010x}\x1b[0m\n",
        corrupted_sp
    ));

    // Decode mcause: bit 31 distinguishes asynchronous hardware interrupts from synchronous exceptions.
    let is_interrupt = (mcause & (1 << 31)) != 0;
    // Clear the top bit to isolate the specific exception or interrupt code number.
    let cause_code = mcause & !(1 << 31);

    // Stream full system diagnostic context using the shared telemetry reporting function.
    write_info(is_interrupt, cause_code, mepc);

    // Transition into an unrecoverable kernel panic, locking the system securely on the emergency stack.
    panic!("System halted securely on emergency stack");
}