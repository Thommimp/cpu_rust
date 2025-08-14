# cpu_rust

Final project for computer architecture DTU course number 02155.

## Overview

This repository contains an educational implementation of a CPU written in Rust. The design is based on a five-stage pipeline architecture, typical in modern RISC processors, and demonstrates key concepts including instruction decoding, execution, and memory access.

## Features

- **Rust Implementation:** Safe and performant code for CPU simulation.
- **Five-Stage Pipeline:** Models real CPU behavior with fetch, decode, execute, memory, and write-back stages.
- **Instruction Decoding:** Full instruction set decoding for RISC-V like instructions.
- **Register File and Memory:** Simulates registers and RAM for program execution.
- **Educational Focus:** Made for learning and experimenting with CPU architecture.

## CPU Pipeline Stages

The CPU is structured around the classic five-stage pipeline:

1. **Fetch:** Loads the next instruction from memory into the instruction register (IR) and advances the program counter (PC).
2. **Decode:** Decodes the binary instruction word into its operation and operands. This uses the instruction set architecture (ISA) defined in [`isa.rs`](src/isa.rs), supporting operations such as arithmetic, branching, and memory access.
3. **Execute:** Performs the actual computation or logic based on the decoded instruction. Includes arithmetic (ADD, SUB, etc.), logical (AND, OR, XOR), shift, and branch operations.
4. **Memory:** Handles memory read and write instructions. For loads (LB, LH, LW, etc.), retrieves data from memory into registers. For stores (SB, SH, SW), writes data from registers to memory.
5. **Write-back:** Updates the destination register with the result of computations or memory loads.

Each clock cycle advances the CPU through these stages via the `tick()` method, enabling instruction-by-instruction simulation.

## Instruction Decoding

Instruction decoding interprets the binary format of an instruction, extracting opcode, register indices, and immediate values. The decoding logic supports various instruction formats (I-type, S-type, B-type, U-type, J-type), matching RISC-V conventions.

Example decode snippet:
```rust
fn decode(&mut self) -> Result<(), String> {
    self.inst = {
        match Instruction::decode(self.ir) {
            Ok(inst) => inst,
            Err(e) => return Err(e),
        }
    };
    Ok(())
}
```

The decoding maps opcodes to instructions such as loads (`LB`, `LH`, `LW`, `LBU`, `LHU`), stores (`SB`, `SH`, `SW`), arithmetic (`ADDI`, `SLTI`, etc.), logical, and branch instructions.

## Getting Started

Clone the repository:

```bash
git clone https://github.com/Thommimp/cpu_rust.git
cd cpu_rust
```

Build the project:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

## More Details

For a deeper dive into the instruction set and architecture, see [`Architecture.md`](Architecture.md).

## License

This project is for educational purposes. Please refer to your course guidelines for usage.

## Author

Thommimp

---
Feel free to expand this README with code samples or diagrams from the repo!
