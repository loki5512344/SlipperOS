# Архитектура SlipperOS

SlipperOS — монолитное ядро, работающее в S-mode RISC-V.

## Компоненты

- **boot** — точка входа _start, обнуление bss, стек
- **SlipperBoot** — загрузчик с диска (ELF парсер, FDT, boot.cfg)
- **kernel** — инициализация, panic handler
- **drivers** — UART, CLINT, PLIC, VirtIO (block + GPU)
- **mm** — bump allocator, page allocator, Sv39
- **proc** — задачи, round-robin, контекст
- **fs** — slipfs (блочная ФС)
- **shell** — slip shell (UART-based CLI)
- **compositor** — Wayland + virtio-gpu (в разработке)

## Поток управления

```
OpenSBI → _start → [SlipperBoot] → kernel_main → shell_start
                                            → compositor_start (v1.0)
```
