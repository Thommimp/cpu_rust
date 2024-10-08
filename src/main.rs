use std::io;
use memory::Memory;

mod memory;
mod cpu;


const MEMORY_CAPACITY: usize = 4096;
const XLEN: usize = 32;

fn decode(input: u32, registers: &mut [u32; 32]) {

    registers[1] = 10;

    let reg1: u32 = (0xF8000 & input) >> 15;
    let reg2: u32 = (0x1F00000 & input) >> 20;
    let rd: u32 = (0xF80 & input) >> 7;
    let funct7 = (0xFE000000 & input) >> 25;
    let funct3 = (0x7000 & input) >> 12;

    let opcode = input & 0x7F;
    //println!("{:b}", reg1);
    //println!("{:b}", input);
    println!("{:b}", funct3);
    //println!("{:b}", funct7);

    

match opcode {
    51 => { // R-Type opcode
        match funct3 {
            0 => { // funct3 for ADD and SUB
                match funct7 {
                    0 => {
                        // ADD function
                        println!("ADD");
                        registers[rd as usize] = registers[reg1 as usize] + registers[reg2 as usize];
                    }
                    32 => {
                        // SUB function
                        println!("SUB");
                        registers[rd as usize] = registers[reg1 as usize] - registers[reg2 as usize];
                    }
                    _ => {} // Handle other funct7 cases if needed
                }
            }
            1 => {
                // SLL
                println!("SLL");
                
            }
            2 => {
                // SLT
                println!("SLT");
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
                match funct7 {
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
    19 => { //I TYPE INSTRUCTIONS
        match funct3 {
            0 => {
                //ADDI
                println!("ADD IMMEDIATE");
            }

            4 => {
                //ADDI
                println!("XOR IMMEDIATE");
            }

            6 => {
                //ORI
                println!("OR IMMEDIATE");
            }
            7 => {
                //ANDI
                println!("AND IMMEDIATE");
            }
            1 => {
                //SLLI
            }

            5 => {
                match funct7 {
                    0 => {
                        //SRLI
                        println!("SRLI");

                    }
                    32 => {
                        //SRAI
                        println!("SRAI");
                    }

                    _ => {}

                }
            }
            2 => {
                //Set less than Imm
                println!("SLTI");

            }
            3 => {
                //set less than imm (U)
                println!("set less than imm U");
            }

            _ => {} // Handle other func3 cases if needed

        }

    }
    _ => {} // Handle other opcode cases if needed
}

}

fn pc_func(pc: &mut u32, ting: &u32, _offset: &mut u32, _branch: &mut u32) {
    match ting {
        1 => *pc = *pc + 1,
        _ => println!("ERROR SOMETHING WENT VERY WRONG"),
    }
}

fn main() -> io::Result<()> {
    let mut registers: [u32; 32] = [0; 32];
    let mut pc_counter: u32 = 0;
    let mut branch: u32 = 0;
    let mut offset: u32 = 0; 
    let instruction_count: usize;

    let mut memory = Memory::new(MEMORY_CAPACITY);

    match memory.load_from_file("example.bin") {
        Ok(i) => {
            instruction_count = i;
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            instruction_count = 0;
        }
    }
    println!("Number of instructions {}", instruction_count);
    println!("{:08X}", memory.read_word(0));

    decode(memory.read_word(0), &mut registers);
    pc_func(&mut pc_counter, &1, &mut branch, &mut offset);

    Ok(())
}
