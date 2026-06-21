# SlipperBoot — загрузчик на C++

## Дизайн

Чистый C++, без единого `.S` файла. Всё что нужно ассемблеру — inline asm
в `[[gnu::naked]]` точке входа. Одна цель: прочитать ядро с диска и прыгнуть
в него.

## Почему C++, не Rust и не C

| Язык | За | Против |
|---|---|---|
| Rust | типобезопасность | `#[naked]` нестабилен, extra тулчейн |
| C | простота | нет шаблонов для MMIO-врапперов |
| **C++** | шаблоны для регистров, `constexpr` для MMIO, naked entry | — |

Выбран C++ потому что bootloader маленький, `volatile` + шаблоны удобны
для MMIO, а naked entry работает стабильно в GCC.

## Entry point (ноль ассемблера)

Единственная функция с inline asm — `_start`. Всё остальное — чистый C++.

```cpp
// boot_entry.cpp
#include <cstdint>

extern "C" {
    extern uint8_t _sbss[], _ebss[], _stack_end[];
    void boot_main();
}

[[gnu::naked]] [[gnu::section(".text.boot")]]
void _start() {
    asm volatile(
        "csrr    t0, mhartid\n"
        "li      t1, 0\n"
        "bne     t0, t1, _hang\n"
        "la      t0, _sbss\n"
        "la      t1, _ebss\n"
        "li      t2, 0\n"
        "1:\n"
        "bgeu    t0, t1, 2f\n"
        "sw      t2, 0(t0)\n"
        "addi    t0, t0, 4\n"
        "j       1b\n"
        "2:\n"
        "la      sp, _stack_end\n"
        "call    boot_main\n"
        "_hang:\n"
        "wfi\n"
        "j       _hang\n"
    );
}
```

Это единственный `asm volatile` в проекте. Всё что ниже — C++.

## Структура

```
bootloader/
├── linker.ld              # линкуется на 0x80000000
├── Makefile                # cross-riscv64 g++
├── boot_entry.cpp          # _start (inline asm, 1 файл)
├── boot_main.cpp           # boot_main(), init sequence
├── uart.hpp                # NS16550A MMIO-враппер (шаблон)
├── uart.cpp
├── virtio.hpp              # VirtIO block legacy MMIO
├── virtio.cpp
├── elf.hpp                 # ELF64 header parser
├── elf.cpp
├── fdt.hpp                 # DeviceTree parser
└── fdt.cpp
```

## boot_main()

```cpp
void boot_main() {
    uart.init();
    uart.puts("SlipperBoot v0.1\n");

    FDT fdt(fdt_addr);
    auto mem = fdt.memory();

    VirtIOBlock disk;
    if (!disk.probe()) fail("no disk");

    static uint8_t kernel_buf[4_MB] __attribute__((aligned(512)));

    for (int i = 0; i < KERNEL_SECTORS; i++) {
        disk.read(KERNEL_LBA + i, &kernel_buf[i * 512]);
    }

    ELF elf(kernel_buf);
    if (!elf.valid()) fail("bad ELF");

    elf.load_all();
    uart.puts("jumping to kernel\n");
    elf.entry()(hart_id, fdt_addr);
}
```

## UART (NS16550A)

```cpp
struct UART {
    static constexpr uintptr_t BASE = 0x10000000;

    template <uintptr_t Off>
    using reg = volatile uint8_t *;

    void init() {
        reg<3>()[BASE] = 0x03;  // LCR: 8N1
        reg<2>()[BASE] = 0x07;  // FCR: FIFO
        reg<1>()[BASE] = 0x01;  // IER: RX IRQ
    }

    void put(char c) {
        while (!(reg<5>()[BASE] & (1 << 5)));
        reg<0>()[BASE] = c;
    }

    char get() {
        while (!(reg<5>()[BASE] & 1));
        return reg<0>()[BASE];
    }
};
```

Шаблонный доступ через `reg<N>()` — компилятор схлопнет в константное
смещение. Никаких магических чисел в коде.

## VirtIO Block

- Legacy MMIO (одна страница vring через QueuePfn)
- Feature negotiation: ACK → DRIVER → FEATURES_OK → DRIVER_OK
- 3 дескриптора на запрос: header + data + status
- Polling used ring (прерываний в bootloader не нужно)

Проверить на OC2r: если отдаёт v2 MMIO (раздельные QueueDesc/QueueAvail/
QueueUsed) — переписать под v2.

## ELF парсер

- Читает ELF64 header: magic, machine=RISCV(0xF3), type=EXEC
- Итерирует Program Headers
- Для каждого PT_LOAD: копирует `filesz` байт из ELF, обнуляет `memsz-filesz`
- Возвращает entry point

## FDT парсер

- Ищет `memory` node в дереве
- Читает `reg` свойство: (start, size)
- Fallback: 0x80000000, 128MB если FDT нет

## Сборка

```makefile
CXX = riscv64-unknown-elf-g++
CXXFLAGS = -march=rv64gc -mabi=lp64d -ffreestanding -nostdlib -O2
LDFLAGS = -T linker.ld -nostdlib

bootloader.bin: bootloader.elf
    riscv64-unknown-elf-objcopy -O binary $< $@
```

## Линкер-скрипт

```
BASE = 0x80000000;
.text : { *(.text.boot) *(.text) }
.rodata : { *(.rodata) }
.data : { *(.data) }
.bss : { _sbss = .; *(.bss); _ebss = .; }
. = ALIGN(16);
_stack_start = .;
. += 4K;
_stack_end = .;
```

## Ограничения

- Максимум: 32KB на весь bootloader
- Стек: 4KB
- bss обнуляется в _start inline asm
- Никаких глобальных конструкторов (не используем static objects)
