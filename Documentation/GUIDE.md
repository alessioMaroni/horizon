# RSC-V Documentation Guide
``(guide)`` ``(initial)``

Welcome to the **RSC-V** (Rust-SPARK Critical Kernel for RISC-V) documentation repository. This guide provides an overview of how the project's technical specifications, architectural decisions, and source code are documented and verified.

---

## 1.0 Scope of Documentation

Inside this repository, you will find:

* **Architectural Decisions:** Design philosophy, memory safety model, FFI bridge specifications, and RISC-V Hardware Abstraction Layer (HAL) decisions.
* **Implementation Details:** Exhaustive breakdowns of kernel subsystems, scheduling logic, driver implementations, and inter-language contracts.
* **Verification & Test Artifacts:** Formal proof reports (from `gnatprove` and `kani`), simulation results (OpenRocket/HIL), and execution logs.

---

## 2.0 Navigating the Documentation

The project documentation is divided into two main layers: **high-level spec files** and **in-code formal annotations**.

### 2.1 Specification Files (`.md`)

High-level documentation is written in Standard Markdown and stored in the [`/Documentation`](./) directory.

#### 2.1.1 Finding Specific Files
To find specific files inside the [`/Documentation`](./) directory, you can use the [`INDEX`](./INDEX.md) file located inside the directory. You will find direct links to folders and to every documentation file (`.md`).

#### 2.1.2 File Model
This is a description of the structure of a [`/Documentation`](./) file:

<!-- TODO: Once files struct is bether defined update the file example -->
```md
# Title                                         <!-- Title of the page -->
``docs`` ``example``                            <!-- Flags that locate the file in a category of the documentation -->

Basic file description                          <!-- A description of what the file documents -->

## Subsections 1                                <!-- Subsections of the file -->
### Subsubsection 2

`SPDX-License-Identifier: MIT OR Apache-2.0
Copyright (c) 2026 The RSC-V Kernel Project,
See [License](../LICENSE)`
```

Subsections provide detailed explanations of specific decisions, components, or executed tests. Note that if a file covers testing, the bottom section will include links to test results, details about conducted tests, and code examples.

### 2.2 Code & In-Line Annotations

Source code comments serve as both operational documentation and inputs for formal verification tools.

---

`SPDX-License-Identifier: MIT OR Apache-2.0  
Copyright (c) 2026 The RSC-V Kernel Project,  
See [License](../LICENSE)`