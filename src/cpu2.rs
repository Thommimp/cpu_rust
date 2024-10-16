use std::string::String;

use crate::pipeline::Pipeline;
use crate::memory::Memory;
use crate::isa::*;

pub struct Cpu {
    clock: usize,
    x: [u32; 32],
    pc: u32,
    pipeline: Pipeline,
    memory: Memory,
    halt: bool,
}

impl Cpu {
    pub fn new(memory_capacity: usize) -> Self {
        Cpu {
            clock: 0,
            x: [0; 32],
            pc: 0,
            pipeline: Pipeline::new(),
            memory: Memory::new(memory_capacity),
            halt: false,
        }
    }
    pub fn load_from_file(&mut self, path: &str) -> Result<usize, String> {
        self.memory.load_from_file(path)
    }

    fn read_register(&mut self, reg: usize) -> u32 {
        self.x[reg]
    }
    fn write_register(&mut self, value: u32, reg: usize) {
        self.x[reg] = value
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.write_back();
        self.memory_access();
        self.execute();
        let _ = self.decode();
        self.fetch();
        self.pipeline.tick();
        Ok(())
    }
    fn fetch(&mut self) {
        self.pc += 4;
        let i_word: u32 = self.memory.read_word(self.pc);
        self.pipeline.update_ifid(self.pc, i_word);
    }
    fn decode(&mut self) -> Result<(), String> {
        let i_word = self.pipeline.get_inst_word();
        let inst = {
            match phrase_instruction(i_word) {
                Ok(inst) => inst,
                Err(e) => return Err(e),
            }
        };
        let arg = (inst.decode)(i_word);
        let reg1 = self.read_register(arg.rs1);
        let reg2 = self.read_register(arg.rs2);
        self.pipeline
            .update_idex(Some(inst), reg1, reg2, arg.imm, arg.rd);
        Ok(())
    }
    fn execute(&mut self) {
        let inst = match self.pipeline.get_execute_instruction() {
            Some(i) => i,
            None => return,
        };
        let (reg1, reg2, imm) = self.pipeline.get_execute_arguments();

        let op_1 = reg1;
        let op_2 = match inst.ex_src {
            ExSrc::Reg2 => reg2,
            ExSrc::Imm => imm,
            _ => 0,
        };
        let result = (inst.execute)(op_1, op_2);
        self.pipeline.update_exmem(result)
    }
    fn memory_access(&mut self) {
        let inst = match self.pipeline.get_memory_instruction() {
            Some(i) => i,
            None => return,
        };
        let (address, data) = self.pipeline.get_memory_arg();
        let mut memory: u32 = 0;
        match inst.mem_rw {
            MemRW::Read => {
                memory = match inst.name {
                    "lb" => self.memory.load_byte(address),
                    "lh" => self.memory.load_halfword(address),
                    "lw" => self.memory.load_word(address),
                    "lbu" => self.memory.load_byte_unsigned(address),
                    "lhu" => self.memory.load_halfword_unsigned(address),
                    _ => 0,
                };
            }
            MemRW::Write => match inst.name {
                "sb" => self.memory.store_byte(address, data),
                "sh" => self.memory.store_halfword(address, data),
                "sw" => self.memory.store_word(address, data),
                _ => (),
            },
            _ => memory = 0,
        };
        self.pipeline.update_memwb(memory);
    }
    fn write_back(&mut self) {
        let inst = match self.pipeline.get_writeback_instruction() {
            Some(i) => i,
            None => return,
        };
        let (result, memory, rd) = self.pipeline.get_writeback_arg();
        match inst.wb_src {
            WBSrc::Result => self.x[rd] = result,
            WBSrc::Memory => self.x[rd] = memory,
            _ => (),
        };
    }
    fn print_status(&self) {
        println!("|-----------|");
        for i in 31..0 {
            println!("| R{:02}: {:5}|", i, self.x[i]);
        }
        println!("| PC: {:6}|", self.pc);
    }
}

pub fn get_register_alias(register: usize) -> &'static str {
    match register {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0",
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        _ => ""
    }
}

