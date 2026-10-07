# Trap Handler Module Test Suite

`Tests` | `Trap Handler` | `Exceptions` | `Interrupts`

This document details the test procedures designed to validate the RISC-V kernel's [**Trap Handler**](./Trap_handler.md). 

The test suite is structured into two main sections:
1. **Exception Handling Tests** (`0x0`–`0xB` cause codes triggered via hardware assembly)
2. **Interrupt Handling Tests** (Asynchronous timer, software, and external interrupts)

* **[Code](../../src/trap/mod.rs)**
* **[Docs](./Trap_handler.md)**

---

## **1.0** Exception Handling Tests

The test suite is executed by building the kernel with the `tests` feature flag enabled. This includes the external `tests` workspace crate, providing specialized debugging routines and testing functions to evaluate kernel trap behavior.

> **Execution Strategy Note:** Recoverable exceptions are tested first to verify that execution can safely resume. Unrecoverable exceptions are executed last, as they deliberately trigger a kernel [**panic!**](../../src/panic.rs) to halt execution.

The following section presents the execution results, register states, and specifications for each exception test case.

* **[Test Suite Source Code](../../tests/src/trap/mod.rs)**
* **[Exceptions Reference Table](./exceptions_table.md)**

#### **1.1** Kernel Vector Setup
The **trap vector initialization sequence** executed prior to test invocation:

```rust
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
```

---

### **1.2** Instruction Address Misaligned (0x0)
#### **1.2.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x0`: **Instruction Address Misaligned**.
///
/// Forces an unaligned instruction fetch by executing a jump (`jr`) to an unaligned 
/// memory address (`0x2000_0001`).
#[inline(always)]
pub fn trigger_0x0_inst_misaligned() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20000001",
            "jr t0",
            out("t0") _
        );
    }
}
```
# Stack Overlow Test


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
#[cfg(feature = "tests")]
#[allow(unconditional_recursion)]
fn trigger_natural_stack_overflow(depth: usize) {
    // Allocate a local data buffer on the stack to consume stack memory per frame.
    let _frame_buffer: [u32; 32] = [0; 32];
    
    // Throttle progress reporting to the console every 64 calls to maintain simulation speed.
    if depth % 64 == 0 {
        CONSOLE.write_fmt(format_args!("Consuming stack... depth: {}\n", depth));
    }

    // Recurse further down the stack.
    trigger_natural_stack_overflow(depth + 1);
}
```
#### **1.2.2** Main Setup

The main adaptation to the test for **Instruction Address Misaligned `(0x0)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x0_inst_misaligned();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.2.3** Expected Results & Analysis

An unaligned instruction fetch or invalid program execution path represents a critical control-flow violation. Because the CPU core cannot safely decode or fetch valid machine instructions from an invalid memory boundary, this condition is classified as an **unrecoverable fatal exception**.

Upon trapping, the system behavior must strictly adhere to the following execution contract:

* **Context Capture:** The low-level assembly trap vector captures the execution context and dumps a comprehensive `TrapFrame` diagnostic report to the system console.
* **Panic Escalation:** Control passes to the designated exception handler, which determines that the faulting instruction pointer (`mepc`) cannot be safely advanced.
* **Controlled System Termination:** The handler invokes the kernel [`panic!`](../../src/panic.rs) subsystem, halting CPU execution and preventing instruction stream corruption or erratic execution.

#### **1.2.4** Test Result
```
Message 1
[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x00000002 (Illegal Instruction)
 Program Ctr  : 0x20000000 (mepc)
 Target Value : 0x00000000 (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x8000041a  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x20000001  T1 : 0x0000000a  T2 : 0x00000000  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

[CRITICAL!] Illegal Instruction Exception
[PANIC!] Kernel panic! panicked at src/trap/exeptions/h_illegal_inst.rs:25:5:
Execution Fault (Illegal Instruction):
- Faulting Instruction Address (mepc): 0x20000000
- Raw Instruction / Bad CSR Opcode (mtval): 0x00000000
```

---

### **1.3** Instruction fetch fault (0x1)
#### **1.3.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x1`: **Instruction Access Fault**.
///
/// Forces a execution access fault by jumping (`jr`) to an unmapped or restricted 
/// memory address (`0x4000_0000`).
#[inline(always)]
pub fn trigger_0x1_inst_access_fault() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x40000000",
            "jr t0",
            out("t0") _
        );
    }
}
```

#### **1.3.2** Main Setup

The main adaptation to the test for **Instruction fetch fault `(0x1)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x1_inst_access_fault();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.3.3** Expected Results & Analysis

Attempting to fetch an instruction from an unmapped, protected, or non-executable memory region (`0x4000_0000`) violates the processor's Memory Protection Unit (MPU) or Physical Memory Protection (PMP) rules. Because the CPU cannot legitimately retrieve executable code from such a space, this condition is classified as an **unrecoverable fatal exception (Instruction Access Fault)**.

Upon trapping, the system behavior must strictly adhere to the following execution contract:

* **Context Capture:** The low-level assembly trap vector captures the execution context and dumps a comprehensive `TrapFrame` diagnostic report to the system console, highlighting the faulting program counter (`mepc` at `0x40000000`) and target value (`mtval`).
* **Panic Escalation:** Control passes to the designated exception handler, which evaluates the `mcause` code (`0x1`) and determines that execution cannot safely proceed or be retried.
* **Controlled System Termination:** The handler invokes the kernel [`panic!`](../../src/panic.rs) subsystem, safely halting CPU execution to prevent security breaches, memory bus faults, or erratic kernel behavior.

#### **1.3.4** Test Result
```
Message 1
[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x00000002 (Illegal Instruction)
 Program Ctr  : 0x40000000 (mepc)
 Target Value : 0xffffffff (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x80000418  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x40000000  T1 : 0x0000000a  T2 : 0x00000000  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

[CRITICAL!] Illegal Instruction Exception
[PANIC!] Kernel panic! panicked at src/trap/exeptions/h_illegal_inst.rs:25:5:
Execution Fault (Illegal Instruction):
- Faulting Instruction Address (mepc): 0x40000000
- Raw Instruction / Bad CSR Opcode (mtval): 0xffffffff
```

---

### **1.4** Illegal instruction (0x2)
#### **1.4.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x2`: **Illegal Instruction**.
///
/// Emits an invalid 32-bit opcode (`0x00000000`), forcing the CPU decoder to fault.
#[inline(always)]
pub fn trigger_0x2_illegal_instruction() {
    unsafe {
        core::arch::asm!(".4byte 0x00000000");
    }
}
```

#### **1.4.2** Main Setup

The main adaptation to the test for **Illegal instruction `(0x2)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x2_illegal_instruction();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.4.3** Expected Results & Analysis

Encountering an invalid, reserved, or unhandled opcode (`0x2`) violates the processor instruction set architecture rules. Because the CPU cannot legitimately decode or execute an illegal instruction, this condition is classified as an **unrecoverable fatal exception (Illegal Instruction Fault)**.

Upon trapping, the system behavior must strictly adhere to the following execution contract:

* **Context Capture:** The low-level assembly trap vector captures the execution context and dumps a comprehensive `TrapFrame` diagnostic report to the system console, highlighting the faulting program counter (`mepc`) and the raw offending instruction or bad CSR opcode (`mtval`).
* **Panic Escalation:** Control passes to the designated exception handler, which evaluates the `mcause` code (`0x2`) and determines that execution cannot safely proceed or be recovered.
* **Controlled System Termination:** The handler invokes the kernel [`panic!`](../../src/panic.rs) subsystem, safely halting CPU execution with an expected kernel panic to prevent instruction stream corruption, privilege escalation, or erratic kernel behavior.

#### **1.4.4** Test Result
```
Message 1
[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x00000002 (Illegal Instruction)
 Program Ctr  : 0x800003d0 (mepc)
 Target Value : 0x00000000 (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x80000416  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x0000000a  T1 : 0x00000000  T2 : 0x00000000  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

[CRITICAL!] Illegal Instruction Exception
[PANIC!] Kernel panic! panicked at src/trap/exeptions/h_illegal_inst.rs:25:5:
Execution Fault (Illegal Instruction):
- Faulting Instruction Address (mepc): 0x800003d0
- Raw Instruction / Bad CSR Opcode (mtval): 0x00000000

```

---

### **1.5** Breakpoint (0x3)
#### **1.5.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x3`: **Breakpoint**.
///
/// Executes the hardware `ebreak` instruction to trigger a software breakpoint exception.
#[inline(always)]
pub fn trigger_0x3_breakpoint() {
    unsafe {
        core::arch::asm!("ebreak");
    }
}
```

#### **1.5.2** Main Setup

The main adaptation to the test for **Breakpoint `(0x3)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x3_breakpoint();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.5.3** Expected Results & Analysis

Executing a software breakpoint (`ebreak`) generates a synchronous exception (`0x3`). Unlike fatal hardware faults, a breakpoint is a controlled debugging trap designed to be intercepted and safely handled by the kernel without terminating execution. 

Upon trapping, the system behavior must strictly adhere to the following recovery contract:

* **Context Capture:** The low-level assembly trap vector captures the execution context and dumps a comprehensive `TrapFrame` diagnostic report to the system console, indicating cause code `0x00000003` with the program counter (`mepc`) pointing directly to the `ebreak` instruction.
* **Instruction Length Decoding:** The kernel inspects the instruction memory at `mepc` to dynamically determine whether it is a 16-bit compressed instruction (RVC) or a standard 32-bit instruction.
* **Program Counter Advancement:** The handler explicitly advances `mepc` past the breakpoint instruction and updates the `mepc` CSR.
* **Safe Resumption:** Control returns via `mret`, allowing execution to resume smoothly and proceed to subsequent instructions (e.g., printing `"Message 2"`).

#### **1.5.4** Test Result
```
Message 1
[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x00000003 (Breakpoint (ebreak))
 Program Ctr  : 0x800003d0 (mepc)
 Target Value : 0x800003d0 (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x80000414  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x0000000a  T1 : 0x00000000  T2 : 0x00000000  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

Message 2
```

---

### **1.6** Load alignment (0x4)
#### **1.6.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x4`: **Load Address Misaligned**.
///
/// Attempts to load a 32-bit word (`lw`) from an unaligned memory address (`0x2000_0001`).
#[inline(always)]
pub fn trigger_0x4_load_misaligned() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20000001",
            "lw t1, 0(t0)",
            out("t0") _,
            out("t1") _
        );
    }
}
```

#### **1.6.2** Main Setup

The main adaptation to the test for **Load alignment `(0x4)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x4_load_misaligned();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.6.3** Expected Results & Analysis

Attempting to load a 32-bit word (`lw`) from an unaligned memory address (`0x2000_0001`) theoretically corresponds to a **Load Address Misaligned (`0x4`)** exception condition.

However, on modern RISC-V architectures and environments equipped with native hardware or emulator support for unaligned accesses (such as the RP2350 Hazard3 core or standard QEMU configurations):

* **Hardware/Emulation Transparency:** Unaligned memory read operations are managed transparently by the underlying microarchitecture or runtime environment.
* **Absence of Trapping:** Because the CPU core resolves the misaligned boundary fetch without raising a fault, **no hardware exception (`mcause = 0x4`) is triggered**.
* **Seamless Execution Flow:** The instruction completes successfully (or is handled via transparent emulation), bypassing the trap handler entirely and allowing the control flow to proceed immediately to subsequent instructions (printing `"Message 2"`).

#### **1.6.4** Test Result
```
Message 1
Message 2
```

---

### **1.7** Load fault (0x5)
#### **1.7.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x5`: **Load Access Fault**.
///
/// Attempts to load a 32-bit word (`lw`) from an invalid or protected memory address (`0x0000_0000`).
#[inline(always)]
pub fn trigger_0x5_load_access_fault() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x00000000",
            "lw t1, 0(t0)",
            out("t0") _,
            out("t1") _
        );
    }
}
```

#### **1.7.2** Main Setup

The main adaptation to the test for **Load fault `(0x5)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x5_load_access_fault();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.7.3** Expected Results & Analysis

Attempting an invalid memory access or violating processor protection rules represents a critical hardware fault. Because the CPU cannot legitimately recover from or fulfill this operation, this condition is classified as an **unrecoverable fatal exception**.

Upon trapping, the system behavior must strictly adhere to the following execution contract:

* **Context Capture:** The low-level assembly trap vector captures the execution context and dumps a comprehensive `TrapFrame` diagnostic report to the system console, highlighting the faulting instruction pointer (`mepc`) and the target address or status (`mtval`).
* **Panic Escalation:** Control passes to the designated exception handler, which evaluates the `mcause` code and determines that execution cannot safely proceed or be retried.
* **Controlled System Termination:** The handler invokes the kernel [`panic!`](../../src/panic.rs) subsystem, safely halting CPU execution to prevent memory corruption, security violations, or erratic kernel behavior.

#### **1.7.4** Test Result
```
Message 1
[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x00000005 (Load Access Fault)
 Program Ctr  : 0x800003d2 (mepc)
 Target Value : 0x00000000 (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x80000418  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x00000000  T1 : 0x00000000  T2 : 0x0000000a  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

[CRITICAL!] Load Access Fault (Read Violation)
[PANIC!] Kernel panic! panicked at src/trap/exeptions/h_load_fault.rs:25:5:
Security/Memory Fault (Load Access Fault):
- Faulting Instruction (mepc): 0x800003d2
- Invalid Read Target   (mtval): 0x00000000
```

---

### **1.8** Store/AMO alignment (0x6)
#### **1.8.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x6`: **Store/AMO Address Misaligned**.
///
/// Attempts to store a 32-bit word (`sw`) to an unaligned memory address (`0x2000_0003`).
#[inline(always)]
pub fn trigger_0x6_store_misaligned() {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20000003",
            "sw x0, 0(t0)",
            out("t0") _
        );
    }
}
```

#### **1.8.2** Main Setup

The main adaptation to the test for **Store/AMO alignment `(0x6)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x6_store_misaligned();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.8.3** Expected Results & Analysis

Attempting to store a 32-bit word (`sw`) to an unaligned memory address (`0x2000_0003`) theoretically corresponds to a **Store/AMO Address Misaligned (`0x6`)** exception condition.

However, on modern RISC-V architectures and environments equipped with native hardware or emulator support for unaligned accesses (such as the RP2350 Hazard3 core or standard QEMU configurations):

* **Hardware/Emulation Transparency:** Unaligned memory write operations are managed transparently by the underlying microarchitecture or runtime environment.
* **Absence of Trapping:** Because the CPU core resolves the misaligned boundary write without raising a fault, **no hardware exception (`mcause = 0x6`) is triggered**.
* **Seamless Execution Flow:** The instruction completes successfully (or is handled via transparent emulation), bypassing the trap handler entirely and allowing the control flow to proceed immediately to subsequent instructions (printing `"Message 2"`).

#### **1.8.4** Test Result
```
Message 1
Message 2
```

---

### **1.9** Store/AMO fault (0x7)
#### **1.9.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x7`: **Store/AMO Access Fault**.
///
/// Attempts to store a 32-bit word (`sw`) directly into the hardware-locked 
/// PMP Guard Page (`_stack_guard_start`), violating the `pmp0cfg` rule (`0x98`: R=0, W=0, X=0, L=1).
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

#### **1.9.2** Main Setup

The main adaptation to the test for **Store/AMO fault `(0x7)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    trigger_0x7_store_access_fault();  

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.9.3** Expected Results & Analysis

Attempting to store data or perform an atomic memory operation (AMO) to an unmapped, protected, or non-writable memory region violates the processor's Memory Protection Unit (MPU) or Physical Memory Protection (PMP) rules. Because the CPU cannot legitimately fulfill a write operation to such an invalid space, this condition is classified as an **unrecoverable fatal exception (Store/AMO Access Fault)**.

Upon trapping, the system behavior must strictly adhere to the following execution contract:

* **Context Capture:** The low-level assembly trap vector captures the execution context and dumps a comprehensive `TrapFrame` diagnostic report to the system console, highlighting the faulting instruction pointer (`mepc`) and the offending target address (`mtval`).
* **Panic Escalation:** Control passes to the designated exception handler, which evaluates the `mcause` code (`0x7`) and determines that execution cannot safely proceed or be retried.
* **Controlled System Termination:** The handler invokes the kernel [`panic!`](../../src/panic.rs) subsystem, safely halting CPU execution to prevent memory corruption, security violations, or erratic kernel behavior.

#### **1.9.4** Test Result
```
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

`Updated on: 6/10/26 at 20:24 by Alessio Maroni`

---

### **1.10** U-mode/ M-mode ecall (0x8 - 0x9)
#### **1.10.1** Trigger Mechanism

This exception is tested using the following assembly trigger:
```rust
/// Triggers Exception `0x8`: **Environment Call from U-mode**.
///
/// Issues an `ecall` instruction while operating in User Mode.
#[inline(always)]
pub fn trigger_0x8_syscall_umode() {
    unsafe {
        core::arch::asm!("ecall");
    }
}

/// Triggers Exception `0xB`: **Environment Call from M-mode**.
///
/// Issues an `ecall` instruction while operating in Machine Mode.
#[inline(always)]
pub fn trigger_0xb_syscall_mmode() {
    unsafe {
        core::arch::asm!("ecall");
    }
}
```

#### **1.10.2** Main Setup

The main adaptation to the test for **U-mode/ M-mode ecall `(0x8 - 0x9)`**:
```rust
#[unsafe(no_mangle)]
pub fn _main() -> ! {
    CONSOLE.write_str("Message 1\n");

    #[cfg(feature = "tests")]
    {
        trigger_0x8_syscall_umode();
        trigger_0xb_syscall_mmode();
    }

    CONSOLE.write_str("Message 2\n");

    loop {}
}
```

#### **1.10.3** Expected Results & Analysis

Executing an environment call (`ecall`) generates a synchronous exception used to request services from a higher privilege level.

* **Privilege Level Behavior:** Because the kernel (`_main`) is currently executing entirely in **Machine Mode (M-mode)** and User mode (`U-mode`) has not yet been initialized or transitioned into, issuing an `ecall` from this context causes the hardware to record an **Environment Call from M-mode (`mcause = 0xB`)** in both instances. This explains why two consecutive `0xB` traps are observed instead of an `0x8` followed by an `0xB`.
* **Program Counter Advancement:** Similar to breakpoints, system calls are recoverable traps. The trap handler advances `mepc` past the 4-byte `ecall` instruction so that execution does not loop infinitely on the same instruction.
* **Safe Resumption:** Control returns via `mret`, allowing execution to resume smoothly and proceed to subsequent instructions (printing `"Message 2"`).

#### **1.10.4** Test Result
```
Message 1
[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x0000000b (Environment Call from M-mode)
 Program Ctr  : 0x800003d0 (mepc)
 Target Value : 0x00000000 (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x8000041a  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x0000000a  T1 : 0x00000000  T2 : 0x00000000  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

[DEBUG] Trap Handler!

==================== [ TRAP DIAGNOSTIC REPORT ] ====================
 Type         : EXCEPTION
 Cause Code   : 0x0000000b (Environment Call from M-mode)
 Program Ctr  : 0x800003d4 (mepc)
 Target Value : 0x00000000 (mtval: BadAddr or Opcode)
 CPU Status   : 0x00001800 (mstatus)
 --- Saved Registers (TrapFrame) ---
 RA : 0x8000041a  SP : 0x81ffff80  GP : 0x00000000  TP : 0x00000000
 A0 : 0x10000000  A1 : 0x0000004d  A2 : 0x00000065  A3 : 0x00000073
 A4 : 0x00000061  A5 : 0x00000067  A6 : 0x00000020  A7 : 0x00000031
 T0 : 0x0000000a  T1 : 0x00000000  T2 : 0x00000000  T3 : 0x00000000
 T4 : 0x00000000  T5 : 0x00000000  T6 : 0x00000000  S0 : 0x00000000
 S1 : 0x00000000  S2 : 0x00000000  S3 : 0x00000000  S4 : 0x00000000
 S5 : 0x00000000  S6 : 0x00000000  S7 : 0x00000000  S8 : 0x00000000
 S9 : 0x00000000  S10: 0x00000000  S11: 0x00000000
====================================================================

Message 2
```

---

Tests Executed By: Alessio Maroni

Date: 4/10/26 14:57 - 15:49


`SPDX-License-Identifier: MIT OR Apache-2.0  
Copyright (c) 2026 The RSC-V Kernel Project,  
See [License](../LICENSE)`

---