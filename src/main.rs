use std::io;

mod cpu;
mod memory;
mod registers;
mod cpu2;
mod isa;
mod pipeline;

use cpu::CPU;

const MEMORY_CAPACITY: usize = 4096;

fn combine_to_u32_array(array: &[u8]) -> Vec<u32> {
    let mut combined = Vec::new();
    for chunk in array.chunks(4) {
        let mut value: u32 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            value |= (byte as u32) << (8 * (3 - i));
        }
        combined.push(value);
    }
    combined
}

fn main() -> io::Result<()> {

    let mut cpu = CPU::new(MEMORY_CAPACITY);
        let instruction_count: usize;



     match cpu.memory.load_from_file("example.bin") {
        Ok(i) => {
            instruction_count = i;
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            instruction_count = 0;
        }
    }

    // let combined = combine_to_u32_array(&cpu.memory.data);

    //println!("{:?}", combined[0]);



    //cpu.memory.write_byte(400, -2);
     cpu.memory.write_byte(401, 1);
      cpu.memory.write_byte(403, 1);



    cpu.registers.data[1] = 400;

    //cpu.execute_instruction(&combined[0]);

    //println!("{:?}", cpu.memory.read_word(504));

    //println!("{:?} register", cpu.registers.data[0]);




  


    // }

    // let combined = combine_to_u32_array(&memory.data);

    // registers.decode(&combined[0]);

    //println!("{:0x}", combined[0]);





    Ok(())
}
