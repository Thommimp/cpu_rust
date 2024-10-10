use std::io;
use memory::Memory;

mod memory;
mod cpu;


const MEMORY_CAPACITY: usize = 4096;
const XLEN: usize = 32;



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
