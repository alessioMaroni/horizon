# PMP Configuration Byte (`pmpcfg0[7..0] = 0x98`)

| Bit Range | Field | Value | Binary | Description |
| :---: | :---: | :---: | :---: | :--- |
| **7** | **L** | `1` | `1` | **Locked:** Enforces protection rules in Machine Mode (M-mode) and locks configuration until system reset. |
| **6..5** | *Reserved* | `0` | `00` | Unused in RISC-V privileged specification. |
| **4..3** | **A** | `3` | `11` | **NAPOT Mode:** Address matching based on natural power-of-two alignment. |
| **2** | **X** | `0` | `0` | **No Execute:** Instruction fetch causes Instruction Access Fault (`mcause = 1`). |
| **1** | **W** | `0` | `0` | **No Write:** Store instruction causes Store Access Fault (`mcause = 7`). |
| **0** | **R** | `0` | `0` | **No Read:** Load instruction causes Load Access Fault (`mcause = 5`). |

### NAPOT Address Calculation (`pmpaddr0`)

For a guard page at base address `guard_base` with power-of-two size `guard_size`:

$$\text{pmpaddr0} = (\text{guard\_base} \gg 2) \mid ((\text{guard\_size} \gg 3) - 1)$$

---