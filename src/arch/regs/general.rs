// TODO: Document

use crate::CONSOLE;

// Trap frame
// extracted from arch/trap/trap_entry.s
// TODO: Consider to change this enum name
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    _padding: usize,
}

impl TrapFrame {
    pub fn dump(&self, mcause: Option<usize>, mepc: Option<usize>) {
        let _ = CONSOLE.write_fmt(format_args!("\x1b[1;33mREGISTER DUMP:\x1b[0m\n"));
        
        if let (Some(cause), Some(epc)) = (mcause, mepc) {
            let _ = CONSOLE.write_fmt(format_args!(
                "\x1b[1;33mmcause:\x1b[0m {:#010x} | \x1b[1;33mmepc:\x1b[0m {:#010x}\n",
                cause, epc
            ));
        }

        let _ = CONSOLE.write_fmt(format_args!(
            "\x1b[1;33mra\x1b[0m : {:#010x}   \x1b[1;33msp\x1b[0m : {:#010x}   \x1b[1;33mgp\x1b[0m : {:#010x}   \x1b[1;33mtp\x1b[0m : {:#010x}\n\
             \x1b[1;33mt0\x1b[0m : {:#010x}   \x1b[1;33mt1\x1b[0m : {:#010x}   \x1b[1;33mt2\x1b[0m : {:#010x}   \x1b[1;33ms0\x1b[0m : {:#010x}\n\
             \x1b[1;33ms1\x1b[0m : {:#010x}   \x1b[1;33ma0\x1b[0m : {:#010x}   \x1b[1;33ma1\x1b[0m : {:#010x}   \x1b[1;33ma2\x1b[0m : {:#010x}\n\
             \x1b[1;33ma3\x1b[0m : {:#010x}   \x1b[1;33ma4\x1b[0m : {:#010x}   \x1b[1;33ma5\x1b[0m : {:#010x}   \x1b[1;33ma6\x1b[0m : {:#010x}\n\
             \x1b[1;33ma7\x1b[0m : {:#010x}   \x1b[1;33ms2\x1b[0m : {:#010x}   \x1b[1;33ms3\x1b[0m : {:#010x}   \x1b[1;33ms4\x1b[0m : {:#010x}\n\
             \x1b[1;33ms5\x1b[0m : {:#010x}   \x1b[1;33ms6\x1b[0m : {:#010x}   \x1b[1;33ms7\x1b[0m : {:#010x}   \x1b[1;33ms8\x1b[0m : {:#010x}\n\
             \x1b[1;33ms9\x1b[0m : {:#010x}   \x1b[1;33ms10\x1b[0m: {:#010x}   \x1b[1;33ms11\x1b[0m: {:#010x}   \x1b[1;33mt3\x1b[0m : {:#010x}\n\
             \x1b[1;33mt4\x1b[0m : {:#010x}   \x1b[1;33mt5\x1b[0m : {:#010x}   \x1b[1;33mt6\x1b[0m : {:#010x}\n\n",

            self.ra, self.sp, self.gp, self.tp,
            self.t0, self.t1, self.t2, self.s0,
            self.s1, self.a0, self.a1, self.a2,
            self.a3, self.a4, self.a5, self.a6,
            self.a7, self.s2, self.s3, self.s4,
            self.s5, self.s6, self.s7, self.s8,
            self.s9, self.s10, self.s11, self.t3,
            self.t4, self.t5, self.t6
        ));
    }
}