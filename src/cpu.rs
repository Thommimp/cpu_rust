use std::string::String;

use crate::isa::Instruction;
use crate::memory::Memory;
use crate::register_file::RegisterFile;

pub struct Cpu {
    clock: usize,
    registers: RegisterFile,
    pc: u32,
    ir: u32,
    inst: Instruction,
    memory: Memory,
    halt: bool,
}

impl Cpu {
    pub fn new(memory_capacity: usize) -> Self {
        let mut cpu = Cpu {
            clock: 0,
            registers: RegisterFile::new(),
            pc: 0,
            ir: 0,
            inst: Instruction::None,
            memory: Memory::new(memory_capacity),
            halt: false,
        };
        cpu.registers.set(2, ((memory_capacity / 2) - 1) as u32);
        return cpu;
    }

    // Wrapper fn for the Memory load_file method
    pub fn load_file(&mut self, path: &str) -> Result<usize, String> {
        self.memory.load_file(path)
    }

    // returns halt status
    pub fn halt(&self) -> bool {
        return self.halt;
    }

    // tick fn to execute one clock cycle of the cpu
    pub fn tick(&mut self) -> Result<(), String> {
        self.fetch();
        self.decode().unwrap();
        self.execute();
        self.disassemble();
        self.clock += 1;
        Ok(())
    }

    // fetch fn to fetch the next instruction and then increment the program counter by +4
    fn fetch(&mut self) {
        self.ir = self.memory.load_word(self.pc);
        self.pc += 4;
    }

    // decode fn to decode the word from the fetch into a Instruction
    fn decode(&mut self) -> Result<(), String> {
        self.inst = {
            match Instruction::decode(self.ir) {
                Ok(inst) => inst,
                Err(e) => return Err(e),
            }
        };
        Ok(())
    }

    // execute fn to make the operation specified by the inst
    fn execute(&mut self) {
        match &self.inst {
            Instruction::Lui(arg) => {
                self.registers.set(arg.rd, arg.imm << 12);
            }
            Instruction::Auipc(arg) => {
                self.registers
                    .set(arg.rd, self.pc.wrapping_add(arg.imm << 12));
            }
            Instruction::Jal(arg) => {
                self.registers.set(arg.rd, self.pc);
                self.pc = self.pc.wrapping_add(arg.imm);
            }
            Instruction::Jalr(arg) => {
                self.registers.set(arg.rd, self.pc);
                self.pc = self.registers.get(arg.rs1).wrapping_add(arg.imm);
            }
            Instruction::Beq(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                if op_a == op_b {
                    self.pc = self.pc.wrapping_add(arg.imm);
                }
            }
            Instruction::Bne(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                if op_a != op_b {
                    self.pc = self.pc.wrapping_add(arg.imm);
                }
            }
            Instruction::Blt(arg) => {
                let op_a = self.registers.get(arg.rs1) as i32;
                let op_b = self.registers.get(arg.rs2) as i32;
                if op_a < op_b {
                    self.pc = self.pc.wrapping_add(arg.imm);
                }
            }
            Instruction::Bge(arg) => {
                let op_a = self.registers.get(arg.rs1) as i32;
                let op_b = self.registers.get(arg.rs2) as i32;
                if op_a >= op_b {
                    self.pc = self.pc.wrapping_add(arg.imm);
                }
            }
            Instruction::Bltu(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                if op_a < op_b {
                    self.pc = self.pc.wrapping_add(arg.imm);
                }
            }
            Instruction::Bgeu(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                if op_a >= op_b {
                    self.pc = self.pc.wrapping_add(arg.imm);
                }
            }
            Instruction::Lb(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.memory.load_byte(addr);
                self.registers.set(arg.rd, data);
            }
            Instruction::Lh(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.memory.load_halfword(addr);
                self.registers.set(arg.rd, data);
            }
            Instruction::Lw(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.memory.load_word(addr);
                self.registers.set(arg.rd, data);
            }
            Instruction::Lbu(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.memory.load_byte_unsigned(addr);
                self.registers.set(arg.rd, data);
            }
            Instruction::Lhu(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.memory.load_halfword_unsigned(addr);
                self.registers.set(arg.rd, data);
            }
            Instruction::Sb(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.registers.get(arg.rs2);
                self.memory.store_byte(addr, data);
            }
            Instruction::Sh(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.registers.get(arg.rs2);
                self.memory.store_halfword(addr, data);
            }
            Instruction::Sw(arg) => {
                let addr = self.registers.get(arg.rs1);
                let data = self.registers.get(arg.rs2);
                self.memory.store_word(addr, data);
            }
            Instruction::Addi(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = arg.imm;
                self.registers.set(arg.rd, op_a.wrapping_add(op_b));
            }
            Instruction::Slti(arg) => {
                let op_a = self.registers.get(arg.rs1) as i32;
                let op_b = arg.imm as i32;
                if op_a < op_b {
                    self.registers.set(arg.rd, 1);
                } else {
                    self.registers.set(arg.rd, 0);
                }
            }
            Instruction::Sltiu(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = arg.imm;
                if op_a < op_b {
                    self.registers.set(arg.rd, 1);
                } else {
                    self.registers.set(arg.rd, 0);
                }
            }
            Instruction::Xori(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = arg.imm;
                self.registers.set(arg.rd, op_a ^ op_b);
            }
            Instruction::Ori(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = arg.imm;
                self.registers.set(arg.rd, op_a | op_b);
            }
            Instruction::Andi(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = arg.imm;
                self.registers.set(arg.rd, op_a & op_b);
            }
            Instruction::Slli(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let shift = arg.imm & 0x1f;
                self.registers.set(arg.rd, op_a << shift);
            }
            Instruction::Srli(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let shift = arg.imm & 0x1f;
                self.registers.set(arg.rd, op_a >> shift);
            }
            Instruction::Srai(arg) => {
                let op_a = self.registers.get(arg.rs1) as i32;
                let shift = arg.imm & 0x1f;
                self.registers.set(arg.rd, (op_a >> shift) as u32);
            }
            Instruction::Add(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                self.registers.set(arg.rd, op_a.wrapping_add(op_b));
            }
            Instruction::Sub(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                self.registers.set(arg.rd, op_a.wrapping_sub(op_b));
            }
            Instruction::Sll(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let shift = self.registers.get(arg.rs2) & 0x1f;
                self.registers.set(arg.rd, op_a << shift);
            }
            Instruction::Slt(arg) => {
                let op_a = self.registers.get(arg.rs1) as i32;
                let op_b = self.registers.get(arg.rs2) as i32;
                if op_a < op_b {
                    self.registers.set(arg.rd, 1);
                } else {
                    self.registers.set(arg.rd, 0);
                }
            }
            Instruction::Sltu(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                if op_a < op_b {
                    self.registers.set(arg.rd, 1);
                } else {
                    self.registers.set(arg.rd, 0);
                }
            }
            Instruction::Xor(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                self.registers.set(arg.rd, op_a ^ op_b);
            }
            Instruction::Srl(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let shift = self.registers.get(arg.rs2) & 0x1f;
                self.registers.set(arg.rd, op_a >> shift);
            }
            Instruction::Sra(arg) => {
                let op_a = self.registers.get(arg.rs1) as i32;
                let shift = self.registers.get(arg.rs2) & 0x1f;
                self.registers.set(arg.rd, (op_a >> shift) as u32);
            }
            Instruction::Or(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                self.registers.set(arg.rd, op_a | op_b);
            }
            Instruction::And(arg) => {
                let op_a = self.registers.get(arg.rs1);
                let op_b = self.registers.get(arg.rs2);
                self.registers.set(arg.rd, op_a & op_b);
            }
            Instruction::Fence(_arg) => {}
            Instruction::FenceTso => {}
            Instruction::Pause => {}
            Instruction::Ecall => {}
            Instruction::Ebreak => {
                self.halt = true;
            }
            _ => {}
        }
    }

    // wrapper to print the Instruction
    fn disassemble(&self) {
        self.inst.print();
    }

    pub fn print_memory(&self) {
        self.memory.print()
    }

    pub fn print_status(&self) {
        self.registers.print();
    }
}
