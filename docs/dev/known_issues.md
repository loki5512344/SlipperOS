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

## VirtIO: проверить Legacy vs v2 MMIO для OC2r

Документация и прототип драйвера (`virtio/block.rs`) написаны под Legacy
MMIO (один QueuePfn). Современный QEMU `virt` отдаёт v2 по умолчанию
(раздельные QueueDescLow/High, QueueAvailLow/High, QueueUsedLow/High).
OC2r (Minecraft-мод) может эмулировать любой из них — неизвестно.

**Риск**: драйвер собранный под Legacy не заведётся на реальном OC2r.

**План**: свериться с исходниками/вики OC2r перед v0.4.

## Обработка трапов: S-mode регистры

В документацию внесены правки (`sepc`/`scause`/`stval`/`stvec`/`sret`),
но кода trap handler ещё нет. При реализации важно не скопировать M-mode
формулировки из старых доков — настроить `stvec`, а не `mtvec`.

**План**: v0.3, через `global_asm!` или `naked_asm!` внутри Rust-файла.

## Userspace: ELF load + syscalls не реализованы

ELF-загрузчик (`elf_loading.md`) спроектирован, но не реализован.
Зависит от v0.4 (VirtIO block) и v0.5 (SlipFS).

## Wayland: свой compositor вместо wlroots

Составной риск:
- VirtIO-GPU драйвер не написан (v0.6)
- wlroots требует POSIX-слоя (epoll, libinput, DRM), которого нет
- Hyprland добавляет C++17, STL, exceptions

**План**: свой минимальный compositor на Rust (v0.7).
wlroots/Hyprland — отдельная исследовательская ветка, не блокирует v1.0.
