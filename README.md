# scent

scent is an EVM bytecode analyzer. Currently, it's only capability is parsing bytecode into opcodes via `disasm` subcommand.

## Installation

Ensure you have [Rust and Cargo installed](https://rustup.rs/).

```bash
git clone https://github.com/synthlike/scent.git
cd scent
cargo build --release
```

The binary will be located at `./target/release/scent`.

## Usage

```bash
$ ./scent disasm assets/counter.bin
0000 96   PUSH1 0x80
0002 96   PUSH1 0x40
0004 82   MSTORE
0005 52   CALLVALUE
0006 128  DUP1
0007 21   ISZERO
0008 96   PUSH1 0x0e
000a 87   JUMPI
000b 95   PUSH0
000c 95   PUSH0
000d 253  REVERT
000e 91   JUMPDEST
// ...
```
