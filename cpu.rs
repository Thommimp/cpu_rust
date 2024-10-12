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
    pub pc: u32,
}

impl CPU {
    pub fn new(memory_size: usize) -> Self {
        CPU {
            registers: Registers::new(32),
            memory: Memory::new(memory_size),
            pc: 0,

        }
    }

    pub fn execute_instruction(&mut self, instruction: &u32) {
        let opcode = instruction & 0x7F;
        println!("{:b}", instruction);
        
        match opcode {
            R_TYPE => {
                // Call the function that handles R-Type instructions
                self.registers.R_code(&instruction);
            }
            IMM_TYPE => {
                self.registers.Imm_code(&instruction);
            }
            LOAD_TYPE => {
                self.load_code(&instruction, opcode); // Now this method can access both memory and registers
            }

            STORE_TYPE => {
                self.store_code(&instruction);
            }
            BRANCH_TYPE => {
                self.branch_code(&instruction);
            }
            JAL_TYPE => {
                self.jal_code(&instruction);

            }
            JALR_TYPE => {
                self.load_code(&instruction, opcode);
            }
            LUI_TYPE => {
                self.decode_u_type(&instruction, opcode);
            }
            AUIPC_TYPE => {
                self.decode_u_type(&instruction, opcode);
            }
            _ => {}
        }
    }




    fn load_code(&mut self, instruction: &u32, p_type: u32) {
        let rd = ((0xF80 & instruction) >> 7) as usize;
        let funct3: u32 = (0x7000 & instruction) >> 12;
        let reg1: u32 = (0xF8000 & instruction) >> 15;
        let imm: u32 = (0xFFF00000 & instruction) >> 20;

        let imm_signed: i32 = (imm as i32) << 12 >> 12;     // Sign extend the immediate


        let r1 = self.registers.get(reg1 as usize);
        let address = (r1 as u32).wrapping_add(imm);

        //println!("{:?}",  address);

        match p_type {
            IMM_TYPE => {

        match funct3 {
            0 => {
                let byte = self.memory.read_signed_byte(address);
                self.registers.set(rd, byte as i32);
                println!("LB executed");
            }

            1 => {
                let half_word = self.memory.read_signed_halfword(address);
                self.registers.set(rd, half_word as i32);
                println!("{:b}", half_word);
                println!("LH executed");

            }

            2 => {
                let word = self.memory.read_signed_word(address);
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
     JALR_TYPE => {
         self.registers.set(rd, (self.pc + 4) as i32);
         self.pc = ((r1  + imm_signed) as u32);
         println!("jalr");

    }
    _=> {}
    }
}

    fn store_code(&mut self, instruction: &u32) {
        let imm4_0 = ((0xF80 & instruction) >> 7);
        let funct3: u32 = (0x7000 & instruction) >> 12;
        let reg1: u32 = (0xF8000 & instruction) >> 15;
        let reg2: u32 = (0x1F00000 & instruction) >> 20;
        let imm11_5: u32 = (0xFE000000 & instruction) >> 25;
        let imm = (imm11_5 << 5) | imm4_0;


        let r1 = self.registers.get(reg1 as usize) as u32;


        match funct3 {
            0 => {
                //println!("{:?}", r1);
                let r2 = self.registers.get(reg2 as usize) as i8;
                //println!("{:?}", r2);
                self.memory.write_signed_byte(r1 + imm, r2);
                println!("SB");



            }
            1 => {
                let r2 = self.registers.get(reg2 as usize) as i16;
                self.memory.write_signed_halfword(r1 + imm, r2);
                println!("SH");


            }
            2 => {
                let r2 = self.registers.get(reg2 as usize) as i32;
                self.memory.write_signed_word(r1 + imm, r2);
                println!("SW");

            }
            _ => {}
        }


    }

    fn branch_code(&mut self, instruction: &u32) {

    let funct3: u32 = (instruction & 0x7000) >> 12;     // funct3 (bits 14:12)
    let reg1: u32 = (instruction & 0xF8000) >> 15;      // rs1 (bits 19:15)
    let reg2: u32 = (instruction & 0x1F00000) >> 20;    // rs2 (bits 24:20)



    let imm11 = (instruction & 0x80) >> 7;              // imm[11] (bit 7)
    let imm4_1 = (instruction & 0xF00) >> 8;            // imm[4:1] (bits 11:8)
    let imm10_5 = (instruction & 0x7E000000) >> 25;     // imm[10:5] (bits 30:25)
    let imm12 = (instruction & 0x80000000) >> 31;       // imm[12] (bit 31)

    // Combine immediate fields into a 13-bit immediate (signed) for branch offset
    let imm13 = (imm12 << 12)     // imm[12] goes to bit 12
               | (imm11 << 11)    // imm[11] goes to bit 11
               | (imm10_5 << 5)   // imm[10:5] go to bits 10:5
               | (imm4_1 << 1);   // imm[4:1] go to bits 4:1

    // Sign extend imm13 to a 32-bit signed integer if needed
    let imm_signed = ((imm13 as i32) << 19) >> 19;

    let r1 = self.registers.get(reg1 as usize);
    let r2 = self.registers.get(reg2 as usize);

    println!("{:?}", imm13);




    match funct3 {
        0 => {
            if r1 == r2 {
                self.pc = (((self.pc as i32) + imm_signed) as u32);

            }
            println!("BEQ");



        }
        1 => {
            if r1 != r2 {
             self.pc = (((self.pc as i32) + imm_signed) as u32);


            }
            println!("BNE");

        }

        4 => {
              if r1 < r2 {
             self.pc = (((self.pc as i32) + imm_signed) as u32);


            }
            println!("BLT");


        }

        5 => {
             if r1 >= r2 {
             self.pc = (((self.pc as i32) + imm_signed) as u32);


            }
            println!("BGE");

        }

         6 => {
             if (r1 as u32) < (r2 as u32) {
             self.pc = (((self.pc as i32) + imm_signed) as u32);


            }
            println!("BLTU");

        }

        7 => {
              if (r1 as u32) >= (r2 as u32) {
             self.pc = (((self.pc as i32) + imm_signed) as u32);


            }
            println!("BGEU");

        }




        _ => {}
    }





    }

    fn jal_code(&mut self, instruction: &u32) {
            let imm20 = (instruction & 0x80000000) >> 31;       // imm[20] (bit 31)
    let imm10_1 = (instruction & 0x7FE00000) >> 21;     // imm[10:1] (bits 30:21)
    let imm11 = (instruction & 0x100000) >> 20;         // imm[11] (bit 20)
    let imm19_12 = (instruction & 0xFF000);             // imm[19:12] (bits 19:12)

    // Combine immediate fields into a 21-bit immediate (signed) for jump offset
    let imm21 = (imm20 << 20)     // imm[20] goes to bit 20
               | (imm19_12)       // imm[19:12] go to bits 19:12
               | (imm11 << 11)    // imm[11] goes to bit 11
               | (imm10_1 << 1);  // imm[10:1] go to bits 10:1

    // Sign extend imm21 to a 32-bit signed integer
    let imm_signed = ((imm21 as i32) << 11) >> 11;

    let rd = ((0xF80 & instruction) >> 7) as usize;

    self.registers.set(rd, (self.pc + 4) as i32);
    self.pc = (((self.pc as i32) + imm_signed) as u32);

    println!("JAL");






    }

    fn decode_u_type(&mut self, instruction: &u32, opcode: u32) {
    let rd = ((instruction & 0xF80) >> 7) as usize;       // Destination register (bits 11:7)
    let imm: u32 = (instruction & 0xFFFFF000) >> 12; // Immediate (upper 20 bits)

    match opcode {
        LUI_TYPE => {
            self.registers.set(rd, (imm << 12) as i32);
            println!("LUI");
            println!("{:b}", (imm <<12));

        }
        AUIPC_TYPE => {
            self.registers.set(rd, (self.pc + self.pc + (imm << 12)) as i32);
            println!("AUIPC");

        }
        _ => {}
    }
}
}


