#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

mod panic;

pub mod arch;
pub mod drivers;
pub mod io;
pub mod info;
pub mod mm;

pub use crate::io::output::CONSOLE;
use crate::info::Info;
use crate::mm::ALLOCATOR;

core::arch::global_asm!(include_str!("../image_def.s"));
core::arch::global_asm!(include_str!("../set_stack.s"));

#[unsafe(no_mangle)]
pub extern "C" fn _setup() -> ! {
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

    loop{}
}
