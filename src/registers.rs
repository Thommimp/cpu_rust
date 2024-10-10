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


pub struct Registers {
    data: [u32; 32],  // 32 registers, each 32-bit (u32)
}

impl Registers {
    // Creates a new Registers struct with all registers initialized to 0
    pub fn new() -> Self {
        Registers {
            data: [0; 32],  // Initialize all 32 registers to 0
        }
    }

    // Example method to set a register value
    pub fn set(&mut self, index: usize, value: u32) {
        if index < 32 {
            self.data[index] = value;
        }
    }

    // Example method to get a register value
    pub fn get(&self, index: usize) -> u32 {
        if index < 32 {
            self.data[index]
        } else {
            0  // Return 0 if the index is out of bounds (optional behavior)
        }
    }

fn decode(&mut self, input: &u32) {

    let reg1: u32 = (0xF8000 & input) >> 15;
    let reg2: u32 = (0x1F00000 & input) >> 20;
    let rd: u32 = (0xF80 & input) >> 7;
    let funct7: u32 = (0xFE000000 & input) >> 25;
    let funct3: u32 = (0x7000 & input) >> 12;

    let opcode = input & 0x7F;

    self.set(1, 4);
    self.set(2, 16);

    match opcode {
        R_TYPE => { // R-Type opcode
            // Pass registers as a struct to the R_code function

           self.R_code(&reg1, &reg2, &funct3, &funct7, &rd);
        }
        IMM_TYPE => {
            // Handle I-Type instructions
        }
        _ => {} // Handle other opcode cases if needed
    }
}

fn R_code(&mut self, reg1: &u32, reg2: &u32, f3: &u32, f7: &u32, rd: &u32) {
    let r1 = self.get(*reg1 as usize);
    let r2 = self.get(*reg2 as usize);
    let r = *rd as usize;

  match f3 {
            0 => { // funct3 for ADD and SUB
                match f7 {
                    0 => {
                        // ADD function
                        println!("ADD");
                        self.set(r, r1 + r2);

                        //tested virker
                       


                    }
                    32 => {
                        // SUB function
                        println!("SUB");
                        self.set(r, r1 - r2);
                        println!("{:}", self.data[0]);

                        //tested virker


                    }
                    _ => {} // Handle other funct7 cases if needed
                }
            }
            1 => {
                // SLL
                println!("SLL");
                 self.set(r, r1 << r2);
                 println!("{:}", self.data[0]);

                 //tested virker

                
            }
            2 => {
                // SLT
                println!("SLT");
                if (r1 > r2) {
                    self.set(r, 0);
                } else {
                    self.set(r, 1);
                }

              println!("{:}", self.data[0]);

              //tested og virker

            }
            4 => {
                //XOR
                println!("XOR");

            }
            6 => {
                //OR
                println!("OR");
            }
            7 => {
                //AND
                println!("AND");
            }
            5 => {
                match f7 {
                    0 => {
                        //SRL
                        println!("SRL");
                    }
                    32 => {
                        //SRA
                        println!("SRA");
                    }

                    _ => {}

                }
            }

            3 => {
                //SLTU
                println!("SLTU");
            }
            _ => {} // Handle other funct3 cases if needed
        }

}




}

// fn Imm_code(reg1: &u32) {
//         match funct3 {
//             0 => {
//                 //ADDI
//                 println!("ADD IMMEDIATE");
//             }

//             4 => {
//                 //ADDI
//                 println!("XOR IMMEDIATE");
//             }

//             6 => {
//                 //ORI
//                 println!("OR IMMEDIATE");
//             }
//             7 => {
//                 //ANDI
//                 println!("AND IMMEDIATE");
//             }
//             1 => {
//                 //SLLI
//             }

//             5 => {
//                 match funct7 {
//                     0 => {
//                         //SRLI
//                         println!("SRLI");

//                     }
//                     32 => {
//                         //SRAI
//                         println!("SRAI");
//                     }

//                     _ => {}

//                 }
//             }
//             2 => {
//                 //Set less than Imm
//                 println!("SLTI");

//             }
//             3 => {
//                 //set less than imm (U)
//                 println!("set less than imm U");
//             }

//             _ => {} // Handle other func3 cases if needed
//         }
// }