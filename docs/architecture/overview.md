# Архитектура Slipper

Проект состоит из трёх независимых компонентов, каждый в своей директории:

```
OpenSBI (M-mode)
    ↓
SlipperBoot (S-mode, C++)
    ↓
SlipperKernel (S-mode, Rust — no_std)
    ↓
SlipperOS (userspace — будущее)
```

---

## SlipperBoot

Загрузчик на C++ (ни одного `.S` файла). Работает в S-mode, получает управление после OpenSBI.

**Ответственность:**
- UART (NS16550A) — вывод сообщений
- FDT-парсер — определение памяти, устройств
- VirtIO block — чтение `kernel.elf` с диска
- ELF64-парсер — загрузка сегментов в память
- Передача управления ядру (a0=hart_id, a1=fdt)

**Статус:** v0.1 (прототип на asm) → v0.2 (C++ реализация — план)

---

## SlipperKernel

Монолитное ядро на чистом Rust (`no_std`, `no_main`, panic=abort).
Единственная зависимость — `riscv = "0.16.1"`.

**Ответственность:**
- `kernel/main.rs` — точка входа `kernel_main`, инициализация
- `kernel/drivers/` — UART, CLINT, PLIC, VirtIO
- `kernel/mm/` — bump allocator, page allocator, Sv39
- `kernel/proc/` — задачи, round-robin, контекст
- `kernel/fs/` — SlipFS (блочная ФС)
- `kernel/shell/` — slip shell (UART CLI)

**Статус:** v0.1 (активно)

---

## SlipperOS

Системный слой. Пока содержит документацию и билд-скрипты.
В будущем — userspace-программы, init, библиотеки.

**Ответственность:**
- `docs/` — вся документация проекта
- `build-docs.sh` — генератор HTML-документации
- В будущем: userspace (picolibc, C-программы)

**Статус:** формируется
