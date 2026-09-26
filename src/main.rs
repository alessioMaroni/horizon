#![no_std]
#![no_main]

mod panic;

pub mod arch;
pub mod drivers;
pub mod io;

pub use crate::io::output::CONSOLE;

core::arch::global_asm!(include_str!("../image_def.s"));
core::arch::global_asm!(include_str!("../set_stack.s"));

#[unsafe(no_mangle)]
pub extern "C" fn _main() -> ! {
    CONSOLE.write_str("Hello, Kernel!\n");

    loop{}
}
