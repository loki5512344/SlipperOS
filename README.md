# SlipperOS!

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Arch: RISC-V 64](https://img.shields.io/badge/arch-RISCV64-green)](https://riscv.org)
[![Status](https://img.shields.io/badge/status-development-yellow)]()
[![Rust](https://img.shields.io/badge/rust-nightly_2026--06--01-orange)](https://rust-lang.org)
[![Lines](https://img.shields.io/badge/lines-3500%2B-blue)]()
[![OC2r](https://img.shields.io/badge/target-OC2r-8A2BE2)]()

A minimalist RISC-V 64-bit kernel written in Rust. Monolithic design, S-mode, built for [OC2r](https://github.com/loki5512344/SlipperOS).

---

## Quick Start

```bash
# Build
cargo build --release

# Run in QEMU
qemu-system-riscv64 \
  -machine virt -m 128M -nographic \
  -bios default \
  -kernel target/riscv64gc-unknown-none-elf/release/slipperos
```

Once booted, type `help` in the shell.

---

## Boot Flow

```
 OpenSBI (M-mode)
     |
 [boot/boot.S]      ← temporary, replaced by SlipperBoot
     |
 SlipperOS (S-mode, Rust)
  |-- [kernel/main.rs]    UART, PLIC, CLINT init
  |-- [kernel/lib.rs]     module wiring
  |-- [kernel/mm/]        bump + page allocator + Sv39
  |-- [kernel/proc/]      tasks, scheduler, context switch
  |-- [kernel/drivers/]   UART, VirtIO block
  |-- [kernel/fs/]        SlipFS block filesystem
  +-- [kernel/shell/]     slip shell (UART CLI)
```

---

## Components

| Component | Status | Source |
|-----------|--------|--------|
| UART (NS16550A) | done | [kernel/drivers/uart.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/drivers/uart.rs) |
| CLINT timer | done | [kernel/drivers/clint.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/drivers/clint.rs) |
| PLIC interrupts | done | [kernel/drivers/plic.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/drivers/plic.rs) |
| Bump allocator | done | [kernel/mm/bump.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/mm/bump.rs) |
| Page allocator | done | [kernel/mm/page.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/mm/page.rs) |
| Sv39 page tables | done | [kernel/mm/map.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/mm/map.rs) |
| Round-robin scheduler | wip | [kernel/proc/sched.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/proc/sched.rs) |
| Context switch | wip | [kernel/proc/context.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/proc/context.rs) |
| VirtIO block | wip | [kernel/drivers/virtio/block.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/drivers/virtio/block.rs) |
| SlipFS | planned | [kernel/fs/slipfs.rs](https://github.com/loki5512344/SlipperOS/blob/main/kernel/fs/slipfs.rs) |
| Trap handler | planned | [docs/kernel/interrupts.html](docs/kernel/interrupts.html) |

---

## Repository

- [`boot/`](https://github.com/loki5512344/SlipperOS/tree/main/boot) — boot.S + linker script
- [`bootloader/`](https://github.com/loki5512344/SlipperOS/tree/main/bootloader) — planned C++ bootloader
- [`kernel/`](https://github.com/loki5512344/SlipperOS/tree/main/kernel) — Rust kernel
  - `drivers/` — UART, CLINT, PLIC, VirtIO
  - `mm/` — memory management
  - `proc/` — tasks and scheduler
  - `fs/` — filesystem
  - `shell/` — slip CLI
- [`docs/`](https://github.com/loki5512344/SlipperOS/tree/main/docs) — documentation site
- [`build-docs.sh`](build-docs.sh) — HTML generator

## License

GPL-3.0-or-later
