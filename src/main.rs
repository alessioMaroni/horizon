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

#[unsafe(no_mangle)]
pub extern "C" fn _setup() -> ! {
    // Set the Trap Handler
    unsafe {
        core::arch::asm!(
            "la t0, trap_entry",
            "csrw mtvec, t0",
            options(nostack, preserves_flags)
        );
    }

	let info: Info = Info::init();
	ALLOCATOR.init(&info);

	_main();
}

#[unsafe(no_mangle)]
pub fn _main() -> ! {
	CONSOLE.write_str("Hello, Kernel!\n");

	let mut try_vec: Vec<u32> = Vec::new();
	try_vec.push(129 as u32);

	CONSOLE.write_fmt(format_args!("Try vec: {:?}\n", try_vec));

    // Trap Handler Test
    // ---------------------------------------
    // Exeption 0x3 Breakpoint
    unsafe {
        core::arch::asm!("ebreak");
    }
    
    CONSOLE.write_str("[DEBUG] Hello, Kernel! After trap_handler return\n");
    // ---------------------------------------


	loop {}
}
