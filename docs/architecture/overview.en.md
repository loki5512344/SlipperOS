# SlipperOS Architecture

SlipperOS is a monolithic kernel running in RISC-V S-mode.
Bootloader in C++, kernel in Rust, userspace C (picolibc) — see language_policy.

## Components

- **boot** — _start entry, bss zeroing, stack
- **SlipperBoot** — disk bootloader (ELF parser, FDT, boot.cfg) — **v0.2, not implemented**
- **kernel** — initialization, panic handler
- **drivers** — UART, CLINT, PLIC, VirtIO (block + GPU)
- **mm** — bump allocator, page allocator, Sv39
- **proc** — tasks, round-robin, context
- **fs** — slipfs (block filesystem)
- **shell** — slip shell (UART-based CLI)
- **compositor** — Wayland + GPU (separate addon, not part of kernel)

## Control flow

```
OpenSBI → _start → [SlipperBoot] → kernel_main → shell_start
```
