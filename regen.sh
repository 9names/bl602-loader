#!/bin/sh
cargo build --release
target-gen elf -u target/riscv32imac-unknown-none-elf/release/bl602-flashloader BL6xx.yaml

