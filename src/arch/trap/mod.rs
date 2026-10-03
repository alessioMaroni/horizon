//! Trap entry
//!
//! This module declares the external assembly entry point (`trap_entry`) 
//! used by the processor hardware when a trap (exception or interrupt) occurs.

// External assembly declaration for the low-level trap entry stub.
//
// The processor jumps directly to this label (configured via `mtvec`) when a trap occurs.
// The assembly stub is responsible for saving all general-purpose registers onto the stack,
// passing the trap parameters to the high-level Rust handler, and eventually 
// restoring the register state before executing `mret`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn trap_entry();
}