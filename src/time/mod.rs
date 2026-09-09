// Copyright (c) 2026 NewHorizon Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::arch::asm;

#[repr(C, packed)]
pub struct IdtEntry {
    pointer_low: u16,
    gdt_selector: u16,
    ist_options: u16,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

#[inline]
pub fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;

    unsafe {
        asm!(
            "rdtsc",

            out("eax") low,
            out("edx") high,
            
            options(nomem, nostack)
        );
    }

    ((high as u32) << 32) | (low as u32)
}

#[inline]
pub fn wrmsr(msr: u32, val: u64) {
    let low: u32 = val as u32;
    let high: u32 = (val >> 32) as u32;

    unsafe {
        asm!(
            "wrmsr",

            in("ecx") msr,
            in("eax") low,
            in("edx"), high,
            
            options(nomem, nostack)
        );
    }
}

impl IdtDescriptor {
    pub unsafe fn load(&self) {
        asm!("lidt [{}]", in(reg) self, options(readonly, nostack, preserves_flags));
    }
}

pub unsafe fn apic_write(base: usize, offset: usize, value: u32) {
    let reg = (base + offset) as *mut u32;
    core::ptr::write_volatile(reg, value);
}
