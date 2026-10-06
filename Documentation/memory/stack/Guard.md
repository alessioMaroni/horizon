# RISC-V Kernel Stack Guard

`Docs` `Memory` `Stack` `PMP` `Safety`

## 1. Definition

A **Stack Guard** (or *Guard Page*) is a hardware-enforced memory protection mechanism designed to detect and prevent stack overflow conditions before they can silently corrupt adjacent kernel data.

In bare-metal embedded environments without a Memory Management Unit (MMU), memory stacks grow downwards (from higher to lower physical addresses). If a deep call stack or a large local buffer allocation exceeds the reserved stack space, the Stack Pointer (`sp`) crosses into lower memory regions containing global variables (`.bss`/`.data`) or task control blocks.

Using the RISC-V **Physical Memory Protection (PMP)** unit, a Stack Guard designates a naturally aligned, zero-permission memory page immediately below the stack boundary. Any load, store, or instruction fetch hitting this region instantly triggers a CPU hardware trap (`Store Access Fault` or `Load Access Fault`), halting execution safely before memory corruption occurs.

---

## 1.1 Code Links

* [**stack folder**](../../../src/arch/mm/stack)
* [**stack guard (rs)**](../../../src/arch/mm/stack/guard.rs)
* [**linker script (ld)**](../../../RISC-V-linker.ld)

---

## 2. System Behavior & Failure Modes

### 2.1 Without the Guard Page

If a function performs excessive **recursive calls or allocates a large stack buffer** without protection:

```text
[RAM Memory Map]
0x20042000 ┌────────────────────────┐
           │ Kernel Stack           │
           │                        │
           │    │                   │
           │    ▼ (sp decreases)    │
0x20040200 ├────────────────────────┤
           │ Global Data / .bss     │  <-- Stack overflows into this region!
           │ (Kernel State Data)    │  <-- SILENT CORRUPTION of memory.
0x20040000 └────────────────────────┘      System crashes or exhibits undefined behavior.
```

### 2.2 With the PMP Guard Page

The PMP hardware acts as an invisible, hardware-locked boundary:

When the Stack Pointer (`sp`) enters the Guard Page and executes a memory store instruction (e.g., `sw ra, 0(sp)`):

1. **Bus Write Cancellation:** The CPU hardware cancels the memory write before modifying RAM.
2. **Hardware Trap Generation:** The CPU immediately raises a hardware trap (`mcause = 0x7`: Store Access Fault).
3. **Safe Interception:** The Kernel Trap Handler catches the exception, logs diagnostic registers, and panics safely without data corruption.

```text
[RAM Memory Map]
0x20042000 ┌────────────────────────┐
           │ Kernel Stack           │
           │    │                   │
           │    ▼ (push / sw)       │
0x20040200 ├────────────────────────┤
           │ PMP Guard Page (0x98)  │  <-- TRAP! Hardware intercepts 'sw'
0x20040000 ├────────────────────────┤      BEFORE memory write reaches RAM.
           │ Global Data / .bss     │  <-- Memory remains pristine and uncorrupted.
           └────────────────────────┘
```

---

## 3. PMP Configuration

The Stack Guard is programmed into PMP Entry 0 using Naturally Aligned Power-Of-Two (NAPOT) addressing mode.

For detailed register bits and byte definitions, see the [**PMP Table**](pmp_table.md).