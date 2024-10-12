use std::io;


mod cpu;        // Declare the cpu module
mod memory;     // Declare the memory module
mod registers;  // Declare the registers module

use cpu::CPU;



const MEMORY_CAPACITY: usize = 4096;
const XLEN: usize = 32;

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

    let combined = combine_to_u32_array(&cpu.memory.data);

    println!("{:02x}", combined[0]);

  println!("{:?}", instruction_count);





    // cpu.registers.data[1] = 400;

     // Loop through the instructions, ensuring we don't exceed instruction_count
    for i in 0..instruction_count {
        //println!("Executing instruction: {:b}, PC: {}", combined[i], cpu.pc);
        cpu.execute_instruction(&combined[i]);
        cpu.pc += 4; // Increment PC by 4 for the next instruction
    }

    for reg_num in 0..32 {
    let reg_value = cpu.registers.get(reg_num);  // Assuming `cpu.registers.get()` returns the value in the register
    println!("x{}: 0x{:08x} ({})", reg_num, reg_value, reg_value); // Print in both hex and decimal
}



    //println!("{:?}", cpu.memory.read_word(504));

    //println!("{:?} register", cpu.registers.data[0]);




  


    // }

    // let combined = combine_to_u32_array(&memory.data);

    // registers.decode(&combined[0]);

    //println!("{:0x}", combined[0]);





    Ok(())
}
