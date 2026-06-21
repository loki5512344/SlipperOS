# Последовательность загрузки

## OpenSBI (M-mode)

OpenSBI (вшит в OC2r) стартует в M-mode. Настраивает делегирование
прерываний, передаёт управление на 0x80000000 в S-mode.

## SlipperBoot (S-mode, C++)

Встречает управление по адресу 0x80000000. Написан на чистом C++,
единственный `asm volatile` — naked `_start()`.

1. **Hart select** — `mhartid` → hart 0 работает, остальные `wfi`.
2. **BSS clear** — обнуляет `.bss` через inline asm в _start().
3. **Stack** — `sp = &_stack_end`.
4. **UART init** — NS16550A, вывод "SlipperBoot v0.1\n".
5. **FDT parse** — читает Device Tree (a1), определяет размер памяти.
6. **VirtIO probe** — ищет VirtIO block device, читает LBA.
7. **ELF load** — находит `kernel.elf` на диске, парсит Program Headers,
   копирует сегменты по адресам из ELF (0x80200000).
8. **Entry** — прыгает на точку входа ядра, передавая a0=hart_id, a1=fdt.

```
OpenSBI (M-mode)
    ↓
SlipperBoot @ 0x80000000 (S-mode, C++)
    │  uart_init()
    │  fdt_parse()
    │  virtio_read(kernel.elf)
    │  elf_load()
    ↓
SlipperOS @ 0x80200000 (S-mode, Rust)
```

## SlipperOS kernel (S-mode, Rust)

Загружается SlipperBoot'ом по адресу из ELF. Точка входа — `kernel_main`.

1. **UART init** — "SlipperOS v0.1 booting..."
2. **print_seal** — аски тюлень
3. **PLIC init** — priority, enables, threshold
4. **CLINT init** — mtimecmp = mtime + slice
5. **MM init** — page allocator из _end..memory_top
6. **Sched init** — idle task
7. **Shell start** — "slip> ready"

## S-mode vs M-mode

SlipperOS работает в S-mode. OpenSBI выполняет M-mode bootstrap
и делегирует трапы.

### Регистры

| Назначение | M-mode | S-mode |
|---|---|---|
| Trap vector | `mtvec` | `stvec` |
| Exception PC | `mepc` | `sepc` |
| Cause | `mcause` | `scause` |
| Trap value | `mtval` | `stval` |
| Status | `mstatus` | `sstatus` |
| IE | `mie` | `sie` |
| Return | `mret` | `sret` |
