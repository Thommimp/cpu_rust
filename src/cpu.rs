use crate::memory::Memory;  // Import Memory module from the src directory
use crate::registers::Registers;  // Import Registers module from the src directory

const R_TYPE: u32 = 51;
const IMM_TYPE: u32 = 19;
const LOAD_TYPE: u32 = 3;
const STORE_TYPE: u32 = 35;
const BRANCH_TYPE: u32 = 99;
const JAL_TYPE: u32 = 111;
const JALR_TYPE: u32 = 103;
const LUI_TYPE: u32 = 55;
const AUIPC_TYPE: u32 = 23;
const E_TYPE: u32 = 115;

pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    pub fn new(memory_size: usize) -> Self {
        CPU {
            registers: Registers::new(32),
            memory: Memory::new(memory_size),
        }
    }

    pub fn execute_instruction(&mut self, instruction: &u32) {
        let opcode = instruction & 0x7F;
        
        match opcode {
            R_TYPE => {
                // Call the function that handles R-Type instructions
                self.registers.R_code(&instruction);
            }
            IMM_TYPE => {
                self.registers.Imm_code(&instruction);
            }
            LOAD_TYPE => {
                self.load_code(&instruction); // Now this method can access both memory and registers
            }
            _ => {}
        }
    }




    fn load_code(&mut self, instruction: &u32) {
        let rd = ((0xF80 & instruction) >> 7) as usize;
        let funct3: u32 = (0x7000 & instruction) >> 12;
        let reg1: u32 = (0xF8000 & instruction) >> 15;
        let imm: u32 = (0xFFF00000 & instruction) >> 20;

        let r1 = self.registers.get(reg1 as usize);
        let address = (r1 as u32).wrapping_add(imm);

        //println!("{:?}",  address);

        match funct3 {
            0 => {
                let byte = self.memory.read_byte(address);
                self.registers.set(rd, byte as i32);
                println!("LB executed");
            }

            1 => {
                let half_word = self.memory.read_halfword(address);
                self.registers.set(rd, half_word as i32);
                println!("{:b}", half_word);
                println!("LH executed");

            }

            2 => {
                let word = self.memory.read_word(address);
                self.registers.set(rd, word as i32);
                println!("{:b}", word);
            }

            4 => {
                let byte_u = self.memory.read_byte(address);
                self.registers.set(rd, byte_u as i32);
                println!("LBU");

            }

            5 => {
                let half_word_u = self.memory.read_halfword(address);
                self.registers.set(rd, half_word_u as i32);
                println!("LHU");
            }
            _ => {}
        }
    }
}
