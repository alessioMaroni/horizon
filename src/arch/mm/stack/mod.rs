// TODO: Document

pub mod guard;

unsafe extern "C" {
    /// Linker symbol indicating the start address of the PMP Guard Page.
    ///
    /// # Realignment & Sizing
    /// This symbol must be aligned to `guard_size` in the linker script to satisfy 
    /// the RISC-V NAPOT (Naturally Aligned Power-Of-Two) requirement.
    pub static _stack_guard_start: u8;

    /// Linker symbol indicating the end address of the PMP Guard Page.
    pub static _stack_guard_end: u8;
}
