#!/bin/sh
cargo build --release --features offset_2000
target-gen elf -u target/riscv32imac-unknown-none-elf/release/bl602-flashloader BL6xx.yaml

cargo build --release --features offset_0
target-gen elf -u target/riscv32imac-unknown-none-elf/release/bl602-flashloader BL6xx_no_offset.yaml
