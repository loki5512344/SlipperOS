# SlipperOS

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](../LICENSE)
[![Arch: RISC-V 64](https://img.shields.io/badge/arch-RISCV64-green)](https://riscv.org)
[![Status](https://img.shields.io/badge/status-development-yellow)]()
[![Rust](https://img.shields.io/badge/rust-nightly_2026--06--01-orange)](https://rust-lang.org)
[![Lines](https://img.shields.io/badge/lines-3500%2B-blue)]()
[![OC2r](https://img.shields.io/badge/target-OC2r-8A2BE2)]()

A minimalist RISC-V 64-bit kernel written in Rust. Monolithic design, S-mode, built for [OC2r](https://github.com/loki5512344/SlipperOS) and Milk-V Duo S. Created by Loki + Seal.

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

## Links

- [Why SlipperOS](docs/lore/why_slipperos.en.html) — the story behind the project
- [Roadmap](docs/dev/roadmap.en.html) — current status and plans
- [Getting Started](docs/dev/getting_started.en.html) — build and run
