# Карта памяти

## Устройства (MMIO)

| Адрес | Устройство |
|---|---|
| 0x02000000 | CLINT |
| 0x0C000000 | PLIC (QEMU virt) |
| 0x10000000 | UART (NS16550A) |
| 0x10001000 | VirtIO MMIO #0 (block) |
| 0x10002000 | VirtIO MMIO #1 |
| ... | ... |

## RAM

| Адрес | Что | Размер |
|---|---|---|
| 0x80000000 | SlipperBoot (текст + данные) | ≤ 32KB |
| 0x80008000 | SlipperBoot стек | 4KB |
| 0x80100000 | SlipperBoot bss/data (опциональный резерв) | — |
| 0x80200000 | **SlipperOS kernel** (из ELF) | переменный |
| 0x80200000 + kernel_size | Heap (bump, page allocator) | до top |
| memory_top | конец RAM (из FDT, ~128MB-1GB) | — |

## SlipperBoot layout (внутри 0x80000000)

```
.text.boot   → _start() naked (inline asm)
.text        → boot_main, uart, virtio, elf, fdt
.rodata      → строки, константы
.data        → глобалы (мало, только fifo)
.bss         → буферы (kernel_buf[4MB])
_stack       → 4KB в конце
```
