# OnyxOS

RISC-V operating system (OC2r / Milk-V Duo S). Three components:

```
Onyx/
├── OnyxBoot/     — bootloader (C++)
├── OnyxKernel/   — kernel (Rust, no_std)
└── OnyxOS/       — system layer (docs, scripts, userspace)
```

---

## Components

| Component | Lang | Role | Status |
|-----------|------|------|--------|
| **OnyxBoot** | C++ | UART, FDT, VirtIO, SDHCI, FAT32/EXT4, ELF, boot menu | v0.4 |
| **OnyxKernel** | Rust | Monolithic kernel: MM, processes, drivers, shell | In progress (v0.1) |
| **OnyxOS** | — | Documentation, build scripts, future userspace | Forming |

---

## Build

### OnyxKernel

```bash
cd OnyxKernel
cargo build --release

# Run in QEMU
qemu-system-riscv64 \
  -machine virt -m 128M -nographic \
  -bios default \
  -kernel target/riscv64gc-unknown-none-elf/release/Onyxos
```

### OnyxBoot

```bash
cd OnyxBoot
make
```

---

## Roadmap

| Version | Component | Milestone |
|---------|-----------|-----------|
| v0.1 | OnyxKernel | Kernel skeleton: UART, MM, drivers, shell — **done** |
| — | OnyxBoot | C++ bootloader: FDT, VirtIO, ELF |
| v0.3 | OnyxKernel | Traps, tasks, round-robin, syscall |
| v0.4 | OnyxKernel | VirtIO block v2 MMIO |
| v0.5 | Both | SlipFS + first userspace |
| v0.6 | OnyxOS | CLI tools, init process |
| v1.0 | All | Userspace shell, picolibc, modules |

Details — [docs/dev/roadmap.md](docs/dev/roadmap.md)

---

## License

GPL-3.0-or-later
