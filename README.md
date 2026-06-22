# SlipperOS

Операционная система для RISC-V (OC2r / Milk-V Duo S). Состоит из трёх компонентов:

```
Slipper/
├── SlipperBoot/     — загрузчик (C++)
├── SlipperKernel/   — ядро (Rust, no_std)
└── SlipperOS/       — системный слой (доки, скрипты, userspace)
```

---

## Компоненты

| Компонент | Язык | Назначение | Статус |
|-----------|------|-----------|--------|
| **SlipperBoot** | C++ | Первичная загрузка: UART, FDT, VirtIO, ELF-парсер | План (v0.2) |
| **SlipperKernel** | Rust | Монолитное ядро: MM, процессы, драйверы, shell | В разработке (v0.1) |
| **SlipperOS** | — | Документация, билд-скрипты, в будущем — userspace | Активно |

---

## Сборка

### SlipperKernel

```bash
cd SlipperKernel
cargo build --release

# Запуск в QEMU
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

## План работ

| Версия | Компонент | Что делаем |
|--------|-----------|-----------|
| v0.1 | SlipperKernel | Костяк ядра: UART, MM, драйверы, shell — **сделано** |
| v0.2 | SlipperBoot | Загрузчик на C++: FDT, VirtIO, ELF |
| v0.3 | SlipperKernel | Прерывания, задачи, round-robin, syscall |
| v0.4 | SlipperKernel | VirtIO block v2 MMIO в ядре |
| v0.5 | Оба | SlipFS + первый userspace |
| v0.6 | SlipperOS | CLI-утилиты, init-процесс |
| v1.0 | Все | Slip shell как userspace, picolibc, модули |

Подробнее — [docs/dev/roadmap.md](docs/dev/roadmap.md)

---

## License

GPL-3.0-or-later
