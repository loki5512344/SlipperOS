# NS16550A UART

- **Адрес**: 0x10000000
- **MMIO**, 8-bit регистры
- PLIC interrupt ID: **10**

## Регистры

| Смещение | DLAB=0 | DLAB=1 | Описание |
|---|---|---|---|
| 0 | RBR/THR | DLL | Приём/передача / делитель (мл.) |
| 1 | IER | DLM | Прерывания / делитель (ст.) |
| 2 | FCR/IIR | — | FIFO control / идентификация |
| 3 | LCR | LCR | Line control |
| 4 | MCR | MCR | Modem control |
| 5 | LSR | LSR | Line status |
| 6 | MSR | MSR | Modem status |
| 7 | SCR | SCR | Scratch |

## Инициализация

```rust
base[3] = 0x03;         // LCR: 8N1 (word length 8, no parity, 1 stop)
base[2] = 0x07;         // FCR: FIFO enable, clear
base[1] = 0x01;         // IER: enable RX interrupt
```

### Baud rate (опционально для QEMU)

```rust
// Делитель = clock_hz / (baud * 16)
// QEMU: clock = 22_729_000, target = 2400 baud
// divisor = ceil(22_729_000 / (2400 * 16)) = 592
base[3] |= 1 << 7;      // DLAB = 1
base[0] = divisor & 0xFF;
base[1] = divisor >> 8;
base[3] &= !(1 << 7);   // DLAB = 0
```

## LSR (Line Status Register)

- Bit 0 (DR): данные готовы к чтению
- Bit 5 (THRE): пустой буфер передачи
- Bit 6 (TEMT): transmitter empty

## Режимы

1. **Polling** — читаем LSR.DR в цикле
2. **Interrupt** — UART дёргает PLIC IRQ 10, читаем в обработчике
