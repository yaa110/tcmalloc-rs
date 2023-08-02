# TCMalloc-rs

[![crates.io](https://img.shields.io/crates/v/tcmalloc2.svg)](https://crates.io/crates/tcmalloc2)

Rust FFI bindings to [TCMalloc](https://github.com/gperftools/gperftools).

## Dependencies

`libstdc++`, `libclang` and `libunwind` must be installed on the system.

## How to Use

```rust
use tcmalloc2::TcMalloc;

#[global_allocator]
static GLOBAL: TcMalloc = TcMalloc;
```
