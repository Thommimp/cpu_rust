use std::fs::File;
use std::io::{self, Read, BufReader};

use memory::Memory;

const MEMORY_CAPACITY: usize = 64*1024;

mod memory;

fn read_binary(file_path: &str, output: &mut [u32; 100]) -> io::Result<usize> {
    // Specify the path to the text file
    let path = std::path::Path::new(file_path);

    // Open the file
    let file = File::open(&path)?;
    
    // Create a buffered reader
    let mut reader = BufReader::new(file);

    // Read the entire file into a string
    let mut binary_string = String::new();
    reader.read_to_string(&mut binary_string)?;

    // Remove any whitespace (if there happens to be any)
    let binary_string = binary_string.trim();

    // Check if the length is a multiple of 32
    let length = binary_string.len();
    println!("{:?}", length);
    if length % 32 != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Binary data is not a multiple of 32 bits."));
    }

    // Initialize a counter for the number of instructions read
    let mut count = 0;

    // Read 32 bits at a time
    for chunk in binary_string.as_bytes().chunks(32) {
        if count >= output.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Memory array is full."));
        }

        // Convert the 32-bit binary chunk to a string
        let binary_str = std::str::from_utf8(chunk).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

        // Convert the binary string to an unsigned 32-bit integer
        output[count] = u32::from_str_radix(binary_str, 2)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        count += 1; // Increment the count
    }

    Ok(count) // Return the number of instructions read
}

fn decode(input: &u32, registers: &mut [u32; 32]) {

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
    // Variable to hold up to 100 unsigned integers
    let mut memory: [u32; 100] = [0; 100];
    let mut registers: [u32; 32] = [0; 32];
    let mut pc_counter: u32 = 0;
    let mut branch: u32 = 0;
    let mut offset: u32 = 0; 
    let mut count2: usize = 0;  

    let mut memory_struct = Memory::new(MEMORY_CAPACITY);

    memory_struct.load_from_file("example.bin");
    println!("{:X}", memory_struct.read_word(0));

    // Call the read_binary function
    match read_binary("binary.txt", &mut memory) {
        Ok(count) => {
            count2 = count;
            println!("Read {} instructions from the binary file:", count);
            for i in 0..count {
                println!("Memory[{}]: {}", i, memory[i]);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }


     decode(&memory[0], &mut registers);

     pc_func(&mut pc_counter, &1, &mut branch, &mut offset);
     println!("{:?}", count2);



    Ok(())
}
