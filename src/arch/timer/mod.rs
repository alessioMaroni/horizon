//! # Architecture - Timer Memory-Mapped Registers & Linker Symbols
//!
//! This module provides low-level definitions for RISC-V machine timer registers
//! and external symbol declarations exported by the linker script for the CLINT
//! (Core Local Interrupter) timer subsystem.

unsafe extern "C" {
    /// External linker symbol representing the memory reference 
    /// or value container for the machine time counter (`mtime`).
    pub static __MTIME_ADDR__: u64;
    
    /// External linker symbol representing the mutable memory reference 
    /// or value container for the machine time comparator (`mtimecmp`).
    pub static mut __MTIMECMP_ADDR__: u64;
}

/// Physical memory-mapped address for the RISC-V `mtime` (Machine Time) register.
/// This hardware counter increments monotonically at the system clock frequency.
pub const MTIME_ADDR: usize    = 0x0200BFF8;

/// Physical memory-mapped address for the RISC-V `mtimecmp` (Machine Time Compare) register.
/// An exception/interrupt is signaled when `mtime` reaches or exceeds this value.
pub const MTIMECMP_ADDR: usize = 0x02004000;