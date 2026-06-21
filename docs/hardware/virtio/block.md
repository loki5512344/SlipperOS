# VirtIO Block

Device ID: 2, PLIC IRQ: 1-8 (зависит от индекса)

## Очередь

- 1 очередь (requestq), размер определяется через `QueueNumMax`
- Descriptor table: 16 байт на entry (addr, len, flags, next)
- Available ring: driver → device
- Used ring: device → driver

## Запрос (3 дескриптора)

1. **Header** — `struct { blktype: u32, ioprio: u32, sector: u64 }` (16 байт)
2. **Data** — буфер данных (in = устройство пишет, out = устройство читает)
3. **Status** — 1 байт (0=OK, 1=error, 2=unsupported)

## Типы запросов

| blktype | Операция |
|---|---|
| 0 | IN (читаем с диска) |
| 1 | OUT (пишем на диск) |
| 8 | FLUSH |

## Размер сектора

Один сектор = 512 байт. Все смещения в секторах.

## PLIC прерывания

VirtIO device 0 (0x10001000) → PLIC IRQ 1
VirtIO device 1 (0x10002000) → PLIC IRQ 2
...
VirtIO device 7 (0x10008000) → PLIC IRQ 8

## Инициализация

1. Reset (Status = 0)
2. ACK → DRIVER
3. Negotiate features (отключаем VIRTIO_BLK_F_RO если надо)
4. FEATURES_OK (проверяем подтверждение)
5. Настройка vring: QueueSel → QueueNum → alloc → QueuePfn
6. DRIVER_OK → device live

## Чтение сектора

```
header.blktype = 0 (IN)
header.sector = offset / 512
header.ioprio = 0

// 3 дескриптора в vring
desc[0] = { addr: &header, len: 16, flags: NEXT }
desc[1] = { addr: buffer,  len: 512, flags: NEXT | WRITE }
desc[2] = { addr: &status, len: 1,   flags: WRITE }

avail.ring[idx] = 0  // head descriptor index
avail.idx++
QueueNotify = 0      // kick device
```
