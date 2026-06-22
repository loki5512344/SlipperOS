# SlipperOS

RISC-V operating system (OC2r / Milk-V Duo S). Three components:

```
Slipper/
├── SlipperBoot/     — bootloader (C++)
├── SlipperKernel/   — kernel (Rust, no_std)
└── SlipperOS/       — system layer (docs, scripts, userspace)
```

---

## Components

| Component | Lang | Role | Status |
|-----------|------|------|--------|
| **SlipperBoot** | C++ | UART, FDT, VirtIO, ELF loader | Planned (v0.2) |
| **SlipperKernel** | Rust | Monolithic kernel: MM, processes, drivers, shell | In progress (v0.1) |
| **SlipperOS** | — | Documentation, build scripts, future userspace | Forming |

---

## Build

### SlipperKernel

```bash
cd SlipperKernel
cargo build --release

# Run in QEMU
qemu-system-riscv64 \
  -machine virt -m 128M -nographic \
  -bios default \
  -kernel target/riscv64gc-unknown-none-elf/release/slipperos
```

### SlipperBoot

```bash
cd SlipperBoot
make
```

---

## Roadmap

| Version | Component | Milestone |
|---------|-----------|-----------|
| v0.1 | SlipperKernel | Kernel skeleton: UART, MM, drivers, shell — **done** |
| v0.2 | SlipperBoot | C++ bootloader: FDT, VirtIO, ELF |
| v0.3 | SlipperKernel | Traps, tasks, round-robin, syscall |
| v0.4 | SlipperKernel | VirtIO block v2 MMIO |
| v0.5 | Both | SlipFS + first userspace |
| v0.6 | SlipperOS | CLI tools, init process |
| v1.0 | All | Userspace shell, picolibc, modules |

Details — [docs/dev/roadmap.md](docs/dev/roadmap.md)

---

## License

GPL-3.0-or-later
