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

Currently, only disassembly is supported via the `disasm` subcommand.

By default, scent analyzes the bytecode and adds `.init`/`.code`/`.metadata` labels to help distinguish binary "sections".
It also labels `JUMPDEST` instructions based on detected function selectors.

```bash
$ scent disasm contract.bin

.init:
  0000: 60 PUSH1 0x80
  0002: 60 PUSH1 0x40
  0004: 52 MSTORE
  0005: 34 CALLVALUE
; ...
  001b: f3 RETURN
  001c: fe INVALID
.runtime:
  0000: 60 PUSH1 0x80
  0002: 60 PUSH1 0x40
  0004: 52 MSTORE
  0005: 34 CALLVALUE
  0006: 80 DUP1
  0007: 15 ISZERO
  0008: 61 PUSH2 0x000f
  000b: 57 JUMPI
  000c: 5f PUSH0
  000d: 80 DUP1
  000e: fd REVERT
  000f: 5b JUMPDEST
  0010: 50 POP
  0011: 60 PUSH1 0x04
  0013: 36 CALLDATASIZE
  0014: 10 LT
  0015: 61 PUSH2 0x0055
  0018: 57 JUMPI
  0019: 5f PUSH0
; ...
  001a: 35 CALLDATALOAD
  001b: 60 PUSH1 0xe0
  001d: 1c SHR
  001e: 80 DUP1
  001f: 63 PUSH4 0x20965255
  0024: 14 EQ
  0025: 61 PUSH2 0x0059
  0028: 57 JUMPI
  0029: 80 DUP1
; ...
0x20965255:
  0059: 5b JUMPDEST
  005a: 61 PUSH2 0x0061
  005d: 61 PUSH2 0x00c5
  0060: 56 JUMP
  0061: 5b JUMPDEST
; ...
  0076: f3 RETURN
; ...
  024c: fe INVALID
.metadata:
  0000: a2646970667358221220d623ce44df9f6bdf57826803e242c0cb8831b8ebb30e362113004578c8071f5664736f6c63430008140033
```

Additionally, the `--raw` flag can be passed to disable all analysis, or `--runtime` to disable sections analysis while still labeling jump destinations for external functions.

Push data can be decorated with helpful information. Currently, only function selector decoration is supported.
Function selectors need to be provided via a JSON file (generated from [sift](https://github.com/synthlike/sift)), as shown below:

```bash
$ scent disasm contract.bin --decorated --selectors selectors.json

; ...
  001f: 63 PUSH4 0x20965255 ; getValue()
  0024: 14 EQ
  0025: 61 PUSH2 0x0059
  0028: 57 JUMPI
  0029: 80 DUP1
  002a: 63 PUSH4 0x3bfd7fd3 ; incrementValue()
  002f: 14 EQ
  0030: 61 PUSH2 0x0077
  0033: 57 JUMPI
  0034: 80 DUP1
; ...
0x20965255: ; getValue()
  0059: 5b JUMPDEST
  005a: 61 PUSH2 0x0061
  005d: 61 PUSH2 0x00c5
  0060: 56 JUMP
  0061: 5b JUMPDEST
; ...
  0076: f3 RETURN
0x3bfd7fd3: ; incrementValue()
  0077: 5b JUMPDEST
  0078: 61 PUSH2 0x007f
  007b: 61 PUSH2 0x00cd
  007e: 56 JUMP
  007f: 5b JUMPDEST
  0080: 00 STOP
```
