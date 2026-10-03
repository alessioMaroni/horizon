#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

mod panic;
mod trap;

pub mod arch;
pub mod drivers;
pub mod info;
pub mod io;
pub mod mm;

use crate::info::Info;
pub use crate::io::output::CONSOLE;
use crate::mm::ALLOCATOR;

core::arch::global_asm!(include_str!("arch/init/image_def.s"));
core::arch::global_asm!(include_str!("arch/init/set_stack.s"));
core::arch::global_asm!(include_str!("arch/trap/trap_entry.s"));

/// Kernel initialization and boot setup routine.
/// 
/// This function is the primary entry point called right after low-level boot 
/// assembly. It configures the core architecture features, initializes the 
/// memory allocator, and hands control over to the main kernel loop.
/// 
/// # Safety
/// This function is marked `unsafe` and `extern "C"` because it relies on raw hardware 
/// manipulation, interacts directly with assembly labels, and must never return (`-> !`).
#[unsafe(no_mangle)]
pub extern "C" fn _setup() -> ! {
    // Configure the RISC-V trap handler vector base register (mtvec).
    // This directs the CPU where to jump when an exception or interrupt occurs.
    unsafe {
        core::arch::asm!(
            "la t0, trap_entry",  // Load the address of the assembly trap entry stub into t0
            "csrw mtvec, t0",     // Write t0 into the Machine Trap-Vector Base-Address Register
            options(nostack, preserves_flags)
        );
    }

    // Initialize hardware and platform information structure
    let info: Info = Info::init();
    
    // Initialize the global heap/memory allocator using the discovered system info
    ALLOCATOR.init(&info);

    // Transition execution to the main kernel logic (never returns)
    _main();
}

#[unsafe(no_mangle)]
pub fn _main() -> ! {
	CONSOLE.write_str("Hello, Kernel!\n");

	let mut try_vec: Vec<u32> = Vec::new();
	try_vec.push(129 as u32);

	CONSOLE.write_fmt(format_args!("Try vec: {:?}\n", try_vec));

	// panic!("[TEST] Kernel panic");

    // Trap Handler Test
    // ---------------------------------------
    unsafe {
        core::arch::asm!("ebreak");
    }
    
    CONSOLE.write_str("[DEBUG] Hello, Kernel! After trap_handler return\n");
    // ---------------------------------------

	loop {}
}
