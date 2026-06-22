# Known Issues

Задокументированные проблемы, которые есть в коде на данный момент.

## Scheduler: context switch не подключён

`sched_yield()` двигает `CURRENT` и возвращается. `save_context()` /
`restore_context()` написаны в `context.rs`, но нигде не вызваны.
Планировщик пока не переключает задачи — просто ходит по кругу.

**План**: v0.3 — trap handler + вызов `sched_yield` из CLINT прерывания.

## Page allocator: нет contiguous-аллокаций

Текущий free list возвращает одну страницу за раз. Гарантии что два
последовательных `alloc()` вернут соседние страницы — нет.

Это заблокирует VirtIO block (v0.4): vring обязан лежать в физически
непрерывной памяти. Либо внедрить descriptor-based allocator (флаги
Taken/Last), либо уметь выделять N последовательных страниц.

**План**: решить до v0.4.

## VirtIO: Legacy vs v2 MMIO — resolved

Проверено по исходникам Sedna (`AbstractVirtIODevice.java`):
`VIRTIO_MMIO_VERSION = 0x2`, требует `VIRTIO_F_VERSION_1` при негоциации.
**Legacy транспорт не поддерживается**.

Документация (`hardware/virtio/overview.md`, `hardware/virtio/block.md`,
`dev/bootloader.md`) переписана под v2 MMIO:
`QueueDescLow/High`, `QueueDriverLow/High`, `QueueDeviceLow/High`,
`QueueReady`. Драйвер в ядре (`kernel/drivers/virtio/block.rs`) — v0.4.

**Риск снят**. Подтверждено: только v2.

## Обработка трапов: S-mode регистры

В документацию внесены правки (`sepc`/`scause`/`stval`/`stvec`/`sret`),
но кода trap handler ещё нет. При реализации важно не скопировать M-mode
формулировки из старых доков — настроить `stvec`, а не `mtvec`.

**План**: v0.3, через `global_asm!` или `naked_asm!` внутри Rust-файла.

## UART/VirtIO адреса захардкожены, не из FDT

`kernel/drivers/uart.rs` (0x10000000), `kernel/drivers/virtio/block.rs`
(0x10001000, IRQ 1-8), и kernel и bootloader пока хардкодят адреса
и IRQ. На OC2r с динамическим размещением устройств это не заведётся.

**План**: v0.2 (SlipperBoot) — FDT-парсер для UART и VirtIO;
v0.3 или v0.4 — передача FDT в ядро и отказ от хардкода.

## Userspace: ELF load + syscalls не реализованы

ELF-загрузчик (`elf_loading.md`) спроектирован, но не реализован.
Зависит от v0.4 (VirtIO block) и v0.5 (SlipFS).

## Shell: read_line() всегда возвращает None

`shell/slip.rs` — баг: `INPUT_LEN` сбрасывается в 0 до того, как строка
обработана, из-за чего `read_line()` всегда возвращает `None`, и команды
не выполняются.

**План**: исправить до v0.2.

## Sv39: map_page() не аллоцирует page tables

`mm/map.rs` — `map_page()` проверяет `pte.is_valid()` но при отсутствии
таблицы просто возвращается, а не выделяет новую страницу для page table.

**План**: v0.3, вместе с реальным переключением контекста.
