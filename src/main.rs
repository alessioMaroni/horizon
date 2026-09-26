#![no_std]
#![no_main]

mod panic;

core::arch::global_asm!(include_str!("../image_def.s"));
core::arch::global_asm!(include_str!("../set_stack.s"));

#[unsafe(no_mangle)]
pub extern "C" fn _main() -> ! {
    let uart = 0x1000_0000 as *mut u8;
    
    unsafe {
        uart.write_volatile(b'A');
        uart.write_volatile(b'\n');
    }

    loop{}
}
