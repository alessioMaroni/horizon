# Stack Guard Protection Test

This document outlines the architecture, trigger mechanism, and expected behavior for validating the **PMP Stack Guard Protection** in the RISC-V kernel.

> For the detailed exception report and trap vector assembly dump, refer to [Trap Handler Test Section 1.9](../../trap/exeptions_tests.md).

---

## 1. Overview & Purpose

The Physical Memory Protection (**PMP**) Stack Guard is designed to prevent stack overflow conditions from silently corrupting critical kernel memory regions (`.bss`, `.data`, or adjacent task stacks).

By hardware-locking a zero-permission guard page (`pmp0cfg = 0x98`: `R=0, W=0, X=0, L=1`) directly adjacent to the active kernel stack, any store operation into this memory boundary is immediately intercepted by the CPU memory controller before RAM is modified.

---

## 2. Test Architecture & Setup

### 2.1 Hardware Configuration
* **Target Address:** `_stack_guard_start` (e.g., `0x80200000`).
* **PMP Entry 0 (`pmpcfg0`):** Configured as NAPOT / TOR with `0x98` flags (Locked in M-Mode, No Read, No Write, No Execute).
* **Guard Size:** 512 Bytes.

### 2.2 Trigger Mechanism
The test executes `trigger_0x7_store_access_fault()`, which performs an inline assembly store operation (`sw`) targeting the protected guard page pointer:

```rust
extern "C" {
    static _stack_guard_start: u8;
}

/// Triggers Exception 0x7: Store/AMO Access Fault via PMP Guard violation.
#[inline(always)]
pub fn trigger_0x7_store_access_fault() {
    unsafe {
        let guard_ptr = core::ptr::addr_of!(_stack_guard_start) as *mut u32;
        core::arch::asm!(
            "sw x0, 0({addr})",
            addr = in(reg) guard_ptr,
        );
    }
}
```

---

## 3. Execution Flow

1. **Kernel Boot & PMP Init:** Kernel initializes console and calls `setup_pmp_stack_guard()`.
2. **Test Trigger:** Under `#[cfg(feature = "tests")]`, the kernel invokes `trigger_0x7_store_access_fault()`.
3. **Hardware Fault:** The CPU attempts `sw x0, 0(reg)`. The PMP unit blocks the write bus cycle and triggers an immediate trap.
4. **Trap Interception:** Control transfers to `trap_vector`, saving the execution context to a `TrapFrame`.

---

## 4. Expected Results & Verification Matrix

During execution, the test **MUST** strictly fulfill the following hardware and software contracts:

### 4.1 System Behavior Matrix

| Layer | Responsible Subsystem | Expected Outcome |
| :--- | :--- | :--- |
| **Bus / Memory** | Hardware PMP Unit | Cancels store transaction. **RAM modification is strictly prevented**. |
| **CPU Core** | Decoder / Exception Vector | Raises Exception `0x7` (`Store/AMO Access Fault`). Sets `mepc` to `sw` instruction address and `mtval` to `0x80200000`. |
| **Trap Handler** | `h_store_amo_fault.rs` | Captures trap, decodes `mcause = 0x7`, and logs diagnostic register report. |
| **Kernel Subsystem** | `panic!` | Invokes fatal panic handler, halting CPU execution safely. |

---

### 4.2 Expected Console Log Output

Running the test suite **MUST** produce serial log output structurally matching the following dump:

```text
Message 1

[TRAP TRIGGERED] Type: EXCEPTION | Code: 0x00000007 | mepc: 0x800003d8

Type         : EXCEPTION
Cause Code   : 0x00000007 (Store/AMO Access Fault)
Program Ctr  : 0x800003d8 (mepc)
Target Value : 0x80200000 (mtval: BadAddr or Opcode)
CPU Status   : 0x00001800 (mstatus)

REGISTER DUMP:
mcause: 0x00000007 | mepc: 0x800003d8
ra : 0x8000047a   sp : 0x81ffff80   gp : 0x00000000   tp : 0x00000000
t0 : 0x80200000   t1 : 0xffffff00   t2 : 0x00000000   s0 : 0x00000000
s1 : 0x00000000   a0 : 0x10000000   a1 : 0x0000004d   a2 : 0x00000065
a3 : 0x00000073   a4 : 0x00000061   a5 : 0x00000067   a6 : 0x00000020
a7 : 0x0000000a   s2 : 0x00000000   s3 : 0x00000000   s4 : 0x00000000
s5 : 0x00000000   s6 : 0x00000000   s7 : 0x00000000   s8 : 0x00000000
s9 : 0x00000000   s10: 0x00000000   s11: 0x00000000   t3 : 0x00000000
t4 : 0x00000000   t5 : 0x00000000   t6 : 0x00000000

[CRITICAL!] Store Access Fault (Write/AMO Violation)!

[PANIC!] Kernel panic!
panicked at src/trap/exeptions/h_store_amo_fault.rs:27:5:
Security/Memory Fault (Store/AMO Access Fault):
- Faulting Instruction (mepc): 0x800003d8
- Invalid Write Target  (mtval): 0x80200000
```

---

## 5. Pass / Fail Verification Checklist

To mark the test as **PASSED**, verify all criteria below:

- [ ] **Cause Code Validation:** `mcause` strictly equals `0x00000007`.
- [ ] **Address Match:** `mtval` strictly matches `_stack_guard_start` address (`0x80200000`).
- [ ] **Execution Block:** Subsequent code execution (e.g., `"Message 2"`) is **NEVER** reached.
- [ ] **Memory Integrity:** Memory adjacent to `_stack_guard_start` remains unaltered.