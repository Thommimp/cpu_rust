use std::string::String;

use crate::pipeline::Pipeline;
use crate::isa::
use crate::memory::Memory;

pub struct Cpu {
    clock: usize,
    x: [u32; 32],
    pc: u32,
    pipeline: Pipeline,
    memory: Memory,
    halt: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            clock: 0,
            x: [0; 32],
            pc: 0,
            pipeline: Pipeline::new(),
            memory: Memory::new(4096),
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
        self.decode();
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
            match decode_instruction(i_word) {
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
                    "lb" => self.memory.read_byte(address) as u32,
                    "lh" => self.memory.read_halfword(address) as u32,
                    "lw" => self.memory.read_word(address) as u32,
                    "lbu" => self.memory.read_byte(address) as u32,
                    "lhu" => self.memory.read_halfword(address) as u32,
                    _ => 0,
                };
            }
            MemRW::Write => match inst.name {
                "sb" => self.memory.write_byte(address, data as u8),
                "sh" => self.memory.write_halfword(address, data as u16),
                "sw" => self.memory.write_word(address, data),
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
}
