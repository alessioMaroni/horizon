#![no_std]
#![no_main]

extern crate alloc;
#[allow(unused_imports)]
use alloc::vec::Vec;

#[cfg(feature = "tests")]
#[allow(unused_imports)]
use tests::trap::*;

mod panic;
mod trap;

pub mod arch;
pub mod drivers;
pub mod info;
pub mod io;
pub mod mm;
pub mod time;

use crate::info::Info;
pub use crate::io::output::CONSOLE;
use crate::mm::ALLOCATOR;
use crate::arch::mm::stack::{
    *,
    guard::setup_pmp_stack_guard,
};
use crate::time::init_timer;

use core::arch::global_asm;

global_asm!(include_str!("arch/init/image_def.s"));
global_asm!(include_str!("arch/init/set_stack.s"));

global_asm!(concat!(
    include_str!("arch/regs/save_regs.s"),
    "\n",
    include_str!("arch/regs/restore_regs.s"),
    "\n",
    include_str!("arch/trap/trap_entry.s")
));

#[unsafe(no_mangle)]
pub extern "C" fn _setup() -> ! {
    unsafe {
        core::arch::asm!(
            "la t0, trap_entry",
            "csrw mtvec, t0",
            options(nostack, preserves_flags)
        );
    }

    unsafe {
        let guard_base = &_stack_guard_start as *const u8 as usize;
        let guard_end = &_stack_guard_end as *const u8 as usize;
        let guard_size = guard_end - guard_base;

        setup_pmp_stack_guard(guard_base, guard_size);
    }

    {
        let info: Info = Info::init();
        ALLOCATOR.init(&info);
    }

    init_timer();

    _main();
}

#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Hello, World!\n");

    loop {}
}