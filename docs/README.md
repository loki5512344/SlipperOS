# SlipperOS

Минималистичное ядро для OC2r, написанное на Rust.

## Быстрый старт

```bash
# Сборка
cargo build --release

# Запуск в QEMU
cargo run --release
```

## Структура

- `boot/` — точка входа, линкер-скрипт
- `kernel/` — ядро: main, panic, lib
- `drivers/` — UART, CLINT, PLIC, VirtIO
- `mm/` — менеджеры памяти
- `proc/` — задачи и планировщик
- `fs/` — файловая система
- `shell/` — slip shell
- `docs/` — документация
