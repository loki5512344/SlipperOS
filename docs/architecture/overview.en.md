# Slipper Architecture

Three independent components, each in its own directory:

```
OpenSBI (M-mode)
    ↓
SlipperBoot (S-mode, C++)
    ↓
SlipperKernel (S-mode, Rust — no_std)
    ↓
SlipperOS (userspace — future)
```

---

## SlipperBoot

C++ bootloader (zero `.S` files). Runs in S-mode after OpenSBI.

**Responsibilities:**
- UART (NS16550A) — boot messages
- FDT parser — detect memory, devices
- VirtIO block — read `kernel.elf` from disk
- ELF64 parser — load segments into memory
- Jump to kernel (a0=hart_id, a1=fdt)

**Status:** v0.1 (asm prototype) → v0.2 (C++ — planned)

---

## SlipperKernel

Monolithic kernel in pure Rust (`no_std`, `no_main`, panic=abort).
Single dependency: `riscv = "0.16.1"`.

**Responsibilities:**
- `kernel/main.rs` — entry point `kernel_main`, init
- `kernel/drivers/` — UART, CLINT, PLIC, VirtIO
- `kernel/mm/` — bump allocator, page allocator, Sv39
- `kernel/proc/` — tasks, round-robin, context switch
- `kernel/fs/` — SlipFS (block FS)
- `kernel/shell/` — slip shell (UART CLI)

**Status:** v0.1 (active)

---

## SlipperOS

System layer. Currently holds documentation and build scripts.
Future: userspace programs, init, libraries.

**Responsibilities:**
- `docs/` — full project documentation
- `build-docs.sh` — HTML doc generator
- Future: userspace (picolibc, C programs)

**Status:** forming
