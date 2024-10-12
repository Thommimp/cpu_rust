



pub struct Registers {
    pub data: [i32; 32], // 32 registers, each 32-bit (u32)
}

impl Registers {
    // Creates a new Registers struct with all registers initialized to 0
    pub fn new(capacity: usize) -> Self {
        Registers {
            data: [0; 32], // Initialize all 32 registers to 0
        }
    }

    // Example method to set a register value
    pub fn set(&mut self, index: usize, value: i32) {
        if index < 32 {
            self.data[index] = value;
        }
    }

    // Example method to get a register value
    pub fn get(&self, index: usize) -> i32 {
        if index < 32 {
            self.data[index]
        } else {
            0 // Return 0 if the index is out of bounds (optional behavior)
        }
    }


    pub fn R_code(&mut self, instruction: &u32) {

        let reg1: u32 = (0xF8000 & instruction) >> 15;
        let reg2: u32 = (0x1F00000 & instruction) >> 20;
        let rd = ((0xF80 & instruction) >> 7) as usize;
        let function7: u32 = (0xFE000000 & instruction) >> 25;
        let function3: u32 = (0x7000 & instruction) >> 12;

        let r1 = self.get(reg1 as usize);
        let r2 = self.get(reg2 as usize);

        match function3 {
            0 => {
                // funct3 for ADD and SUB
                match function7 {
                    0 => {
                        // ADD function
                        println!("ADD");
                        self.set(rd, r1 + r2);

                        //tested virker
                    }
                    32 => {
                        // SUB function
                        println!("SUB");
                        self.set(rd, r1 - r2);
                        println!("{:}", self.data[0]);

                        //tested virker
                    }
                    _ => {} // Handle other funct7 cases if needed
                }
            }
            1 => {
                // SLL
                println!("SLL");
                self.set(rd, r1 << r2);
                println!("{:}", self.data[0]);

                //tested virker
            }
            2 => {
                // SLT
                println!("SLT");
                if (r1 > r2) {
                    self.set(rd, 0);
                } else {
                    self.set(rd, 1);
                }

                println!("{:}", self.data[0]);
                //tested og virker
            }
            4 => {
                //XOR
                println!("XOR");
                self.set(rd, r1 ^ r2);
            }
            6 => {
                //OR
                println!("OR");
                self.set(rd, r1 | r2);
            }
            7 => {
                //AND
                println!("AND");
                self.set(rd, r1 & r2);
            }
            5 => {
                match function7 {
                    0 => {
                        //SRL
                        println!("SRL");
                        self.set(rd, ((r1 as u32) >> r2) as i32);
                    }
                    32 => {
                        //SRA
                        println!("SRA");
                        self.set(rd, r1 >> r2);
                    }

                    _ => {}
                }
            }

            3 => {
                //SLTU
                println!("SLTU");
                if ((r1 as u32) < (r2 as u32)) {
                    // Check if r1 is less than r2 as unsigned
                    self.set(rd, 1); // Set the result register to 1
                } else {
                    self.set(rd, 0); // Set the result register to 0
                }
                //tested og virker
            }
            _ => {
                // Handle other funct3 cases if needed
                println!("AHHHHHH");
            } 

        }
    }


    pub fn Imm_code(&mut self, instruction: &u32) {

        let rd = ((0xF80 & instruction) >> 7) as usize;
        //let rd = rd_u as usize;
        let function3: u32 = (0x7000 & instruction) >> 12;

        let reg1: u32 = (0xF8000 & instruction) >> 15;
let imm_unsigned: u32 = (0xFFF00000 & instruction) >> 20; // Extract the immediate

// Sign extend the immediate using a simple cast
let imm: i32 = (imm_unsigned as i32) << 20 >> 20; // Shift left and then right to sign-extend

let r1 = self.get(reg1 as usize);

        println!("{:}", imm);



        match function3 {
            0 => {
                //ADDI
                println!("ADD IMMEDIATE");
                self.set(rd, r1 + imm);
                println!("{:?}", self.data[rd]);
            }

            4 => {
                //ADDI
                println!("XOR IMMEDIATE");
                self.set(rd, r1 ^ imm);


            }

            6 => {
                //ORI
                println!("OR IMMEDIATE");
                self.set(rd, r1 | imm);
            }
            7 => {
                //ANDI
                println!("AND IMMEDIATE");
                self.set(rd, r1 & imm);

            }
            1 => {
                //SLLI
                println!("SLLI");
                self.set(rd, r1 << (imm as u32));
            }

            5 => {
                let funct7 = (imm >> 5) & 0x7F; // Extract the upper 7 bits of the immediate (imm[11:5])

               
                     if funct7 == 0x00 {
            // SRLI - Logical Shift Right
            println!("SRLI");
            // Perform logical right shift (zero-fill)
            self.set(rd, ((r1 as u32) >> (imm & 0x1F)) as i32); // Use only the lower 5 bits of imm for shift amount
            println!("{:?}", self.data[rd]);
        } else if funct7 == 0x20 {
            // SRAI - Arithmetic Shift Right
            println!("SRAI");
            // Perform arithmetic right shift (sign-extend)
            self.set(rd, r1 >> (imm & 0x1F)); // Use only the lower 5 bits of imm for shift amount
            println!("{:?}", self.data[rd]);

            }
        }
            2 => {
                //Set less than Imm
                println!("SLTI");
                if r1 < imm {
                    self.set(rd, 1);

                } else {
                    self.set(rd, 0);
                }

            }
            3 => {
                //SLTIU
                println!("set less than imm U");

                if (r1 as u32) < (imm as u32) {
                    self.set(rd, 1);

                } else {
                    self.set(rd, 0);

                }
            }

            _ => {
                println!("noget gik galt, I-type");
            } // Handle other func3 cases if needed
        }
}

    }