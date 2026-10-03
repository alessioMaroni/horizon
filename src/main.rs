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

    // =========================================================================
    // 1. RECOVERABLE TRAPS (Sequential - Execution resumes after trap handling)
    // =========================================================================

    // --- TEST 0x3: Breakpoint ---
    CONSOLE.write_str("\n[TEST] Triggering 0x3 Breakpoint...\n");
    unsafe {
        core::arch::asm!("ebreak");
    }
    CONSOLE.write_str("[OK] Returned from 0x3 Breakpoint!\n");

    // --- TEST 0xb: Environment Call in Machine Mode ---
    CONSOLE.write_str("[TEST] Triggering 0xb M-Mode ecall...\n");
    unsafe {
        core::arch::asm!("ecall");
    }
    CONSOLE.write_str("[OK] Returned from 0xb M-Mode ecall!\n");


    // =========================================================================
    // 2. FATAL FAULTS (Triggers Panic: Uncomment ONLY ONE TEST at a time)
    // =========================================================================

    // --- TEST 0x0: Instruction Address Misaligned (SW Simulated) ---
    // Note: On RP2350 with RVC (16-bit), hardware clears bit 0 of odd branch targets.
    /*
    CONSOLE.write_str("\n[FATAL TEST] Simulating 0x0 Instruction Misaligned...\n");
    let fake_bad_pc = 0x1000_0103usize;
    unsafe { core::arch::asm!("csrw mtval, {val}", val = in(reg) fake_bad_pc); }
    crate::trap::h_inst_alig::handle_bad_inst_alig(fake_bad_pc);
    */

    // --- TEST 0x1: Instruction Access Fault ---
    // Attempt to execute code from APB/AHB peripheral space (non-executable)
    /*
    CONSOLE.write_str("\n[FATAL TEST] Triggering 0x1 Instruction Access Fault...\n");
    let bad_fn: fn() = unsafe { core::mem::transmute(0x4000_0000usize) };
    bad_fn();
    */

    // --- TEST 0x2: Illegal Instruction ---
    // Unallocated/invalid 32-bit opcode (0x00000000)
    /*
    CONSOLE.write_str("\n[FATAL TEST] Triggering 0x2 Illegal Instruction...\n");
    unsafe {
        core::arch::asm!(".4byte 0x00000000");
    }
    */

    // --- TEST 0x5: Load Access Fault ---
    // Read from NULL or unmapped bus address (0x0000_0000)
    /*
    CONSOLE.write_str("\n[FATAL TEST] Triggering 0x5 Load Access Fault...\n");
    unsafe {
        let _val = core::ptr::read_volatile(0x0000_0000 as *const u32);
    }
    */

    // --- TEST 0x6: Store Address Misaligned (SW Simulated) ---
    // Note: Hazard3 handles unaligned accesses in HW, so this is simulated for handler validation.
    /*
    CONSOLE.write_str("\n[FATAL TEST] Simulating 0x6 Store Address Misaligned...\n");
    let fake_unaligned_addr = 0x2000_0001usize;
    unsafe { core::arch::asm!("csrw mtval, {val}", val = in(reg) fake_unaligned_addr); }
    crate::trap::h_store_amo_alig::handle_store_misaligned(0x1000_0204);
    */

    // --- TEST 0x7: Store Access Fault ---
    // Write to NULL or PMP/ACCESSCTRL protected address (0x0000_0000)
    /*
    CONSOLE.write_str("\n[FATAL TEST] Triggering 0x7 Store Access Fault...\n");
    unsafe {
        core::ptr::write_volatile(0x0000_0000 as *mut u32, 0xCAFE_BABE);
    }
    */

    CONSOLE.write_str("\n[MAIN] All non-fatal tests completed! Entering idle loop.\n");
    loop {}
}