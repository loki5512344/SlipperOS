# Документация Slipper

Проект состоит из трёх компонентов. Документация описывает каждый из них.

## Структура docs/

```
docs/
├── architecture/    — общая архитектура, boot flow, memory map
├── dev/             — разработка: сборка, отладка, contributing
├── hardware/        — железо: UART, CLINT, PLIC, VirtIO
├── internals/       — внутренности: coding style, panic policy
├── kernel/          — ядро SlipperKernel: прерывания, mm, proc
├── lore/            — история, философия
├── shell/           — slip shell: команды, внутренности
└── README.md        — этот файл
```

## Что где описывается

| Раздел | Про что | Компонент |
|--------|---------|-----------|
| `architecture/` | Как всё работает вместе | Все |
| `dev/` | Разработка и план | Все |
| `hardware/` | Драйверы устройств | SlipperKernel |
| `internals/` | Соглашения и политики | SlipperKernel |
| `kernel/` | Внутренности ядра | SlipperKernel |
| `shell/` | Slip shell | SlipperKernel |
| `lore/` | Почему Slipper | Проект в целом |

## Репозитории

| Компонент | Репозиторий |
|-----------|------------|
| SlipperBoot | https://github.com/loki5512344/SlipperBoot |
| SlipperKernel | https://github.com/loki5512344/SlipperKernel |
| SlipperOS | (корень — этот репозиторий) |

## План

См. [dev/roadmap.md](dev/roadmap.md).
