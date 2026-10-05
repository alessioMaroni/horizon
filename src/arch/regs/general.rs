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
        CONSOLE.write_fmt(format_args!("REGISTER DUMP:\n"));
        
        if let (Some(cause), Some(epc)) = (mcause, mepc) {
            CONSOLE.write_fmt(format_args!("mcause: {:#010x} | mepc: {:#010x}\n", cause, epc));
        }

        CONSOLE.write_fmt(format_args!(
            "ra : {:#010x}   sp : {:#010x}   gp : {:#010x}   tp : {:#010x}\n\
             t0 : {:#010x}   t1 : {:#010x}   t2 : {:#010x}   s0 : {:#010x}\n\
             s1 : {:#010x}   a0 : {:#010x}   a1 : {:#010x}   a2 : {:#010x}\n\
             a3 : {:#010x}   a4 : {:#010x}   a5 : {:#010x}   a6 : {:#010x}\n\
             a7 : {:#010x}   s2 : {:#010x}   s3 : {:#010x}   s4 : {:#010x}\n\
             s5 : {:#010x}   s6 : {:#010x}   s7 : {:#010x}   s8 : {:#010x}\n\
             s9 : {:#010x}   s10: {:#010x}   s11: {:#010x}   t3 : {:#010x}\n\
             t4 : {:#010x}   t5 : {:#010x}   t6 : {:#010x}\n\n\
             ",

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