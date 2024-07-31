## zkfibonacci-rs

Example of Bitcoin contract for use with [Bitcoin Elftrace](https://github.com/halseth/elftrace).

**Contract input**: zkproof.

**Contract output**: 0 if proof is valid for computation of the 16th fibonacci term, 1 otherwise.

## Usege
Compile:
```bash
cargo build --release
```

Resulting binary found in `target/riscv32i-unknown-none-elf/release/zkfibonacci-rs`.
