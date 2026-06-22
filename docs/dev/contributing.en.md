# Contributing

## Code style

- `cargo fmt` before committing
- no unsafe in public APIs (except MMIO)
- Comments for non-obvious logic

## Rules

- No unwrap() — only match/if let
- Panic only in unrecoverable situations
- All MMIO addresses are constants
- Documentation for every module
