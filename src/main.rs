#![no_std]
#![no_main]

extern crate alloc;

#[cfg(feature = "tests")]
use tests::trap::*;

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
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x8_syscall_umode();
        trigger_0xb_syscall_mmode();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}