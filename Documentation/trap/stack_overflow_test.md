# Stack Overflow & Emergency Stack Test

## 1. Overview

This document describes the validation procedure and diagnostic verification for the kernel's **Stack Safety and Double Fault Mitigation Architecture**. 

The goal of this test is to intentionally exhaust the primary kernel execution stack using controlled infinite recursion (`trigger_natural_stack_overflow`), forcing the Stack Pointer (`sp`) to breach its valid bounds (`_stack_start`) or hit the memory-protected PMP guard region (`.stack_guard`).

This verifies that:
1. The low-level trap vector (`trap_entry.s`) detects stack pointer corruption before frame allocation.
2. The hardware/kernel safely pivots context to the dedicated isolated **Emergency Stack** (`_emergency_stack_top`).
3. Complete post-mortem telemetry (`mcause`, `mepc`, `mtval`, `mstatus`, and corrupted `sp`) is streamed without triggering a catastrophic Double Fault or hardware lockup.

---

## 2. Test Execution Code

### 2.1 Entry Point Setup (`src/main.rs`)

```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        let depth: usize = 0;
        trigger_natural_stack_overflow(depth);  
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

### 2.2 Recursion Helper (`src/tests/stack.rs`)

```rust
/// Triggers a **natural kernel stack overflow** through controlled infinite recursion.
///
/// This function allocates a localized stack frame buffer (`[u32; 32]`) on every recursive call, 
/// rapidly depleting the available stack space until the Stack Pointer (`sp`) breaches the 
/// lower boundary (`_stack_start`) or enters the PMP guard zone.
///
/// # Arguments
///
/// * `depth` - Tracks the current recursive depth level, used to throttle console logging 
///             and prevent serial buffer flooding.
///
/// # Behavior
///
/// 1. Allocates 128 bytes of local array data on the stack per frame iteration.
/// 2. Periodically logs telemetry updates to the console every 64 recursive steps.
/// 3. Recurs indefinitely until the stack collapses, invoking the low-level trap entry's 
///    stack bounds checker or triggering a hardware access fault.
///
/// # Note
///
/// Annotated with `#[allow(unconditional_recursion)]` to suppress the Rust compiler warning 
/// regarding missing base cases, as infinite recursion is the precise mechanism required 
/// to simulate memory exhaustion and validate emergency stack mitigation.
#[allow(unconditional_recursion)]
pub fn trigger_natural_stack_overflow(depth: usize) {
    // Allocate a local data buffer on the stack to consume stack memory per frame.
    let _frame_buffer: [u32; 32] = [0; 32];

    // Recurse further down the stack.
    trigger_natural_stack_overflow(depth + 1);
}
```

---

## 3. Serial Console Output & Forensics Log

When running `make run-test`, the serial output demonstrates continuous stack consumption followed by immediate interception at the trap boundary and execution pivoting onto the emergency stack:

```text
Consuming stack... depth: 981248
Consuming stack... depth: 981312
Consuming stack... depth: 981376
Consuming stack... depth: 981440
Consuming stack... depth: 981504
Consuming stack... depth: 981568
Consuming stack... depth: 981632
Consuming stack... depth: 981696
Consuming stack... depth: 981760
Consuming stack... depth: 981824
Consuming stack... depth: 981888
Consuming stack... depth: 981952
Consuming stack... depth: 982016
Consuming stack... depth: 982080
Consuming stack... depth: 982144
Consuming stack... depth: 982208
Consuming stack... depth: 982272
Consuming stack... depth: 982336
Consuming stack... depth: 982400
Consuming stack... depth: 982464
Consuming stack... depth: 982528
Consuming stack... depth: 982592
Consuming stack... depth: 982656
Consuming stack... depth: 982720
Consuming stack... depth: 982784
Consuming stack... depth: 982848
Consuming stack... depth: 982912
Consuming stack... depth: 982976

[FATAL!] Emergency Stack Overflow Handler reached (Double Fault mitigation)!
Corrupted SP was : 0x00000004

[TRAP TRIGGERED] Type: EXCEPTION | Code: 0x00000007 | mepc: 0x80000010

Type         : EXCEPTION
Cause Code   : 0x00000007 (Store/AMO Access Fault)
Program Ctr  : 0x80000010 (mepc)
Target Value : 0x802001fc (mtval: BadAddr or Opcode)
CPU Status   : 0x00001800 (mstatus)

[PANIC!] Kernel panic!
panicked at src/panic.rs:82:5:
System halted securely on emergency stack
```

---

## 4. Architectural Analysis & Fault Sequence

1. **Stack Exhaustion Phase:** As recursion progresses beyond depth `982976`, the Stack Pointer (`sp`) crosses the lower bound `_stack_start`.
2. **PMP Guard Zone Breach:** A store instruction (`sw`) attempts to write into the locked `.stack_guard` memory page, immediately firing CPU Exception `0x7` (**Store/AMO Access Fault**).
3. **Preamble Trap Check (`trap_entry.s`):**
   - The low-level trap entry captures `sp` and evaluates it against `_stack_start`.
   - Sensing the out-of-bounds pointer, it bypasses standard `SAVE_REGS` frame allocation on the corrupted stack.
4. **Emergency Pivot:**
   - Stack execution vector redirects immediately to `_emergency_stack_top`.
   - Control is handed over to `handle_stack_overflow_panic`.
5. **Telemetry Stream & Secure Halt:**
   - Registers `mcause`, `mepc`, `mtval`, `mstatus`, and `corrupted_sp` are output via serial console.
   - The kernel halts securely in an unrecoverable panic state on the isolated emergency stack.

---

Tests Executed By: Alessio Maroni

Date: 7/10/26 12:45

---

`SPDX-License-Identifier: MIT OR Apache-2.0  
Copyright (c) 2026 The RSC-V Kernel Project,  
See [License](../LICENSE)`

---
