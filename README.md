# rustpp

Pure Rust port of [oppai-ng](https://github.com/Francesco149/oppai-ng): fast, lightweight, and zero-dependency osu! difficulty and pp calculator.

[![CI](https://github.com/akikohatsune/rustpp/actions/workflows/ci.yml/badge.svg)](https://github.com/akikohatsune/rustpp/actions/workflows/ci.yml)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

## Features

- **Pure Rust (Zero External Dependencies)**: Built entirely with Rust's standard library (`std`).
- **High Performance**: Evaluates over **1,700 beatmaps/second**.
- **Accurate**: Verified against oppai's official test suite of 895 real scores with 100% pass rate (average error 0.24%).
- **Multi-platform & Multi-arch**:
  - Windows (x86_64)
  - Linux (x86_64, aarch64 / ARM64, armv7 / ARM32)
  - macOS (Intel & Apple Silicon)
- **Compact**: Standalone compiled binary is only ~240 KB.
- **Multiple Output Formats**: `text` (with ASCII strain graph), `json`, `csv`, `binary` (`binoppai`), `gnuplot`, `null`.
- **Modes Supported**: osu!standard (mode 0) and osu!taiko (mode 1, including standard conversions).

---

## Installation & Building

### From Source (Any Platform with Cargo)

```bash
git clone https://github.com/akikohatsune/rustpp.git
cd rustpp
cargo build --release
```
The binary will be located at `target/release/oppai` (or `target/release/oppai.exe` on Windows).

### Linux & Linux ARM

You can also use the build script:
```bash
./build
```

### Cross-Compilation (Linux ARM64 / ARMv7)

Using [`cross`](https://github.com/cross-rs/cross):
```bash
cargo install cross --git https://github.com/cross-rs/cross

# Linux ARM 64-bit (Raspberry Pi 4/5, ARM servers)
cross build --release --target aarch64-unknown-linux-musl

# Linux ARM 32-bit (Raspberry Pi 2/3/Zero 2)
cross build --release --target armv7-unknown-linux-musleabihf

# Linux x86_64 static binary
cross build --release --target x86_64-unknown-linux-musl
```

### Docker

```bash
docker build -t oppai .
cat map.osu | docker run -i --rm oppai - +HDDT
```

---

## CLI Usage

Run without arguments to show usage:
```bash
oppai
```

### Examples

```bash
# Basic calculation
oppai path/to/map.osu

# With mods, accuracy, combo, misses
oppai path/to/map.osu +HDDT 98% 500x 1m

# JSON output
oppai path/to/map.osu +HDHR -ojson

# CSV output
oppai path/to/map.osu +HDHR -ocsv

# Convert to Taiko mode
oppai path/to/map.osu -taiko

# Custom stat overrides
oppai path/to/map.osu AR10 OD10 CS4

# Pipe from stdin
cat path/to/map.osu | oppai - +HDHR
# or on Windows PowerShell:
Get-Content path/to/map.osu -Raw | oppai - +HDHR
```

---

## Library Usage

Add `rustpp` to your `Cargo.toml`:

```toml
[dependencies]
rustpp = { path = "path/to/rustpp" }
```

### Example

```rust
use rustpp::constants::*;
use rustpp::ezpp::Ezpp;

fn main() {
    let mut ez = Ezpp::new();
    ez.set_mods(MODS_HD | MODS_DT);
    ez.set_accuracy_percent(98.0);
    ez.set_combo(400);
    ez.set_nmiss(1);

    if let Ok(_) = ez.parse_file("path/to/map.osu") {
        println!("{} - {} [{}]", ez.artist(), ez.title(), ez.version());
        println!("{:.2} stars", ez.stars());
        println!("{:.2} pp", ez.pp());
    }
}
```

Run the included example:
```bash
cargo run --example basic
```

---

## Testing & Verification

Run the integration test suite (895 real osu! scores from oppai-ng test suite):
```bash
cargo test --test suite --release -- --nocapture
```

Run CLI automated tests:
```bash
cargo test --test cli_test --release -- --nocapture
```

---

## License

This is free and unencumbered software released into the public domain. See [LICENSE](LICENSE) for details.
