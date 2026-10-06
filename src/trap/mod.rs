//! # Trap Handling Module
//!
//! This module contains the trap handling process and logic for the RISC-V architecture.
//! It defines the external assembly entry point (`trap_entry`) and the main high-level
//! Rust trap dispatcher (`trap_handler`).

pub mod exeptions;
pub mod helpers;

use crate::trap::helpers::*;

use crate::trap::exeptions::h_break::handle_breakpoint;
use crate::trap::exeptions::h_inst_alig::handle_bad_inst_alig;
use crate::trap::exeptions::h_inst_featch_fault::handle_inst_access_fault;
use crate::trap::exeptions::h_illegal_inst::handle_illegal_instruction;
use crate::trap::exeptions::h_load_alig::handle_load_misaligned;
use crate::trap::exeptions::h_load_fault::handle_load_access_fault;
use crate::trap::exeptions::h_store_amo_alig::handle_store_misaligned;
use crate::trap::exeptions::h_store_amo_fault::handle_store_access_fault;
use crate::trap::exeptions::h_syscalls::{handle_syscall_mmode, handle_syscall_umode};

use crate::CONSOLE;
use crate::arch::regs::general::TrapFrame;

/// Main high-level trap handler called directly from the assembly stub.
///
/// # Arguments
/// * `mcause` - The cause of the trap, indicating whether it's an interrupt or an exception, 
///             along with the specific error/event code.
/// * `mepc`   - The program counter of the instruction that caused or was interrupted by the trap.
/// * `_frame` - A raw pointer to the saved register frame (trap frame) on the stack.
#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(
        mcause: usize,
        mepc: usize, 
        #[allow(unused_variables)]
        frame_ptr: *mut TrapFrame
    )
{


    // Determines whether the trap is an interrupt or an exception:
    // * `true`  - It's an interrupt
    // * `false` - It's an exception
    let is_interrupt = (mcause & (1 << 31)) != 0;

    // Extracts the exception or interrupt code (e.g., 3)
    let cause_code = mcause & !(1 << 31);

    CONSOLE.write_fmt(format_args!(
        "\n\x1b[1;33m[TRAP TRIGGERED]\x1b[0m Type: \x1b[1;36m{}\x1b[0m | Code: \x1b[1;33m{:#010x}\x1b[0m | mepc: \x1b[1;33m{:#010x}\x1b[0m\n",
        if is_interrupt { "INTERRUPT" } else { "EXCEPTION" },
        cause_code,
        mepc
    ));


    let mtval = read_mtval();
    let mstatus = read_mstatus();

    // Log fundamental RISC-V Control and Status Registers (CSRs) and execution context.
    CONSOLE.write_fmt(format_args!(
        "\nType         : {}\n\
         Cause Code   : {:#010x} ({})\n\
         Program Ctr  : {:#010x} (mepc)\n\
         Target Value : {:#010x} (mtval: BadAddr or Opcode)\n\
         CPU Status   : {:#010x} (mstatus)\n\n\
         ",
        if is_interrupt { "INTERRUPT" } else { "EXCEPTION" },
        cause_code,
        get_cause_name(cause_code, is_interrupt),
        mepc,
        mtval,
        mstatus
    ));

    // Perform a full CPU register dump if a valid trap frame snapshot exists.
    if !frame_ptr.is_null() {
        // # SAFETY: The pointer validity is verified against null. It is assumed that 
        // `frame_ptr` references a correctly aligned, valid `TrapFrame` instance 
        // allocated on the stack during the trap vector entry sequence.
        let frame = unsafe { &*frame_ptr };
        frame.dump(Some(mcause), Some(mepc));
    }

    // Checks whether it's an interrupt or an exception
    if is_interrupt {
        match cause_code {
            // TODO: Finish the interrupt handler
            // Unhandled interrupt
            _ => panic!("[PANIC!] Unhandled interrupt [ No.: {} ]!", cause_code),
        }
    } else {
        match cause_code {
            // Instruction alignment: Does not occur on RP2350, because 16-bit compressed instructions are
            // implemented, and it is impossible to jump to a byte-aligned addres.
            //
            // This can still happen if the core jumps to an odd address.
            0x0 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Instruction Address Misaligned!\x1b[0m\n\n");
                handle_bad_inst_alig(mepc);
            }

            // Instruction fetch fault: Attempted to fetch from an address that does not support instruction fetch,
            // or lacks PMP execute permission, or is forbidden by ACCESSCTRL, or
            // returned a fault from the memory device itself.
            0x1 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Instruction Access Fault (Fetch Violation)!\x1b[0m\n\n");
                handle_inst_access_fault(mepc);
            }

            // Illegal instruction: Encountered an instruction that was not a valid RISC-V opcode implemented by this
            // processor, or attempted to access a nonexistent CSR, or attempted to execute a privileged instruction or
            // access a privileged CSR without sufficient privilege.
            0x2 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Illegal Instruction Exception!\x1b[0m\n\n");
                handle_illegal_instruction(mepc);
            }

            // Breakpoint: An ebreak or c.ebreak instruction was executed,
            // and no external debug host caught it.
            0x3 => handle_breakpoint(mepc),

            // Load alignment: Attempted to load from an address that was not a multiple of access size.
            0x4 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Load Address Misaligned Exception!\x1b[0m\n\n");
                handle_load_misaligned(mepc);
            }

            // Load fault: Attempted to load from an address that does not exist, or lacks PMP read permissions, or is
            // forbidden by ACCESSCTRL, or returned a fault from a peripheral.
            0x5 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Load Access Fault (Read Violation)!\x1b[0m\n\n");
                handle_load_access_fault(mepc);
            }

            // Store/AMO alignment: Attempted to write to an address that was not a multiple of access size.
            0x6 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Store Address Misaligned Exception!\x1b[0m\n\n");
                handle_store_misaligned(mepc);
            }

            // Store/AMO fault: Attempted to write to an address that does not exist, or lacks PMP write permissions, or
            // is forbidden by ACCESSCTRL, or returned a fault from a peripheral. Also raised when attempting an AMO
            // on an address that does not support AHB5 exclusives.
            0x7 => {
                CONSOLE.write_str("\x1b[1;31m[CRITICAL!] Store Access Fault (Write/AMO Violation)!\x1b[0m\n\n");
                handle_store_access_fault(mepc);
            }

            // An ecall instruction was executed in U-mode.
            0x8 => handle_syscall_umode(mepc),

            // An ecall instruction was executed in M-mode.
            0xb => handle_syscall_mmode(mepc),

            // Unhandled exception
           _ => panic!(
                "\x1b[1;31m[PANIC!] Unhandled exception\x1b[0m [ No.: \x1b[1;33m{}\x1b[0m at addr: \x1b[1;33m{:#010x}\x1b[0m ]!",
                cause_code, 
                mepc
            ),
        }
    }
}