/// Configures `pmpaddr0` and `pmpcfg0` to establish a zero-permission PMP Guard Page.
///
/// This function programs PMP Entry 0 in **NAPOT** (Naturally Aligned Power-Of-Two) mode 
/// and locks it (`L = 1`) so that the rules are strictly enforced even in Machine Mode (M-mode).
///
/// # Arguments
///
/// * `guard_base` - The physical base address of the guard page. Must be aligned to `guard_size`.
/// * `guard_size` - The size of the guard region in bytes. Must be a power of two $\ge 8$.
///
/// # Panics / Debug Assertions
///
/// Under debug builds, this function asserts that:
/// 1. `guard_size` is a power of two and $\ge 8$ bytes.
/// 2. `guard_base` is naturally aligned to `guard_size` (`guard_base % guard_size == 0`).
///
/// # Safety
///
/// This function executes inline assembly to write directly to hardware Control and Status 
/// Registers (`pmpaddr0` and `pmpcfg0`).
///
/// - The caller must ensure that `guard_base` and `guard_size` cover a memory region that 
///   contains no valid kernel data or executable code.
/// - Once configured, setting the **Lock (`L`) bit** renders `pmpaddr0` and `pmpcfg0[7..0]` 
///   read-only until the next hard SoC reset.
pub fn setup_pmp_stack_guard(guard_base: usize, guard_size: usize) {
    debug_assert!(guard_size.is_power_of_two() && guard_size >= 8);
    debug_assert!(
        guard_base % guard_size == 0,
        "guard_base must be naturally aligned to guard_size"
    );

    // Encode the address and region size into RISC-V NAPOT format.
    // Right-shifting by 2 converts byte address to 32-bit word address.
    // Masking with ((guard_size >> 3) - 1) encodes the size in trailing ones.
    let pmpaddr_val = (guard_base >> 2) | ((guard_size >> 3) - 1);

    // PMP0 Configuration Byte: 0x98 (0b1001_1000)
    // Bit 7   : L = 1 (Locked, enforced on M-mode)
    // Bit 4..3: A = 11 (NAPOT Addressing Mode)
    // Bit 2   : X = 0 (No Execute)
    // Bit 1   : W = 0 (No Write)
    // Bit 0   : R = 0 (No Read)
    let pmp0cfg: u8 = 0x98;

    unsafe {
        core::arch::asm!(
            // 1. Store NAPOT value into pmpaddr0
            "csrw pmpaddr0, {addr}",
            
            // 2. Read current pmpcfg0 register using the safe placeholder
            "csrr {temp0}, pmpcfg0",
            
            // 3. Clear bits 0..7 while preserving PMP1..PMP3
            "li   {temp1}, ~0xFF",
            "and  {temp0}, {temp0}, {temp1}",
            
            // 4. Merge new PMP0 configuration and write back
            "or   {temp0}, {temp0}, {cfg}",
            "csrw pmpcfg0, {temp0}",
            
            addr   = in(reg) pmpaddr_val,
            cfg    = in(reg) pmp0cfg as usize,
            temp0  = lateout(reg) _,
            temp1  = lateout(reg) _,
        );
    }
}