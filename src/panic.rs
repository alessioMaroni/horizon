//! # Kernel Panic Handler
//!
//! Provides the top-level `#![no_std]` panic handler for the kernel.
//!
//! When an unrecoverable runtime error occurs (such as an explicit `panic!`,
//! a failed `unwrap()`, or an out-of-bounds array access), execution drops
//! into this module to print diagnostic details before halting the system.

pub use crate::CONSOLE;

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

    loop {}
}