//! # RISC-V Machine Timer Subsystem
//!
//! This module implements the low-level hardware timer management for the RISC-V
//! architecture. It interfaces directly with the CLINT (Core Local Interrupter) via 
//! memory-mapped I/O (MMIO) and integrates with the formally verified Ada/SPARK 
//! time calculation routines to guarantee deterministic preemption scheduling.

use core::arch::asm;
use crate::arch::timer::{MTIME_ADDR, MTIMECMP_ADDR};

unsafe extern "C" {
    /// External Foreign Function Interface (FFI) binding to the SPARK-verified 
    /// time computation routine implemented in Ada (`spark_compute_next_tick`).
    /// 
    /// # Safety
    /// The caller must ensure parameters are valid and that the addition within 
    /// the Ada function does not breach safety invariants.
    pub fn spark_compute_next_tick(current_mtime: u64, delta_ticks: i32) -> u64;
}

/// Baseline timer frequency set to 1 MHz (1,000,000 ticks per second).
pub const TIMER_FREQ_HZ: u32 = 1_000_000;

/// Number of hardware clock ticks corresponding to a 1 millisecond interval,
/// utilized to drive the kernel's periodic preemptive scheduler.
pub const TICKS_PER_MS: i32  = (TIMER_FREQ_HZ / 1_000) as i32;

/// Reads the current 64-bit absolute value from the hardware `mtime` register.
///
/// Uses volatile reads to prevent compiler optimizations from caching 
/// or reordering memory accesses to the hardware counter.
#[inline(always)]
pub fn read_mtime() -> u64 {
    unsafe {
        let ptr = MTIME_ADDR as *const u64;
        core::ptr::read_volatile(ptr)
    }
}

/// Sets the comparison target value in the `mtimecmp` register.
///
/// When the hardware `mtime` counter reaches or exceeds this value, 
/// a machine timer interrupt (MTI) is signaled to the core.
///
/// # Arguments
/// * `value` - The absolute target timestamp for the next timer expiration.
#[inline(always)]
pub fn write_mtimecmp(value: u64) {
    unsafe {
        let ptr = MTIMECMP_ADDR as *mut u64;
        core::ptr::write_volatile(ptr, value);
    }
}

/// Initializes the RISC-V machine timer subsystem and enables timer interrupts.
///
/// This function performs the following critical sequence:
/// 1. Samples the current hardware time counter (`mtime`).
/// 2. Invokes the SPARK-verified `spark_compute_next_tick` to calculate 
///    the next absolute deadline safely (+1 ms).
/// 3. Programs the `mtimecmp` register with the computed deadline.
/// 4. Enables Machine Timer Interrupts (MTI) by setting bit 7 in the `mie` CSR.
/// 5. Enables global machine interrupts by setting bit 3 in the `mstatus` CSR.
#[allow(asm_sub_register)]
pub fn init_timer() {
    let current = read_mtime();
    let next_deadline = unsafe { spark_compute_next_tick(current, TICKS_PER_MS) };
    
    write_mtimecmp(next_deadline);

    unsafe {
        // Enable Machine Timer Interrupt (MTI) in the Machine Interrupt Enable (mie) register (bit 7).
        asm!("csrs mie, {mask}", mask = in(reg) (1u32 << 7));
        // Enable Global Interrupts in the Machine Status (mstatus) register (bit 3).
        asm!("csrs mstatus, {mask}", mask = in(reg) (1u32 << 3));
    }
}