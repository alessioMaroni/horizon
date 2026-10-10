//! # Timer Interrupt Handler Subsystem
//!
//! This module manages the periodic servicing of machine timer interrupts (MTI)
//! to drive the kernel's preemption and deterministic task scheduling ticks.

use crate::time::{TICKS_PER_MS, read_mtime, write_mtimecmp, spark_compute_next_tick};

/// Services the periodic machine timer interrupt (MTI).
///
/// This routine is executed upon each timer expiration to maintain the 
/// continuous ticking of the scheduler. It performs the following deterministic sequence:
/// 1. Samples the current absolute hardware time counter (`mtime`).
/// 2. Computes the next absolute target deadline safely by invoking the 
///    SPARK-verified `spark_compute_next_tick` routine (+1 ms interval).
/// 3. Reprograms the `mtimecmp` register to schedule the subsequent timer interrupt.
pub fn handle_timer_tick() {
    let current = read_mtime();
    let next_deadline = unsafe { spark_compute_next_tick(current, TICKS_PER_MS) };
    
    write_mtimecmp(next_deadline);  
}