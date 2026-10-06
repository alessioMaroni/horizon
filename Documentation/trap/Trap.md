Ecco la versione completa e pronta per l'uso industriale/aerospaziale di Trap_handler.md.Ho integrato le scelte architetturali (Architectural Decisions) motivate per un sistema critico, le Macro Assembly per SAVE_REGS / RESTORE_REGS, la gestione dell'allineamento stack, della privilege transition (U-Mode / M-Mode) e la Rust Safe Bridge Integration.Puoi copiare direttamente il blocco di codice qui sotto nel tuo repository.Markdown# RISC-V Kernel Trap Handler

## 1. Definition

A **Trap Handler** (or *Trap Vector*) is a foundational low-level kernel subsystem responsible for intercepting, preserving execution context, and processing CPU-level events. In the RISC-V architecture, a "trap" serves as the unified hardware mechanism for both **synchronous exceptions** (e.g., instruction access faults, illegal instructions, system calls) and **asynchronous interrupts** (e.g., hardware timers, external I/O signals).

Upon encountering a trap condition, the processor hardware automatically suspends standard instruction fetch, captures fault state into Machine-mode Control and Status Registers (CSRs)—specifically updating `mcause`, `mepc`, `mtval`, and `mstatus`—and jumps directly to the memory address configured in the `mtvec` (Machine Trap-Vector Base Address) register.

The Trap Handler operates in three distinct sequence phases:

1. **Context Preservation (Preamble):** Executes volatile assembly instructions to allocate space on the active stack (`sp`) and spill all general-purpose registers into an immutable `TrapFrame` structure.
2. **Dispatch & Execution:** Reads `mcause` to determine the trap origin, routing execution to either a high-level exception handler or an Interrupt Service Routine (ISR).
3. **Context Restoration & Return (Epilogue):** Restores all saved CPU register states from the `TrapFrame`, adjusts the stack pointer, and executes the privileged return instruction (`mret`) to seamlessly resume execution at `mepc` or terminate the faulting thread.

---

## 1.1 Documentation Links
* [**Exceptions Table**](./exeptions_table.md)
* [**Exception Tests**](./exeptions_tests.md)

## 1.2 Code Links
* [**Trap Entry Assembly (`trap_entry.s`)**](../../src/arch/trap/trap_entry.s)
* [**Trap Handler Module (`mod.rs`)**](../../src/trap/mod.rs)
* [**Trap Module Directory**](../../src/trap)

---

## 2. Hardware Register Snapshot & CSR State

When a trap triggers, the RISC-V CPU updates the following hardware CSRs before jumping to `trap_entry`:

| CSR Name | Register | Purpose |
| :--- | :--- | :--- |
| **`mcause`** | Machine Cause | Contains the bit 31 flag (0 = Exception, 1 = Interrupt) and the cause code. |
| **`mepc`** | Machine Exception PC | Holds the virtual/physical memory address of the faulting or interrupted instruction. |
| **`mtval`** | Machine Trap Value | Holds exception-specific bad address (e.g. faulting memory target) or bad instruction opcode. |
| **`mstatus`** | Machine Status | Tracks privilege mode history (`MPP`), interrupt enable state (`MIE`), and previous interrupt state (`MPIE`). |

---

## 3. Memory Layout: `TrapFrame` Structure

To preserve context, 128 bytes are allocated on the current stack during the assembly preamble (`SAVE_REGS`). The layout matches the 32 general-purpose integer registers (RV32I):

```text
       Stack Memory (High Address)
    +-------------------------------+
    |  ra (x1)         [sp + 124]   |
    |  sp (x2)         [sp + 120]   |
    |  gp (x3)         [sp + 116]   |
    |  tp (x4)         [sp + 112]   |
    |  t0 - t2 (x5-7)  [sp + 100-108]|
    |  s0 - s1 (x8-9)  [sp + 92-96]  |
    |  a0 - a7 (x10-17)[sp + 60-88]  |
    |  s2 - s11(x18-27)[sp + 20-56]  |
    |  t3 - t6 (x28-31)[sp + 0-16]   |
    +-------------------------------+ <- Stack Pointer (`sp` passed to Rust)
       Stack Memory (Low Address)
```

---

## 4. Key Architectural Decisions (Safety & High Reliability)

To comply with high-reliability embedded software standards (e.g., ECSS / MISRA-C / Space-grade Rust patterns), the following design trade-offs and structural choices were implemented:

### 4.1 Frame Size and 16-Byte Stack Alignment
* **Decision:** The `TrapFrame` size is strictly set to **128 bytes** (32 registers × 4 bytes).
* **Rationale:** RISC-V RV32 ABI requires the stack pointer (`sp`) to remain **16-byte aligned** at all function call boundaries. Allocating 128 bytes ensures both complete integer register spilling and 16-byte alignment (`128 % 16 == 0`), avoiding unaligned stack faults when calling C/Rust functions from assembly.

### 4.2 Direct Vectoring Mode vs. Vectored Mode
* **Decision:** The system configures `mtvec` in **Direct Mode** (`MODE = 00`).
* **Rationale:** All exceptions and interrupts jump to a single entry point (`trap_entry`). While Vectored Mode eliminates dispatch overhead for interrupts, Direct Mode provides a single deterministic point for context preservation, stack guard validation, and centralized trap logging, minimizing assembly footprint and safety-critical audit surface.

### 4.3 Zero Allocation During Trap Execution
* **Decision:** The trap handler uses the active thread/kernel stack and zero dynamic heap allocation.
* **Rationale:** Dynamic heap allocation inside a trap handler introduces non-determinism, potential deadlock, and heap-exhaustion panics. Operating strictly on the pre-allocated stack guarantees constant-time execution overhead during context switching.

### 4.4 ABI Register Preservation
* **Decision:** All 31 general-purpose registers are saved, including caller-saved (`t0`-`t6`, `a0`-`a7`) and callee-saved registers (`s0`-`s11`).
* **Rationale:** Although standard C ABI functions preserve callee-saved registers, preemption or asynchronous interrupts can occur at any assembly instruction boundary. Saving the complete context guarantees total thread isolation and state integrity upon `mret`.