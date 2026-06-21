# VirtIO — MMIO Interface (Legacy)

VirtIO — стандартный интерфейс виртуальных устройств в QEMU.

## MMIO Base

Устройства в QEMU `virt` располагаются с шагом 0x1000:

```
0x10001000 — VirtIO 0
0x10002000 — VirtIO 1
...
0x10008000 — VirtIO 7
```

## Магическое число

`0x74726976` = "triv" (little-endian "virt").

## Device IDs

| ID | Устройство |
|---|---|
| 0 | none (not connected) |
| 1 | Network |
| 2 | Block |
| 3 | Console |
| 4 | Entropy (RNG) |
| 6 | GPU |

## MMIO регистры (Legacy)

| Смещение | Регистр |
|---|---|
| 0x000 | MagicValue |
| 0x004 | Version |
| 0x008 | DeviceID |
| 0x00C | VendorID |
| 0x010 | HostFeatures |
| 0x014 | HostFeaturesSel |
| 0x020 | GuestFeatures |
| 0x024 | GuestFeaturesSel |
| 0x028 | GuestPageSize |
| 0x030 | QueueSel |
| 0x034 | QueueNumMax |
| 0x038 | QueueNum |
| 0x03C | QueueAlign |
| 0x040 | QueuePfn |
| 0x050 | QueueNotify |
| 0x060 | InterruptStatus |
| 0x064 | InterruptAck |
| 0x070 | Status |
| 0x100 | Config |

## Инициализация

1. **Reset** — `Status = 0`
2. **ACK** — `Status |= ACK`
3. **DRIVER** — `Status |= DRIVER`
4. **Features** — read HostFeatures, write negotiated GuestFeatures
5. **FEATURES_OK** — `Status |= FEATURES_OK`, re-read, проверяем
6. **Queue setup** — QueueSel → QueueNum → alloc Queue → QueuePfn
7. **DRIVER_OK** — `Status |= DRIVER_OK` → device live

## VirtQueue (vring)

Устройство и драйвер общаются через кольцевые буферы в RAM:

```
Descriptor Table (16 байт × N)
  [addr, len, flags, next]

Available Ring (driver → device)
  [idx, ring[N], flags]

Used Ring (device → driver)
  [idx, ring[N], flags]
```

- Descriptor: физический адрес, длина, флаги (NEXT=1, WRITE=2), next idx
- Available: драйвер помещает индекс descriptor head
- Used: устройство возвращает индекс обработанного запроса
