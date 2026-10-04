# Exceptions Table

Exceptions occur for a variety of reasons. The `mcause` register indicates the specific reason for the latest exception.

| Cause (`mcause`) | Exception Name | Meaning / Description |
| :---: | :--- | :--- |
| **`0x0`** | Instruction Address Misaligned | Does not occur on RP2350 because 16-bit compressed instructions (RVC) are implemented, making it impossible to jump to an unaligned byte address. |
| **`0x1`** | Instruction Fetch Fault | Attempted to fetch from an address that does not support instruction fetch (e.g., APB/AHB peripherals on RP2350), lacks PMP execute permission, is forbidden by `ACCESSCTRL`, or returned a hardware fault from the memory device itself. |
| **`0x2`** | Illegal Instruction | Encountered an opcode not implemented by this processor, attempted to access a non-existent CSR, or attempted to execute a privileged instruction/CSR access without sufficient privilege. |
| **`0x3`** | Breakpoint | An `ebreak` or `c.ebreak` instruction was executed, and no external debug host caught it (`DCSR.EBREAKM` or `DCSR.EBREAKU` was not set). |
| **`0x4`** | Load Address Misaligned | Attempted to load data from an address that was not aligned to the access size (e.g., 32-bit load from a non-word boundary). |
| **`0x5`** | Load Fault | Attempted to load from an address that does not exist, lacks PMP read permissions, is forbidden by `ACCESSCTRL`, or returned a fault from a peripheral. |
| **`0x6`** | Store/AMO Address Misaligned | Attempted to write data to an address that was not aligned to the access size. |
| **`0x7`** | Store/AMO Fault | Attempted to write to an address that does not exist, lacks PMP write permissions, is forbidden by `ACCESSCTRL`, or returned a peripheral fault. Also raised when attempting an Atomic Memory Operation (AMO) on an address that does not support AHB5 exclusives. |
| **`0x8`** | Environment Call from U-mode | An `ecall` instruction was executed while operating in User Mode. |
| **`0xB`** | Environment Call from M-mode | An `ecall` instruction was executed while operating in Machine Mode. |


> **Note:** Exceptions jump directly to the base address stored in `mtvec`, regardless of the cause code and regardless of whether vectored interrupts are enabled.

> **Source:** Adapted from Raspberry Pi RP2350 Datasheet (*RP-008373-DS-2*), Section *3.8.4.1. Exceptions*.